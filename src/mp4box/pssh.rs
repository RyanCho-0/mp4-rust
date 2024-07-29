use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use serde::Serialize;
use std::io::{Read, Seek, Write};

use crate::mp4box::*;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct PsshBox {
    pub version: u8,
    pub flags: u32,
    pub system_id: [u8; 16],
    pub data_size: u32,
    pub data: Vec<u8>,
}

impl PsshBox {
    pub fn get_type(&self) -> BoxType {
        BoxType::PsshBox
    }

    pub fn get_size(&self) -> u64 {
        HEADER_SIZE + HEADER_EXT_SIZE + 20 + self.data.len() as u64 + 1
    }
}

impl Mp4Box for PsshBox {
    fn box_type(&self) -> BoxType {
        self.get_type()
    }

    fn box_size(&self) -> u64 {
        self.get_size()
    }

    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self).unwrap())
    }

    fn summary(&self) -> Result<String> {
        let s = format!("system_id={:?} ", self.system_id);
        Ok(s)
    }
}

impl<R: Read + Seek> ReadBox<&mut R> for PsshBox {
    fn read_box(reader: &mut R, size: u64) -> Result<Self> {
        // let start = box_start(reader)?;

        // let (version, flags) = read_box_header_ext(reader)?;

        // reader.read_u32::<BigEndian>()?; // pre-defined
        // let handler = reader.read_u32::<BigEndian>()?;

        // skip_bytes(reader, 12)?; // reserved

        // let buf_size = size
        //     .checked_sub(HEADER_SIZE + HEADER_EXT_SIZE + 20)
        //     .ok_or(Error::InvalidData("hdlr size too small"))?;

        // let mut buf = vec![0u8; buf_size as usize];
        // reader.read_exact(&mut buf)?;
        // if let Some(end) = buf.iter().position(|&b| b == b'\0') {
        //     buf.truncate(end);
        // }
        // let handler_string = String::from_utf8(buf).unwrap_or_default();

        // skip_bytes_to(reader, start + size)?;

        Ok(PsshBox {
            version: 0,
            flags: 0,
            data_size: 0,
            data: vec![],
            system_id: [0u8; 16],
        })
    }
}

impl<W: Write> WriteBox<&mut W> for PsshBox {
    fn write_box(&self, writer: &mut W) -> Result<u64> {
        let size = self.box_size();
        BoxHeader::new(self.box_type(), size).write(writer)?;

        write_box_header_ext(writer, self.version, self.flags)?;
        writer.write_all(&self.system_id)?;
        writer.write_u32::<BigEndian>(self.data_size)?;
        writer.write_all(&self.data)?;

        // writer.write_u32::<BigEndian>((&self.handler_type).into())?;

        Ok(size)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::mp4box::BoxHeader;
//     use std::io::Cursor;

//     #[test]
//     fn test_hdlr() {
//         let src_box = PsshBox {
//             version: 0,
//             flags: 0,
//             handler_type: str::parse::<FourCC>("vide").unwrap(),
//             name: String::from("VideoHandler"),
//         };
//         let mut buf = Vec::new();
//         src_box.write_box(&mut buf).unwrap();
//         assert_eq!(buf.len(), src_box.box_size() as usize);

//         let mut reader = Cursor::new(&buf);
//         let header = BoxHeader::read(&mut reader).unwrap();
//         assert_eq!(header.name, BoxType::PsshBox);
//         assert_eq!(src_box.box_size(), header.size);

//         let dst_box = PsshBox::read_box(&mut reader, header.size).unwrap();
//         assert_eq!(src_box, dst_box);
//     }

//     #[test]
//     fn test_hdlr_empty() {
//         let src_box = PsshBox {
//             version: 0,
//             flags: 0,
//             handler_type: str::parse::<FourCC>("vide").unwrap(),
//             name: String::new(),
//         };
//         let mut buf = Vec::new();
//         src_box.write_box(&mut buf).unwrap();
//         assert_eq!(buf.len(), src_box.box_size() as usize);

//         let mut reader = Cursor::new(&buf);
//         let header = BoxHeader::read(&mut reader).unwrap();
//         assert_eq!(header.name, BoxType::PsshBox);
//         assert_eq!(src_box.box_size(), header.size);

//         let dst_box = PsshBox::read_box(&mut reader, header.size).unwrap();
//         assert_eq!(src_box, dst_box);
//     }

//     #[test]
//     fn test_hdlr_extra() {
//         let real_src_box = PsshBox {
//             version: 0,
//             flags: 0,
//             handler_type: str::parse::<FourCC>("vide").unwrap(),
//             name: String::from("Good"),
//         };
//         let src_box = PsshBox {
//             version: 0,
//             flags: 0,
//             handler_type: str::parse::<FourCC>("vide").unwrap(),
//             name: String::from_utf8(b"Good\0Bad".to_vec()).unwrap(),
//         };
//         let mut buf = Vec::new();
//         src_box.write_box(&mut buf).unwrap();
//         assert_eq!(buf.len(), src_box.box_size() as usize);

//         let mut reader = Cursor::new(&buf);
//         let header = BoxHeader::read(&mut reader).unwrap();
//         assert_eq!(header.name, BoxType::PsshBox);
//         assert_eq!(src_box.box_size(), header.size);

//         let dst_box = PsshBox::read_box(&mut reader, header.size).unwrap();
//         assert_eq!(real_src_box, dst_box);
//     }
// }
