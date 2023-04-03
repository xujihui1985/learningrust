pub type Input<'a> = &'a [u8];
pub type Result<'a, O> = nom::IResult<Input<'a>, O, nom::error::VerboseError<Input<'a>>>;

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
                context(stringify!($type),parser)(input)
            }
       } 
    };
}