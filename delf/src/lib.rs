use nom::error::VerboseError;

mod parse;

#[derive(Debug)]
pub struct File {}

impl File {
    const MAGIC: &'static [u8] = &[0x7f, 0x45, 0x4c, 0x46];

    pub fn parse(input: &[u8]) -> parse::Result<Self> {
        use {
            nom::{
                bytes::complete::{tag, take},
                error::context,
                sequence::tuple,
            }
        };

        let (i, _) = tuple((
            context("Magic", tag(Self::MAGIC)),
            context("Class", tag(&[0x2])),
            context("Endianness", tag(&[0x1])),
            context("Version", tag(&[0x1])),
            context("OS ABI", nom::branch::alt((tag(&[0x0]), tag(&[0x3])))),
            context("Padding", take(8_usize)),
        ))(input)?;
        Ok((i, Self{}))
    }
}