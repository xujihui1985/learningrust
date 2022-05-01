use std::convert::TryFrom;
use std::fs::OpenOptions;

use byteorder::{LittleEndian, ReadBytesExt};
use num_enum::TryFromPrimitive;
use positioned_io::{Cursor, ReadAt, Slice};

type Result<T> = anyhow::Result<T>;

fn main() -> Result<()> {
    let file = OpenOptions::new().read(true).open("/dev/sda1")?;

    let sb = Superblock::new(&file)?;

    let inode = InodeNumber(2).inode(&sb, &file)?;

    println!("({:?}), {:?}", inode.filetype(), inode);

    let root_entries = inode.dir_entries(&sb, &file)?;

    println!("{:#?}", root_entries);

    let etc = inode
        .child("etc", &sb, &file)?
        .expect("/etc should exists")
        .inode(&sb, &file)?;

    println!("found etc {:#?}", etc.filetype());

    let hosts = etc
        .child("hosts", &sb, &file)?
        .expect("hosts should exists")
        .inode(&sb, &file)?;
    
    println!("found hosts {:#?}", hosts.filetype());

    let hosts_data = hosts.data(&sb, &file)?;
    let hosts_data = Reader::new(&hosts_data).vec(0, hosts.size as usize)?;
    let hosts_data = String::from_utf8_lossy(&hosts_data);

    println!("{}", hosts_data);
    Ok(())
}

#[derive(Debug)]
struct Superblock {
    magic: u16,
    block_size: u64,
    blocks_per_group: u64,
    inodes_per_group: u64,
    inode_size: u64,
}

impl Superblock {
    fn new(dev: &dyn ReadAt) -> Result<Self> {
        let r = Reader::new(Slice::new(dev, 1024, None));

        Ok(Self {
            magic: r.u16(0x38)?,
            block_size: (2 << 10 + r.u32(0x18)? - 1) as u64,
            blocks_per_group: r.u32(0x20)? as u64,
            inodes_per_group: r.u32(0x28)? as u64,
            inode_size: r.u16(0x58)? as u64,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct BlockGroupDescriptor {
    inode_table: u64,
}

impl BlockGroupDescriptor {
    const SIZE: u64 = 64;

    fn new(slice: &dyn ReadAt) -> Result<Self> {
        let r = Reader::new(slice);
        Ok(Self {
            inode_table: r.u64_lohi(0x8, 0x28)?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct BlockGroupNumber(u64);

impl BlockGroupNumber {
    fn desc_slice(self, sb: &Superblock, dev: impl ReadAt) -> Slice<impl ReadAt> {
        let gdt_start = 1 * sb.block_size;
        let offset = gdt_start + self.0 * BlockGroupDescriptor::SIZE;
        Slice::new(dev, offset, None)
    }

    fn desc(self, sb: &Superblock, dev: &dyn ReadAt) -> Result<BlockGroupDescriptor> {
        let slice = self.desc_slice(sb, dev);
        BlockGroupDescriptor::new(&slice)
    }
}

#[derive(Debug, Clone, Copy)]
struct InodeNumber(u64);
impl InodeNumber {
    fn blockgroup_number(self, sb: &Superblock) -> BlockGroupNumber {
        let n = (self.0 - 1) / sb.inodes_per_group;
        BlockGroupNumber(n)
    }

    fn inode_slice(self, sb: &Superblock, dev: impl ReadAt) -> Result<Slice<impl ReadAt>> {
        let desc = self.blockgroup_number(sb).desc(sb, &dev)?;
        let table_off = desc.inode_table * sb.block_size;
        let idx_in_table = (self.0 - 1) % sb.inodes_per_group;
        let inode_off = table_off + sb.inode_size * idx_in_table;
        Ok(Slice::new(dev, inode_off, Some(sb.inode_size)))
    }

    fn inode(self, sb: &Superblock, dev: impl ReadAt) -> Result<Inode> {
        let slice = self.inode_slice(sb, dev)?;
        Inode::new(&slice)
    }
}

#[derive(Debug)]
struct Inode {
    mode: u16,
    size: u64,
    block: Vec<u8>,
}

#[derive(Debug, TryFromPrimitive)]
#[repr(u16)]
enum Filetype {
    Fifo = 0x1000,
    Char = 0x2000,
    Dir = 0x4000,
    Block = 0x6000,
    Reg = 0x8000,
    Sym = 0xA000,
    Socket = 0xC000,
}

impl Inode {
    fn new(slice: &dyn ReadAt) -> Result<Self> {
        let r = Reader::new(slice);
        Ok(Self {
            mode: r.u16(0x0)?,
            size: r.u64_lohi(0x4, 0x6C)?,
            block: r.vec(0x28, 60)?,
        })
    }

    fn filetype(&self) -> Filetype {
        Filetype::try_from(self.mode & 0xF000).unwrap()
    }

    fn data(&self, sb: &Superblock, dev: impl ReadAt) -> Result<Slice<impl ReadAt>> {
        // let ext_header = ExtentHeader::new(&Slice::new(&self.block, 0, Some(12)))?;
        let ext = Extent::new(&Slice::new(&self.block, 12, Some(12)))?;
        let offset = ext.start * sb.block_size;
        let len = ext.len * sb.block_size;
        Ok(Slice::new(dev, offset, Some(len)))
    }

    fn dir_entries(&self, sb: &Superblock, dev: impl ReadAt) -> Result<Vec<DirectoryEntry>> {
        let data = self.data(sb, dev)?;
        let mut entries = Vec::new();

        let mut offset = 0u64;
        loop {
            let entry = DirectoryEntry::new(&Slice::new(&data, offset, None))?;
            if entry.inode.0 == 0 {
                break;
            }
            offset += entry.len;
            entries.push(entry);
        }
        Ok(entries)
    }

    fn child(&self, name: &str, sb: &Superblock, dev: &dyn ReadAt) -> Result<Option<InodeNumber>> {
        let entries = self.dir_entries(sb, dev)?;
        Ok(entries
            .into_iter()
            .filter(|x| x.name == name)
            .map(|x| x.inode)
            .next())
    }
}

struct Reader<IO: ReadAt> {
    inner: IO,
}

impl<IO: ReadAt> Reader<IO> {
    fn new(inner: IO) -> Self {
        Self { inner }
    }

    fn u16(&self, offset: u64) -> Result<u16> {
        let mut cursor = Cursor::new_pos(&self.inner, offset);
        Ok(cursor.read_u16::<LittleEndian>()?)
    }

    fn u8(&self, offset: u64) -> Result<u8> {
        let mut cursor = Cursor::new_pos(&self.inner, offset);
        Ok(cursor.read_u8()?)
    }

    fn u32(&self, offset: u64) -> Result<u32> {
        let mut cursor = Cursor::new_pos(&self.inner, offset);
        Ok(cursor.read_u32::<LittleEndian>()?)
    }

    fn u64_lohi(&self, lo: u64, hi: u64) -> Result<u64> {
        Ok(self.u32(lo)? as u64 + ((self.u32(hi)? as u64) << 32))
    }

    fn vec(&self, offset: u64, len: usize) -> Result<Vec<u8>> {
        let mut v = vec![0u8; len];
        self.inner.read_exact_at(offset, &mut v)?;
        Ok(v)
    }
}

#[derive(Debug)]
struct ExtentHeader {
    entries: u64,
    depth: u64,
}

impl ExtentHeader {
    fn new(slice: &dyn ReadAt) -> Result<Self> {
        let r = Reader::new(slice);
        let magic = r.u16(0x0)?;
        assert_eq!(magic, 0xF30A);

        Ok(Self {
            entries: r.u16(0x2)? as u64,
            depth: r.u16(0x6)? as u64,
        })
    }
}

#[derive(Debug)]
struct Extent {
    len: u64,
    start: u64,
}

impl Extent {
    fn new(slice: &dyn ReadAt) -> Result<Self> {
        let r = Reader::new(slice);
        Ok(Self {
            len: r.u16(0x4)? as u64,
            start: ((r.u16(0x6)? as u64) << 32) + r.u32(0x8)? as u64,
        })
    }
}

#[derive(Debug)]
struct DirectoryEntry {
    len: u64,
    inode: InodeNumber,
    name: String,
}

impl DirectoryEntry {
    fn new(slice: &dyn ReadAt) -> Result<Self> {
        let r = Reader::new(slice);
        let name_len = r.u8(0x6)? as usize;
        Ok(Self {
            inode: InodeNumber(r.u32(0x0)? as u64),
            len: r.u16(0x4)? as u64,
            name: String::from_utf8_lossy(&r.vec(0x8, name_len)?).into(),
        })
    }
}
