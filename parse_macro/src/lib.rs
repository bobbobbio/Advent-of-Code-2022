extern crate proc_macro;

use heck::ToSnakeCase as _;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::matches;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned as _;
use syn::*;

fn verify_signature(sig: &Signature) -> Result<()> {
    let as_expected = matches!(sig, Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        generics: Generics {
            lt_token: None,
            gt_token: None,
            where_clause: None,
            ..
        },
        inputs,
        variadic: None,
        output: ReturnType::Type(_, ret_type),
        ..
    } if inputs.is_empty() && matches!(&**ret_type, Type::Infer(_)));

    if !as_expected {
        Err(Error::new(
            sig.ident.span(),
            "function signature wrong for into_parser",
        ))
    } else {
        Ok(())
    }
}

fn into_parser_inner(input: TokenStream) -> Result<TokenStream> {
    let input: ItemFn = parse(input)?;
    verify_signature(&input.sig)?;

    let name = input.sig.ident;
    let block = input.block;
    Ok(quote! {
        type Parser<Input: combine::Stream<Token = char>> = impl Parser<Input, Output = Self>;

        fn #name<Input>() -> Self::Parser<Input>
        where
            Input: ::combine::Stream<Token = char>,
        #block
    }
    .into())
}

#[proc_macro_attribute]
pub fn into_parser(_attr: TokenStream, input: TokenStream) -> TokenStream {
    match into_parser_inner(input) {
        Ok(v) => v,
        Err(e) => e.into_compile_error().into(),
    }
}

fn derive_has_parser_struct(
    name: Ident,
    attrs: Vec<Attribute>,
    data: DataStruct,
) -> Result<ItemImpl> {
    let fields: Vec<&Field> = data.fields.iter().collect();
    let mut patterns: Vec<Pat> = vec![];
    let mut field_names: Vec<Ident> = vec![];
    let mut parsers: Vec<Expr> = vec![];
    let mut fields_iter = fields.iter().peekable();
    let mut unique = (1..).map(|n| Ident::new(&format!("f{n}"), Span::call_site()));

    let sep_parser: Expr =
        get_parser_from_attrs(attrs, "sep_by", name.span())?.unwrap_or(parse_quote!(char(' ')));

    while let Some(f) = fields_iter.next() {
        let ty = &f.ty;

        let parser_expr: Expr = parse_quote!(<#ty as ::parse::HasParser>::parser());
        if fields_iter.peek().is_some() {
            parsers.push(parse_quote!(#parser_expr.skip(#sep_parser)));
        } else {
            parsers.push(parser_expr);
        }

        if let Some(field_name) = f.ident.clone() {
            patterns.push(parse_quote!(#field_name));
            field_names.push(field_name);
        } else {
            let ident = unique.next().unwrap();
            patterns.push(parse_quote!(#ident));
        }
    }

    let map_closure: Expr = if field_names.is_empty() {
        if patterns.len() == 1 {
            parse_quote!(Self)
        } else {
            parse_quote!(|(#(#patterns),*)| Self(#(#patterns),*))
        }
    } else {
        parse_quote!(|(#(#patterns),*)| Self { #(#field_names),* })
    };

    Ok(parse_quote! {
        impl ::parse::HasParser for #name {
            #[into_parser]
            fn parser() -> _ {
                (#(#parsers),*).map(#map_closure)
            }
        }
    })
}

struct ParseAttrs {
    _parens: token::Paren,
    attrs: Punctuated<ParseAttr, Token![,]>,
}

impl Parse for ParseAttrs {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            _parens: parenthesized!(content in input),
            attrs: content.parse_terminated(ParseAttr::parse)?,
        })
    }
}

struct ParseAttr {
    kw: Ident,
    _equal_token: Token![=],
    value: LitStr,
}

impl Parse for ParseAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            kw: input.parse()?,
            _equal_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

fn get_parser_from_attrs(
    attrs: Vec<syn::Attribute>,
    expected_keyword: &str,
    span: Span,
) -> Result<Option<Expr>> {
    let parse_attrs: Vec<_> = attrs
        .iter()
        .filter(|a| a.path.get_ident() == Some(&Ident::new("parse", Span::call_site())))
        .collect();
    if parse_attrs.len() > 1 {
        return Err(Error::new(span, "too many attributes"));
    }

    if parse_attrs.is_empty() {
        return Ok(None);
    }

    let a: Attribute = (*parse_attrs.first().unwrap()).clone();

    let inner_attrs: ParseAttrs = parse2(a.tokens)?;
    if inner_attrs.attrs.len() > 1 {
        return Err(Error::new(span, "too many attributes"));
    }

    if let Some(a) = inner_attrs.attrs.first() {
        let keyword = &a.kw;
        if keyword != &Ident::new(expected_keyword, Span::call_site()) {
            return Err(Error::new(span, "unknown keyword {keyword:?}"));
        }
        let value = &a.value;
        Ok(Some(parse_quote!(string(#value))))
    } else {
        Ok(None)
    }
}

fn name_parser(name: &Ident) -> Expr {
    let name = name.to_string().to_snake_case();
    parse_quote!(string(#name))
}

fn derive_has_parser_enum(name: Ident, data: DataEnum) -> Result<ItemImpl> {
    let mut parsers: Vec<Expr> = vec![];
    for v in data.variants {
        let variant_name = &v.ident;
        let variant_span = v.span();
        match v.fields {
            Fields::Unit => {
                let parser = get_parser_from_attrs(v.attrs, "string", variant_span)?
                    .unwrap_or(name_parser(variant_name));
                parsers.push(parse_quote!(attempt(#parser.map(|_| Self::#variant_name))));
            }
            Fields::Unnamed(f) => {
                if f.unnamed.len() != 1 {
                    return Err(Error::new(
                        variant_span,
                        "unnamed enum fields must have exactly one field",
                    ));
                }
                let f = f.unnamed.first().unwrap();
                let ty = &f.ty;
                let parser: Expr = parse_quote!(<#ty as ::parse::HasParser>::parser());
                parsers.push(parse_quote!(attempt(#parser.map(Self::#variant_name))));
            }
            Fields::Named(_) => {
                return Err(Error::new(variant_span, "named enum fields not supported"));
            }
        };
    }
    Ok(parse_quote! {
        impl ::parse::HasParser for #name {
            #[into_parser]
            fn parser() -> _ {
                choice((#(#parsers),*))
            }
        }
    })
}

fn derive_has_parser_inner(input: DeriveInput) -> Result<ItemImpl> {
    match input.data {
        Data::Struct(ds) => derive_has_parser_struct(input.ident, input.attrs, ds),
        Data::Enum(de) => derive_has_parser_enum(input.ident, de),
        _ => Err(Error::new(Span::call_site(), "Unsupported type")),
    }
}

#[proc_macro_derive(HasParser, attributes(parse))]
pub fn derive_has_parser(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match derive_has_parser_inner(input) {
        Ok(v) => quote!(#v).into(),
        Err(e) => e.into_compile_error().into(),
    }
}
