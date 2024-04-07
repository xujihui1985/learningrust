use derive_try_from_primitive::TryFromPrimitive;
use std::{
    fmt::{self, write, Display},
    ops::{Add, Range},
};

use derive_more::{Add, Sub};
use enumflags2::*;
use nom::{
    combinator::{map_res, verify},
    error::{context, ErrorKind, ParseError, VerboseError},
    Offset,
};

mod parse;

#[derive(Debug)]
pub enum SegmentContents {
    Dynamic(Vec<DynamicEntry>),
    Unknown,
}

#[derive(Debug)]
pub struct DynamicEntry {
    pub tag: DynamicTag,
    pub addr: Addr,
}

impl DynamicEntry {
    pub fn parse(i: parse::Input) -> parse::Result<Self> {
        use nom::sequence::tuple;
        let (i, (tag, addr)) = tuple((DynamicTag::parse, Addr::parse))(i)?;
        Ok((i, Self { tag, addr }))
    }
}

#[derive(Debug, Clone, Copy, TryFromPrimitive, PartialEq, Eq)]
#[repr(u64)]
pub enum DynamicTag {
    Null = 0,
    Needed = 1,
    PltRelSz = 2,
    PltGot = 3,
    Hash = 4,
    StrTab = 5,
    SymTab = 6,
    Rela = 7,
    RelaSz = 8,
    RelaEnt = 9,
    StrSz = 10,
    SymEnt = 11,
    Init = 12,
    Fini = 13,
    SoName = 14,
    RPath = 15,
    Symbolic = 16,
    Rel = 17,
    RelSz = 18,
    RelEnt = 19,
    PltRel = 20,
    Debug = 21,
    TextRel = 22,
    JmpRel = 23,
    BindNow = 24,
    InitArray = 25,
    FiniArray = 26,
    InitArraySz = 27,
    FiniArraySz = 28,
    LoOs = 0x60000000,
    HiOs = 0x6fffffff,
    LoProc = 0x70000000,
    HiProc = 0x7fffffff,
    GnuHash = 0x6ffffef5,
    Flags1 = 0x6ffffffb,
    RelACount = 0x6ffffff9,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Type {
    None = 0x0,
    Rel = 0x1,
    Exec = 0x2,
    Dyn = 0x3,
    Core = 0x4,
}

impl Type {
    pub fn to_u16(&self) -> u16 {
        match self {
            Self::None => 0,
            Self::Rel => 1,
            Self::Exec => 2,
            Self::Dyn => 3,
            Self::Core => 4,
        }
    }

    pub fn from_u16(value: u16) -> Option<Self> {
        match value {
            0 => Some(Self::None),
            1 => Some(Self::Rel),
            2 => Some(Self::Exec),
            3 => Some(Self::Dyn),
            4 => Some(Self::Core),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Machine {
    X86 = 0x03,
    X86_64 = 0x3e,
}

impl Machine {
    pub fn to_u16(&self) -> u16 {
        match self {
            Self::X86 => 0x03,
            Self::X86_64 => 0x3e,
        }
    }

    pub fn from_u16(value: u16) -> Option<Self> {
        match value {
            0x03 => Some(Self::X86),
            0x3e => Some(Self::X86_64),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum SegmentType {
    Null = 0x0,
    Load = 0x1,
    Dynamic = 0x2,
    Interp = 0x3,
    Note = 0x4,
    ShLib = 0x5,
    PHdr = 0x6,
    TLS = 0x7,
    LoOS = 0x6000_0000,
    HiOS = 0x6FFF_FFFF,
    LoProc = 0x7000_0000,
    HiProc = 0x7FFF_FFFF,
    GnuEhFrame = 0x6474_E550,
    GnuStack = 0x6474_E551,
    GnuRelRo = 0x6474_E552,
    GnuProperty = 0x6474_E553,
}

impl SegmentType {
    pub fn to_u32(&self) -> u32 {
        match self {
            Self::Null => 0x0,
            Self::Load => 0x1,
            Self::Dynamic => 0x2,
            Self::Interp => 0x3,
            Self::Note => 0x4,
            Self::ShLib => 0x5,
            Self::PHdr => 0x6,
            Self::TLS => 0x7,
            Self::LoOS => 0x6000_0000,
            Self::HiOS => 0x6FFF_FFFF,
            Self::LoProc => 0x7000_0000,
            Self::HiProc => 0x7FFF_FFFF,
            Self::GnuEhFrame => 0x6474_E550,
            Self::GnuStack => 0x6474_E551,
            Self::GnuRelRo => 0x6474_E552,
            Self::GnuProperty => 0x6474_E553,
        }
    }

    pub fn from_u16(value: u32) -> Option<Self> {
        match value {
            0x0 => Some(Self::Null),
            0x1 => Some(Self::Load),
            0x2 => Some(Self::Dynamic),
            0x3 => Some(Self::Interp),
            0x4 => Some(Self::Note),
            0x5 => Some(Self::ShLib),
            0x6 => Some(Self::PHdr),
            0x7 => Some(Self::TLS),
            0x6000_0000 => Some(Self::LoOS),
            0x6FFF_FFFF => Some(Self::HiOS),
            0x7000_0000 => Some(Self::LoProc),
            0x7FFF_FFFF => Some(Self::HiProc),
            0x6474_E550 => Some(Self::GnuEhFrame),
            0x6474_E551 => Some(Self::GnuStack),
            0x6474_E552 => Some(Self::GnuRelRo),
            0x6474_E553 => Some(Self::GnuProperty),
            _ => None,
        }
    }
}

#[bitflags]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum SegmentFlag {
    Execute = 0x1,
    Write = 0x2,
    Read = 0x4,
}

impl_parse_for_enum!(Type, le_u16);
impl_parse_for_enum!(Machine, le_u16);
impl_parse_for_enum!(SegmentType, le_u32);
impl_parse_for_enum!(DynamicTag, le_u64);
impl_parse_for_enumflags!(SegmentFlag, le_u32);
impl_parse_for_enum!(KnownRelType, le_u32);

pub struct ProgramHeader {
    pub r#type: SegmentType,
    pub flags: BitFlags<SegmentFlag>,
    pub offset: Addr,
    pub vaddr: Addr,
    pub paddr: Addr,
    pub filesz: Addr,
    pub memsz: Addr,
    pub align: Addr,
    pub data: Vec<u8>,
    pub contents: SegmentContents,
}

impl ProgramHeader {
    pub fn file_range(&self) -> Range<Addr> {
        self.offset..self.offset + self.filesz
    }

    pub fn mem_range(&self) -> Range<Addr> {
        self.vaddr..self.vaddr + self.memsz
    }

    pub fn parse<'a>(full_input: parse::Input<'a>, i: parse::Input<'a>) -> parse::Result<'a, Self> {
        use nom::sequence::tuple;
        let (i, (r#type, flags)) = tuple((SegmentType::parse, SegmentFlag::parse))(i)?;
        let ap = Addr::parse;
        let (i, (offset, vaddr, paddr, filesz, memsz, align)) = tuple((ap, ap, ap, ap, ap, ap))(i)?;
        use nom::{combinator::map, multi::many_till};

        let slice = &full_input[offset.into()..][..filesz.into()];
        let (_, contents) = match r#type {
            SegmentType::Dynamic => map(
                many_till(
                    DynamicEntry::parse,
                    verify(DynamicEntry::parse, |e| e.tag == DynamicTag::Null),
                ),
                |(entries, _last)| SegmentContents::Dynamic(entries),
            )(slice)?,
            _ => (slice, SegmentContents::Unknown),
        };
        let res = Self {
            r#type,
            flags,
            offset,
            vaddr,
            paddr,
            filesz,
            memsz,
            align,
            // `to_vec()` turns a slice into an owned Vec (this works because u8 is Clone+Copy)
            data: full_input[offset.into()..][..filesz.into()].to_vec(),
            contents,
        };
        Ok((i, res))
    }
}

impl fmt::Debug for ProgramHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "file {:?} | mem {:?} | align {:?} | {} {:?}",
            self.file_range(),
            self.mem_range(),
            self.align,
            &[
                (SegmentFlag::Read, "R"),
                (SegmentFlag::Write, "W"),
                (SegmentFlag::Execute, "X"),
            ]
            .iter()
            .map(|&(flag, letter)| {
                if self.flags.contains(flag) {
                    letter
                } else {
                    "."
                }
            })
            .collect::<Vec<_>>()
            .join(""),
            self.r#type,
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Add, Sub)]
pub struct Addr(pub u64);

impl fmt::Debug for Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:08x}", self.0)
    }
}

impl Display for Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl Into<u64> for Addr {
    fn into(self) -> u64 {
        self.0
    }
}

impl Into<usize> for Addr {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl From<u64> for Addr {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl Addr {
    pub fn parse(i: parse::Input) -> parse::Result<Self> {
        use nom::{combinator::map, number::complete::le_u64};
        context("Entrypoint", map(le_u64, |x| x.into()))(i)
    }
}

#[derive(Debug)]
pub struct Rela {
    pub offset: Addr,
    pub r#type: RelType,
    pub sym: u32,
    pub addent: Addr,
}

impl Rela {
    pub fn parse(i: parse::Input) -> parse::Result<Self> {
        use nom::{combinator::map, number::complete::le_u32, sequence::tuple};
        map(
            tuple((Addr::parse, RelType::parse, le_u32, Addr::parse)),
            |(offset, r#type, sym, addent)| Self {
                offset,
                r#type,
                sym,
                addent,
            },
        )(i)
    }
}

#[derive(Debug, TryFromPrimitive, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum KnownRelType {
    _64 = 1,
    Copy = 5,
    GlobDat = 6,
    JumpSlot = 7,
    Relative = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelType {
    Known(KnownRelType),
    Unknown(u32),
}

impl RelType {
    pub fn parse(i: parse::Input) -> parse::Result<Self> {
        use nom::{branch::alt, combinator::map, number::complete::le_u32};

        alt((
            map(KnownRelType::parse, Self::Known),
            map(le_u32, Self::Unknown),
        ))(i)
    }
}

#[derive(Debug)]
pub struct File {
    pub r#type: Type,
    pub machine: Machine,
    pub entry_point: Addr,
    pub program_headers: Vec<ProgramHeader>,
    pub section_headers: Vec<SectionHeader>,
}

impl File {
    const MAGIC: &'static [u8] = &[0x7f, 0x45, 0x4c, 0x46];

    pub fn slice_at(&self, mem_addr: Addr) -> Option<&[u8]> {
        self.segment_at(mem_addr)
            .map(|seg| &seg.data[(mem_addr - seg.mem_range().start).into()..])
    }

    pub fn get_string(&self, offset: Addr) -> Result<String, GetStringError> {
        use DynamicTag as DT;
        use GetStringError as E;

        let addr = self.dynmaic_entry(DT::StrTab).ok_or(E::StrTabNotFound)?;
        let slice = self.slice_at(addr + offset).ok_or(E::StrTabSegmentNotFound)?;
        let string_slice = slice.split(|&c| c == 0).next().ok_or(E::StringNotFound)?;
        Ok(String::from_utf8_lossy(string_slice).into())
    }

    pub fn segment_at(&self, addr: Addr) -> Option<&ProgramHeader> {
        self.program_headers
            .iter()
            .filter(|ph| ph.r#type == SegmentType::Load)
            .find(|ph| ph.mem_range().contains(&addr))
    }

    pub fn segment_of_type(&self, r#type: SegmentType) -> Option<&ProgramHeader> {
        self.program_headers.iter().find(|ph| ph.r#type == r#type)
    }

    pub fn dynamic_table(&self) -> Option<&[DynamicEntry]> {
        match self.segment_of_type(SegmentType::Dynamic) {
            Some(ProgramHeader{
                contents: SegmentContents::Dynamic(entries),
                ..
            }) => Some(entries),
            _ => None,
        }
    }

    pub fn dynmaic_entries<'a>(&self, tag_v: DynamicTag) -> impl Iterator<Item=Addr> + '_ {
        self.dynamic_table()
            .unwrap_or_default()
            .iter()
            .filter(move |e| e.tag == tag_v)
            .map(|e| e.addr)
    }

    pub fn dynamic_entry_strings(&self, tag: DynamicTag) -> impl Iterator<Item = String> + '_ {
        self.dynmaic_entries(tag)
            .filter_map(move |addr| self.get_string(addr).ok())
    }

    pub fn dynmaic_entry(&self, tag: DynamicTag) -> Option<Addr> {
        self.dynmaic_entries(tag).next()
    }

    pub fn read_rela_entries(&self) -> Result<Vec<Rela>, ReadRelaError> {
        use DynamicTag as DT;
        use ReadRelaError as E;

        let addr = self.dynmaic_entry(DT::Rela).ok_or(E::RelaNotFound)?;
        let len = self.dynmaic_entry(DT::RelSz).ok_or(E::RelaSzNotFound)?;
        let ent = self.dynmaic_entry(DT::RelaEnt).ok_or(E::RelaEntNotFound)?;

        // let seg = self.segment_at(addr).ok_or(E::RelaSegmentNotFound)?;
        // let i = &seg.data[(addr - seg.mem_range().start).into()..][..len.into()];

        let i = self.slice_at(addr).ok_or(E::RelaSegmentNotFound)?;
        let i = &i[..len.into()];
        let n = (len.0 - ent.0) as usize;

        use nom::multi::many_m_n;
        match many_m_n(n, n, Rela::parse)(i) {
            Ok((_, entries)) => Ok(entries),
            Err(nom::Err::Failure(err)) | Err(nom::Err::Error(err)) => {
                let e = &err.errors[0];
                let (_input, error_kind) = e;
                Err(E::ParsingError(error_kind.clone()))
            }
            _ => unreachable!(),
        }
    }

    pub fn section_starting_at(&self, addr: Addr) -> Option<&SectionHeader> {
        self.section_headers.iter().find(|sh| sh.addr == addr)
    }
    pub fn read_syms(&self) -> Result<Vec<Sym>, ReadSymsError> {
        use DynamicTag as DT;
        use ReadSymsError as E;

        let addr = self.dynmaic_entry(DT::SymTab).ok_or(E::SymTabNotFound)?;
        let section = self
            .section_starting_at(addr)
            .ok_or(E::SymTabSectionNotFound)?;

        let i = self.slice_at(addr).ok_or(E::SymTabSegmentNotFound)?;
        let n = (section.size.0 / section.entsize.0) as usize;
        use nom::multi::many_m_n;

        match many_m_n(n, n, Sym::parse)(i) {
            Ok((_, syms)) => Ok(syms),
            Err(nom::Err::Failure(err)) | Err(nom::Err::Error(err)) => {
                let e = &err.errors[0];
                let (_input, error_kind) = e;
                Err(E::ParsingError(parse::ErrorKind::Nom(error_kind.clone())))
            }
            // we don't use any "streaming" parsers, so.
            _ => unreachable!(),
        }
    }

    pub fn parse_or_print_error(i: parse::Input) -> Option<Self> {
        match Self::parse(i) {
            Ok((_, f)) => Some(f),
            Err(nom::Err::Failure(err)) | Err(nom::Err::Error(err)) => {
                for (input, err) in err.errors {
                    let offset = i.offset(input);
                    eprintln!("{:?} at: position {}:", err, offset);
                    eprintln!("{:>08x}: {:?}", offset, HexDump(input));
                }
                None
            }
            Err(_) => panic!("unexpected error"),
        }
    }

    pub fn parse(input: parse::Input) -> parse::Result<Self> {
        let full_input = input;
        use nom::{
            bytes::complete::{tag, take},
            combinator::map,
            number::complete::{le_u16, le_u32},
            sequence::tuple,
        };

        let (i, _) = tuple((
            context("Magic", tag(Self::MAGIC)),
            context("Class", tag(&[0x2])),
            context("Endianness", tag(&[0x1])),
            context("Version", tag(&[0x1])),
            context("OS ABI", nom::branch::alt((tag(&[0x0]), tag(&[0x3])))),
            context("Padding", take(8_usize)),
        ))(input)?;
        let (i, (r#type, machine)) = tuple((Type::parse, Machine::parse))(i)?;
        let (i, _) = context("Version (bis)", verify(le_u32, |&x| x == 1))(i)?;
        let (i, entry_point) = Addr::parse(i)?;

        let u16_usize = map(le_u16, |x| x as usize);
        let (i, (ph_offset, sh_offset)) = tuple((Addr::parse, Addr::parse))(i)?;
        let (i, (flag, hdr_size)) = tuple((le_u32, le_u16))(i)?;
        let (i, (ph_entsize, ph_count)) = tuple((&u16_usize, &u16_usize))(i)?;
        let (i, (sh_entsize, sh_count, sh_nidx)) = tuple((&u16_usize, &u16_usize, &u16_usize))(i)?;

        let ph_slices = (&full_input[ph_offset.into()..]).chunks(ph_entsize);
        let mut program_headers = Vec::new();
        for ph_slice in ph_slices.take(ph_count) {
            let (_, ph) = ProgramHeader::parse(full_input, ph_slice)?;
            program_headers.push(ph);
        }

        let sh_slices = (&full_input[sh_offset.into()..]).chunks(sh_entsize);
        let mut section_headers = Vec::new();
        for sh_slice in sh_slices.take(sh_count) {
            let (_, sh) = SectionHeader::parse(sh_slice)?;
            section_headers.push(sh);
        }

        Ok((
            i,
            Self {
                r#type,
                machine,
                entry_point,
                program_headers,
                section_headers,
            },
        ))
    }
}

pub struct HexDump<'a>(&'a [u8]);

impl<'a> fmt::Debug for HexDump<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &x in self.0.iter().take(20) {
            write!(f, "{:02x} ", x)?;
        }
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ReadRelaError {
    #[error("Rela dynamic entry not found")]
    RelaNotFound,
    #[error("RelaSz dynamic entry not found")]
    RelaSzNotFound,
    #[error("Rela Segment not found")]
    RelaSegmentNotFound,
    #[error("parsing error")]
    ParsingError(parse::ErrorKind),
    #[error("RelaEnt dynamic entry not found")]
    RelaEntNotFound,
    #[error("RelaSeg dynamic entry not found")]
    RelaSegNotFound,
}

#[derive(thiserror::Error, Debug)]
pub enum ReadSymsError {
    #[error("SymTab dynamic entry not found")]
    SymTabNotFound,
    #[error("SymTab section not found")]
    SymTabSectionNotFound,
    #[error("SymTab segment not found")]
    SymTabSegmentNotFound,
    #[error("Parsing error")]
    ParsingError(parse::ErrorKind),
}


#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum SymBind {
    Local = 0,
    Global = 1,
    Weak = 2,
}

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum SymType {
    None = 0,
    Object = 1,
    Func = 2,
    Section = 3,
}

impl SymBind {
    pub fn parse(i: parse::BitInput) -> parse::BitResult<Option<Self>> {
        use nom::{bits::complete::take, combinator::map};
        map(
            take(4_usize),
            |i: u8| Self::try_from(i).ok()
        )(i)
    }
}

impl SymType {
    pub fn parse(i: parse::BitInput) -> parse::BitResult<Option<Self>> {
        use nom::{bits::complete::take, combinator::map};
        map(
            take(4_usize),
            |i: u8| Self::try_from(i).ok()
        )(i)
    }
}

#[derive(Clone, Copy)]
pub struct SectionIndex(pub u16);

impl SectionIndex {
    pub fn is_undef(&self) -> bool {
        self.0 == 0
    }

    pub fn is_special(&self) -> bool {
        self.0 >= 0xff00
    }

    pub fn get(&self) -> Option<usize> {
        if self.is_undef() || self.is_special() {
            None
        } else {
            Some(self.0 as usize)
        }
    }
}

impl fmt::Debug for SectionIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_special() {
            write!(f, "Special({:04x})", self.0)
        } else if self.is_undef() {
            write!(f, "Undef")
        } else {
            write!(f, "{}", self.0)
        }
    }
}

pub struct Sym {
    pub name: Addr,
    pub bind: Option<SymBind>,
    pub r#type: Option<SymType>,
    pub shndx: SectionIndex,
    pub value: Addr,
    pub size: u64,
}

impl Sym {
    pub fn parse(i: parse::Input) -> parse::Result<Self> {
        use nom::{
            bits::bits,
            combinator::map,
            number::complete::{le_u16, le_u32, le_u64, le_u8},
            sequence::tuple
        };

        let (i, (name, (bind, r#type), _reserved, shndx, value, size)) = tuple((
            map(le_u32, |x| Addr(x as u64)),
            bits(tuple((SymBind::parse, SymType::parse))),
            le_u8,
            map(le_u16, SectionIndex),
            Addr::parse,
            le_u64
        ))(i)?;
        let res = Self {
            name,
            bind,
            r#type,
            shndx,
            size,
            value,
        };
        Ok((i, res))
    }
}


#[derive(Debug)]
pub struct SectionHeader {
    pub name: Addr,
    pub r#type: u32,
    pub flags: u64,
    pub addr: Addr,
    pub off: Addr,
    pub size: Addr,
    pub link: u32,
    pub info: u32,
    pub addralign: Addr,
    pub entsize: Addr,
}

impl SectionHeader {
    pub fn parse(i: parse::Input) -> parse::Result<Self> {
        use nom::{
            combinator::map,
            number::complete::{le_u32, le_u64},
            sequence::tuple,
        };
        let (i, (name, r#type, flags, addr, off, size, link, info, addralign, entsize)) =
            tuple((
                map(le_u32, |x| Addr(x as u64)),
                le_u32,
                le_u64,
                Addr::parse,
                Addr::parse,
                Addr::parse,
                le_u32,
                le_u32,
                Addr::parse,
                Addr::parse,
            ))(i)?;
        let res = Self {
            name,
            r#type,
            flags,
            addr,
            off,
            size,
            link,
            info,
            addralign,
            entsize,
        };
        Ok((i, res))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetStringError {
    #[error("String table not found")]
    StrTabNotFound,
    #[error("String table segment not found")]
    StrTabSegmentNotFound,
    #[error("String not found")]
    StringNotFound,
}

#[cfg(test)]
mod tests {

    #[test]
    fn type_to_u16() {
        assert_eq!(super::Type::Dyn.to_u16(), 0x3);
        assert_eq!(super::Type::from_u16(0xff), None);
        assert_eq!(super::Type::from_u16(0x3), Some(super::Type::Dyn));
    }

    #[test]
    fn test_bitflag() {
        use super::SegmentFlag;
        use enumflags2::*;

        let flag_interger = 6;
        let flags = BitFlags::<SegmentFlag>::from_bits(flag_interger).unwrap();
        assert_eq!(flags, SegmentFlag::Read | SegmentFlag::Write);
        assert_eq!(flags.bits(), flag_interger);

        assert!(BitFlags::<SegmentFlag>::from_bits(1999).is_err());
    }
}
