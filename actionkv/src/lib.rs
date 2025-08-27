use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crc::crc32;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, SeekFrom};
use std::path::Path;

type ByteString = Vec<u8>;
type ByteStr = [u8];

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValuePair {
    pub key: ByteString,
    pub value: ByteString,
}

pub struct ActionKV {
    file: File,
    pub index: HashMap<ByteString, u64>,
}

impl ActionKV {
    pub fn open(path: &Path) -> io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open(path)?;
        let index = HashMap::new();
        Ok(ActionKV { file, index })
    }

    pub fn load(&mut self) -> io::Result<()> {
        let mut file = BufReader::new(&mut self.file);
        loop {
            let position = file.stream_position()?;
            let maybe_kv = ActionKV::process_record(&mut file);

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

    fn process_record<R: Read>(file: &mut R) -> io::Result<KeyValuePair> {
        let saved_checksum = file.read_u32::<LittleEndian>()?;
        let key_len = file.read_u32::<LittleEndian>()?;
        let value_len = file.read_u32::<LittleEndian>()?;
        let data_len = key_len + value_len;
        let mut data = ByteString::with_capacity(data_len as usize);
        {
            file.by_ref().take(data_len as u64).read_to_end(&mut data)?;
        }

        let checksum = crc32::checksum_ieee(&data);
        if checksum != saved_checksum {
            panic!("data corruption encountered ({checksum:08x} != {saved_checksum:08x})");
        }

        let value = data.split_off(key_len as usize);
        let key = data;

        Ok(KeyValuePair { key, value })
    }

    pub fn get(&mut self, key: &ByteStr) -> io::Result<Option<ByteString>> {
        let position = match self.index.get(key) {
            None => return Ok(None),
            Some(position) => *position,
        };

        let kv = self.get_at(position)?;

        Ok(Some(kv.value))
    }

    fn get_at(&mut self, position: u64) -> io::Result<KeyValuePair> {
        let mut file = BufReader::new(&mut self.file);
        file.seek(SeekFrom::Start(position))?;
        ActionKV::process_record(&mut file)
    }

    pub fn insert(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        let position = self.insert_without_index(key, value)?;
        self.index.insert(key.to_vec(), position);
        Ok(())
    }

    fn insert_without_index(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<u64> {
        let mut file = BufWriter::new(&mut self.file);
        let key_len = key.len();
        let val_len = value.len();
        let mut tmp_buffer = ByteString::with_capacity(key_len + val_len);

        for byte in key {
            tmp_buffer.push(*byte);
        }

        for byte in value {
            tmp_buffer.push(*byte);
        }

        let checksum = crc32::checksum_ieee(&tmp_buffer);

        let next_byte = SeekFrom::End(0);
        let current_position = file.stream_position()?;
        file.seek(next_byte)?;
        file.write_u32::<LittleEndian>(checksum)?;
        file.write_u32::<LittleEndian>(key_len as u32)?;
        file.write_u32::<LittleEndian>(val_len as u32)?;
        file.write_all(&tmp_buffer)?;

        Ok(current_position)
    }

    #[inline]
    pub fn update(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        self.insert(key, value)
    }

    #[inline]
    pub fn delete(&mut self, key: &ByteStr) -> io::Result<()> {
        self.insert(key, b"")
    }
}
