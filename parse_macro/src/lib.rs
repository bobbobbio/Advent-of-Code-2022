extern crate proc_macro;

use heck::ToSnakeCase as _;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::collections::BTreeMap;
use std::matches;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
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
        get_container_separator_parser_from_attrs(attrs)?.unwrap_or(parse_quote!(char(' ')));

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

struct ParseAttrs<Kind> {
    _parens: token::Paren,
    attrs: Punctuated<ParseAttr<Kind>, Token![,]>,
}

impl<Kind: AttrKeywordKind> Parse for ParseAttrs<Kind> {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            _parens: parenthesized!(content in input),
            attrs: content.parse_terminated(ParseAttr::parse)?,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
enum VariantKeyword {
    String,
}

impl AttrKeywordKind for VariantKeyword {}

impl TryFrom<Ident> for VariantKeyword {
    type Error = Error;

    fn try_from(id: Ident) -> Result<Self> {
        Ok(match &id.to_string()[..] {
            "string" => Self::String,
            _ => return Err(Error::new(id.span(), "unknown keyword")),
        })
    }
}

trait AttrKeywordKind: TryFrom<Ident, Error = Error> + PartialOrd + Ord {}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
enum ContainerKeyword {
    SepBy,
}

impl AttrKeywordKind for ContainerKeyword {}

impl TryFrom<Ident> for ContainerKeyword {
    type Error = Error;

    fn try_from(id: Ident) -> Result<Self> {
        Ok(match &id.to_string()[..] {
            "sep_by" => Self::SepBy,
            _ => return Err(Error::new(id.span(), "unknown keyword")),
        })
    }
}

#[derive(Debug, Clone)]
struct AttrKeyword<Kind> {
    kind: Kind,
    span: Span,
}

impl<Kind> Spanned for AttrKeyword<Kind> {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl<Kind: AttrKeywordKind> Parse for AttrKeyword<Kind> {
    fn parse(input: ParseStream) -> Result<Self> {
        let id: Ident = input.parse()?;
        Ok(Self {
            span: id.span(),
            kind: id.try_into()?,
        })
    }
}

struct ParseAttr<Kind> {
    kw: AttrKeyword<Kind>,
    _equal_token: Token![=],
    value: LitStr,
}

impl<Kind: AttrKeywordKind> Parse for ParseAttr<Kind> {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            kw: input.parse()?,
            _equal_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

fn parse_attr_map<Kind: AttrKeywordKind>(
    attrs: Vec<syn::Attribute>,
) -> Result<BTreeMap<Kind, LitStr>> {
    let parsed_attrs: Vec<ParseAttrs<Kind>> = attrs
        .into_iter()
        .filter(|a| a.path.get_ident() == Some(&Ident::new("parse", Span::call_site())))
        .map(|a| parse2(a.tokens))
        .collect::<Result<_>>()?;
    let attrs: Vec<_> = parsed_attrs
        .into_iter()
        .map(|a| a.attrs.into_iter())
        .flatten()
        .collect();

    let mut attr_map = BTreeMap::new();
    for attr in attrs {
        if attr_map.contains_key(&attr.kw.kind) {
            return Err(Error::new(attr.kw.span(), "Duplicate attribute"));
        }
        attr_map.insert(attr.kw.kind, attr.value);
    }
    Ok(attr_map)
}

fn get_variant_parser_from_attrs(attrs: Vec<syn::Attribute>) -> Result<Option<Expr>> {
    let attr_map = parse_attr_map::<VariantKeyword>(attrs)?;

    if let Some(value) = attr_map.get(&VariantKeyword::String) {
        Ok(Some(parse_quote!(string(#value))))
    } else {
        Ok(None)
    }
}

fn get_container_separator_parser_from_attrs(attrs: Vec<syn::Attribute>) -> Result<Option<Expr>> {
    let attr_map = parse_attr_map::<ContainerKeyword>(attrs)?;

    if let Some(value) = attr_map.get(&ContainerKeyword::SepBy) {
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
                let parser =
                    get_variant_parser_from_attrs(v.attrs)?.unwrap_or(name_parser(variant_name));
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
