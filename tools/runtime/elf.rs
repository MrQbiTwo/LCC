// Bounded little-endian ELF64 decoding for the NASM runtime object.
use std::io;

pub const LIMIT: usize = 64 * 1024 * 1024;

// Preserve enough section metadata to validate symbols and relocation tables.
#[derive(Clone, Debug)]
pub struct Section {
    pub name: String,
    pub kind: u32,
    pub flags: u64,
    pub offset: usize,
    pub size: usize,
    pub link: usize,
    pub info: usize,
    pub align: usize,
    pub entry: usize,
}

// Retain validated section headers while referring to the caller-owned bytes.
pub struct Object<'a> {
    pub bytes: &'a [u8],
    pub sections: Vec<Section>,
}

// Report malformed object data with a stable contextual prefix.
pub fn bad(message: &str) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        format!("runtime object: {message}"),
    )
}

// Every object read uses checked ranges, including zero-length sections.
pub fn range(bytes: &[u8], offset: usize, size: usize) -> io::Result<&[u8]> {
    let end = offset
        .checked_add(size)
        .ok_or_else(|| bad("range overflow"))?;
    bytes
        .get(offset..end)
        .ok_or_else(|| bad("truncated section or record"))
}

// Decode a fixed-width integer only after checking the complete field.
pub fn u16_at(bytes: &[u8], offset: usize) -> io::Result<u16> {
    Ok(u16::from_le_bytes(
        range(bytes, offset, 2)?.try_into().unwrap(),
    ))
}
pub fn u32_at(bytes: &[u8], offset: usize) -> io::Result<u32> {
    Ok(u32::from_le_bytes(
        range(bytes, offset, 4)?.try_into().unwrap(),
    ))
}
pub fn u64_at(bytes: &[u8], offset: usize) -> io::Result<u64> {
    Ok(u64::from_le_bytes(
        range(bytes, offset, 8)?.try_into().unwrap(),
    ))
}

// Converting offsets separately keeps malformed host-width values explicit.
pub fn size_at(bytes: &[u8], offset: usize) -> io::Result<usize> {
    usize::try_from(u64_at(bytes, offset)?).map_err(|_| bad("offset exceeds host size"))
}

// Names must terminate inside their owning string table.
pub fn name_at(bytes: &[u8], offset: usize) -> io::Result<&str> {
    let tail = bytes
        .get(offset..)
        .ok_or_else(|| bad("string offset out of bounds"))?;
    let end = tail
        .iter()
        .position(|&byte| byte == 0)
        .ok_or_else(|| bad("unterminated name"))?;
    std::str::from_utf8(&tail[..end]).map_err(|_| bad("non-UTF-8 name"))
}

impl<'a> Object<'a> {
    // Accept only ordinary ELF64 little-endian x86-64 relocatable objects.
    pub fn parse(bytes: &'a [u8]) -> io::Result<Self> {
        if bytes.len() > LIMIT {
            return Err(bad("resource limit"));
        }
        if range(bytes, 0, 7)? != b"\x7fELF\x02\x01\x01"
            || u16_at(bytes, 16)? != 1
            || u16_at(bytes, 18)? != 62
            || u32_at(bytes, 20)? != 1
            || u16_at(bytes, 52)? != 64
            || u16_at(bytes, 58)? != 64
            || u64_at(bytes, 24)? != 0
            || u64_at(bytes, 32)? != 0
            || u16_at(bytes, 56)? != 0
        {
            return Err(bad("expected ELF64 x86-64 ET_REL without program headers"));
        }
        let table = size_at(bytes, 40)?;
        let count = usize::from(u16_at(bytes, 60)?);
        let strings = usize::from(u16_at(bytes, 62)?);
        if count == 0 || strings == 0 || strings >= count {
            return Err(bad("invalid section table"));
        }
        range(
            bytes,
            table,
            count.checked_mul(64).ok_or_else(|| bad("table overflow"))?,
        )?;
        let mut sections = Vec::with_capacity(count);
        for index in 0..count {
            let base = table + index * 64;
            let kind = u32_at(bytes, base + 4)?;
            let offset = size_at(bytes, base + 24)?;
            let size = size_at(bytes, base + 32)?;
            if size > LIMIT {
                return Err(bad("section resource limit"));
            }
            if kind != 8 {
                range(bytes, offset, size)?;
            }
            sections.push(Section {
                name: String::new(),
                kind,
                flags: u64_at(bytes, base + 8)?,
                offset,
                size,
                link: u32_at(bytes, base + 40)? as usize,
                info: u32_at(bytes, base + 44)? as usize,
                align: size_at(bytes, base + 48)?,
                entry: size_at(bytes, base + 56)?,
            });
        }
        if sections[0].kind != 0 || sections[strings].kind != 3 {
            return Err(bad("invalid section/string table"));
        }
        let names = range(bytes, sections[strings].offset, sections[strings].size)?;
        for (index, section) in sections.iter_mut().enumerate() {
            section.name = name_at(names, u32_at(bytes, table + index * 64)? as usize)?.into();
        }
        Ok(Self { bytes, sections })
    }

    // NOBITS storage is deliberately materialized by the packer instead.
    pub fn data(&self, index: usize) -> io::Result<&'a [u8]> {
        let section = self
            .sections
            .get(index)
            .ok_or_else(|| bad("section index out of bounds"))?;
        if section.kind == 8 {
            return Err(bad("NOBITS has no object bytes"));
        }
        range(self.bytes, section.offset, section.size)
    }
}
