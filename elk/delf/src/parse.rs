#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    Nom(nom::error::ErrorKind),
    Context(&'static str),
}

pub struct Error<I> {
    pub errors: Vec<(I, ErrorKind)>,
}

impl<I> nom::error::ParseError<I> for Error<I> {
    fn from_error_kind(input: I, kind: nom::error::ErrorKind) -> Self {
        let errors = vec![(input, ErrorKind::Nom(kind))];
        Self { errors }
    }

    fn append(input: I, kind: nom::error::ErrorKind, mut other: Self) -> Self {
        other.errors.push((input, ErrorKind::Nom(kind)));
        other
    }

    fn add_context(input: I, ctx: &'static str, mut other: Self) -> Self {
        other.errors.push((input, ErrorKind::Context(ctx)));
        other
    }
}

pub type Input<'a> = &'a [u8];
pub type Result<'a, O> = nom::IResult<Input<'a>, O, nom::error::VerboseError<Input<'a>>>;

pub type BitInput<'a> = (&'a [u8], usize);
pub type BitResult<'a, O> = nom::IResult<BitInput<'a>, O, Error<BitInput<'a>>>;

use nom::{ErrorConvert, Slice};
use std::{collections::btree_map::Range, ops::RangeFrom};

impl<I> ErrorConvert<Error<I>> for Error<(I, usize)>
where
    I: Slice<RangeFrom<usize>>,
{
    fn convert(self) -> Error<I> {
        let errors = self
            .errors
            .into_iter()
            .map(|((rest, offset), err)| (rest.slice(offset / 8..), err))
            .collect();
        Error { errors }
    }
}

#[macro_export]
macro_rules! impl_parse_for_enum {
    ($type: ident, $number_parser: ident) => {
        impl $type {
            pub fn parse(input: parse::Input) -> parse::Result<Self> {
                let origin_input = input;

                use nom::{number::complete::$number_parser, Err};
                let parser = map_res($number_parser, move |x| {
                    Self::from_u16(x).ok_or(Err::Failure(VerboseError::from_error_kind(
                        origin_input,
                        ErrorKind::Alt,
                    )))
                });

                context(stringify!($type), parser)(input)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_parse_for_enumflags {
    ($type: ident, $number_parser: ident) => {
        impl $type {
            pub fn parse(input: parse::Input) -> parse::Result<enumflags2::BitFlags<Self>> {
                use nom::{
                    combinator::map_res,
                    error::{context, ErrorKind},
                    number::complete::$number_parser,
                };
                let parser = map_res($number_parser, |x| {
                    enumflags2::BitFlags::<Self>::from_bits(x).map_err(|_| ErrorKind::Alt)
                });
                context(stringify!($type), parser)(input)
            }
        }
    };
}
