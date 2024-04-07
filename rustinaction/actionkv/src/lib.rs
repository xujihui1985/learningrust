use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{self, BufReader, BufWriter, Read, Seek, Write},
    path::Path,
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crc::crc32;

type ByteString = Vec<u8>;
type ByteStr = [u8];

pub struct KeyValuePair {
    pub key: ByteString,
    pub value: ByteString,
}

pub struct ActionKV {
    f: File,
    pub index: HashMap<ByteString, u64>,
}

impl ActionKV {
    pub fn open(path: &Path) -> io::Result<Self> {
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(path)?;
        let index = HashMap::new();
        Ok(Self { f, index })
    }

    pub fn load(&mut self) -> io::Result<()> {
        let mut f = BufReader::new(&self.f);

        loop {
            let position = f.seek(io::SeekFrom::Current(0))?;

            let maybe_kv = ActionKV::process_record(&mut f);

            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => match err.kind() {
                    io::ErrorKind::UnexpectedEof => {
                        break;
                    }
                    _ => return Err(err),
                },
            };
            self.index.insert(kv.key, position);
        }
        Ok(())
    }

    pub fn insert(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        let position = self.insert_no_index(key, value)?;
        self.index.insert(key.to_vec(), position);
        Ok(())
    }

    pub fn get(&mut self, key: &ByteStr) -> io::Result<Option<ByteString>> {
        let position = match self.index.get(key) {
            None => return Ok(None),
            Some(p) => *p,
        };
        let kv = self.get_at(position)?;
        Ok(Some(kv.value))
    }

    pub fn find(&mut self, target: &ByteStr) -> io::Result<Option<(u64, ByteString)>> {
        let mut f = BufReader::new(&self.f);
        let mut found: Option<(u64, ByteString)> = None;

        loop {
            let position = f.seek(io::SeekFrom::Current(0))?;

            let maybe_kv = ActionKV::process_record(&mut f);
            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => match err.kind() {
                    io::ErrorKind::UnexpectedEof => {
                        break;
                    }
                    _ => return Err(err),
                },
            };
            if kv.key == target {
                found = Some((position, kv.value));
            }
        }
        Ok(found)
    }

    pub fn update(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        self.insert(key, value)
    }

    pub fn delete(&mut self, key: &ByteStr) -> io::Result<()> {
        self.insert(key, b"")
    }

    pub fn get_at(&mut self, position: u64) -> io::Result<KeyValuePair> {
        let mut f = BufReader::new(&self.f);
        f.seek(io::SeekFrom::Start(position))?;
        let kv = ActionKV::process_record(&mut f)?;
        Ok(kv)
    }

    fn insert_no_index(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<u64> {
        let mut f = BufWriter::new(&self.f);
        let key_len = key.len();
        let value_len = value.len();
        let mut tmp = ByteString::with_capacity(key_len + value_len);
        tmp.extend_from_slice(key);
        tmp.extend_from_slice(value);
        let checksum = crc32::checksum_ieee(&tmp);
        let next_byte = io::SeekFrom::End(0);
        let current_position = f.seek(io::SeekFrom::Current(0))?; // get current position
        f.seek(next_byte)?; // seek to end
        f.write_u32::<LittleEndian>(checksum)?;
        f.write_u32::<LittleEndian>(key_len as u32)?;
        f.write_u32::<LittleEndian>(value_len as u32)?;
        f.write_all(&tmp)?;
        Ok(current_position)
    }

    fn process_record<R: Read>(f: &mut R) -> io::Result<KeyValuePair> {
        let saved_checksum = f.read_u32::<LittleEndian>()?;
        let key_len = f.read_u32::<LittleEndian>()?;
        let value_len = f.read_u32::<LittleEndian>()?;

        let data_len = key_len + value_len;
        let mut data = ByteString::with_capacity(data_len as usize);

        f.take(data_len as u64).read_to_end(&mut data)?;

        let checksum = crc32::checksum_ieee(&data);
        if checksum != saved_checksum {
            panic!(
                "data corruption encounted ({:08x}, {:08x})",
                checksum, saved_checksum
            );
        }
        let value = data.split_off(key_len as usize);
        let key = data;
        Ok(KeyValuePair { key, value })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut v = Vec::<u8>::with_capacity(10);
        let v1: Vec<u8> = vec![1,2,3,4,5];
        let v2: Vec<u8> = vec![5,6,7,8,9];
        v.extend_from_slice(&v1);
        v.extend_from_slice(&v2);
        println!("len of v {}", v.len());
    }
}
