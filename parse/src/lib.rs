#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use combine::eof;
use combine::parser::char::{alpha_num, spaces};
use combine::stream::{easy, position};
use prelude::*;
use std::convert::Infallible;
use std::marker::PhantomData;
use std::{
    io, iter, num,
    ops::{Deref, DerefMut},
    slice, str, vec,
};

pub mod prelude {
    pub use super::*;
    pub use combine::parser::char::*;
    pub use combine::*;
    pub use combine::{Parser, Stream};
    pub use parse_macro::into_parser;
    pub use parse_macro::HasParser;
    pub use std::str::FromStr;
}

pub trait HasParser {
    type Parser<Input: combine::Stream<Token = char>>: Parser<Input, Output = Self>;

    fn parser<Input>() -> Self::Parser<Input>
    where
        Input: combine::Stream<Token = char>;
}

impl HasParser for char {
    #[into_parser]
    fn parser() -> _ {
        alpha_num()
    }
}

impl<A, B> HasParser for (A, B)
where
    A: HasParser,
    B: HasParser,
{
    #[into_parser]
    fn parser() -> _ {
        (A::parser(), B::parser())
    }
}

#[derive(Debug)]
pub enum Error {
    ParseInt(num::ParseIntError),
    Io(io::Error),
    ParseError(String),
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(e: num::ParseIntError) -> Self {
        Self::ParseInt(e)
    }
}

impl From<easy::Errors<char, &str, position::SourcePosition>> for Error {
    fn from(e: easy::Errors<char, &str, position::SourcePosition>) -> Self {
        Self::ParseError(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;

macro_rules! unsigned_number_parser {
    ($($id:ty),*) => {
        $(impl HasParser for $id {
            #[into_parser]
            fn parser() -> _ {
                many1(digit()).map(|s: String| s.parse::<Self>().unwrap())
            }
        })*
    }
}

unsigned_number_parser!(u8, u16, u32, u64, u128, usize);

macro_rules! signed_number_parser {
    ($($id:ty),*) => {
        $(impl HasParser for $id {
            #[into_parser]
            fn parser() -> _ {
                choice((
                    token('-').with(many1(digit()))
                        .map(|s: String| format!("-{s}").parse::<Self>().unwrap()),
                    u32::parser().map(|v| v.try_into().unwrap())
                ))
            }
        })*
    }
}

signed_number_parser!(i8, i16, i32, i64, i128, isize);

impl HasParser for String {
    #[into_parser]
    fn parser() -> _ {
        many1(any())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Comma;

#[derive(Debug, Clone, Copy)]
pub struct CommaSpace;

#[derive(Debug, Clone, Copy)]
pub struct NewLine;

#[derive(Debug, Clone, Copy)]
pub struct Space;

#[derive(Debug, Clone, Copy)]
pub struct SepBy<T>(PhantomData<T>);

#[derive(Debug, Clone, Copy)]
pub struct TermWith<T>(PhantomData<T>);

#[derive(Clone, Debug)]
pub struct List<T, Sep>(Vec<T>, PhantomData<Sep>);

#[derive(Clone, Debug)]
pub struct Nil;

impl<T, Sep> From<Vec<T>> for List<T, Sep> {
    fn from(v: Vec<T>) -> Self {
        Self(v, PhantomData)
    }
}

impl<T: HasParser> HasParser for List<T, Nil> {
    #[into_parser]
    fn parser() -> _ {
        many1(T::parser()).map(|v: Vec<_>| v.into())
    }
}

impl<T: HasParser> HasParser for List<T, SepBy<Comma>> {
    #[into_parser]
    fn parser() -> _ {
        sep_by1(T::parser(), token(',')).map(|v: Vec<_>| v.into())
    }
}

impl<T: HasParser> HasParser for List<T, SepBy<CommaSpace>> {
    #[into_parser]
    fn parser() -> _ {
        sep_by1(T::parser(), string(", ")).map(|v: Vec<_>| v.into())
    }
}

impl<T: HasParser> HasParser for List<T, SepBy<NewLine>> {
    #[into_parser]
    fn parser() -> _ {
        sep_by1(T::parser(), token('\n')).map(|v: Vec<_>| v.into())
    }
}

impl<T: HasParser> HasParser for List<T, TermWith<NewLine>> {
    #[into_parser]
    fn parser() -> _ {
        many1(T::parser().skip(token('\n'))).map(|v: Vec<_>| v.into())
    }
}

impl<T: HasParser> HasParser for List<T, SepBy<Space>> {
    #[into_parser]
    fn parser() -> _ {
        sep_by1(T::parser(), token(' ')).map(|v: Vec<_>| v.into())
    }
}

impl<T, Sep> List<T, Sep> {
    pub fn new() -> Self {
        Self(vec![], PhantomData)
    }

    pub fn push(&mut self, t: T) {
        self.0.push(t);
    }

    pub fn iter<'a>(&'a self) -> slice::Iter<'a, T> {
        self.0.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> slice::IterMut<'a, T> {
        self.0.iter_mut()
    }

    pub fn truncate(&mut self, size: usize) {
        self.0.truncate(size);
    }

    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional)
    }
}

impl<'a, T, Sep> IntoIterator for &'a List<T, Sep> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<'a, T, Sep> IntoIterator for &'a mut List<T, Sep> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}

impl<T, Sep> IntoIterator for List<T, Sep> {
    type Item = T;
    type IntoIter = vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T, Sep> iter::FromIterator<T> for List<T, Sep> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(iter::FromIterator::from_iter(iter), PhantomData)
    }
}

impl<T, Sep> AsRef<[T]> for List<T, Sep> {
    fn as_ref(&self) -> &[T] {
        self.0.as_ref()
    }
}

impl<T, Sep> AsMut<[T]> for List<T, Sep> {
    fn as_mut(&mut self) -> &mut [T] {
        self.0.as_mut()
    }
}

impl<T, Sep> Deref for List<T, Sep> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.0.deref()
    }
}

impl<T, Sep> DerefMut for List<T, Sep> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.0.deref_mut()
    }
}

pub fn parse_str<T: HasParser>(
    input: &str,
) -> std::result::Result<T, easy::Errors<char, &str, position::SourcePosition>> {
    let (t, _): (T, _) = T::parser()
        .skip(spaces())
        .skip(eof())
        .easy_parse(position::Stream::new(input))?;
    Ok(t)
}
