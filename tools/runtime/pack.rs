// Fold allocated runtime sections and canonicalize relocation and export records.
use super::elf::{self, bad, name_at, size_at, u16_at, u32_at, u64_at, Object, LIMIT};
use std::{fmt::Write, io};

// A folded section has one compiler segment and a byte offset within its blob.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Location {
    segment: usize,
    offset: usize,
}

// Relocations use compiler patch-end-relative PC32 semantics.
#[derive(Debug)]
struct Relocation {
    source: Location,
    target: Location,
    width: usize,
    addend: i64,
}

// Bound alignment growth before allocating or extending either embedded blob.
fn append(blob: &mut Vec<u8>, object: &Object<'_>, index: usize) -> io::Result<usize> {
    let section = &object.sections[index];
    let alignment = section.align.max(1);
    if !alignment.is_power_of_two() || alignment > 4096 {
        return Err(bad("unsupported section alignment"));
    }
    let offset = (blob.len() + alignment - 1) & !(alignment - 1);
    let end = offset
        .checked_add(section.size)
        .filter(|&end| end <= LIMIT)
        .ok_or_else(|| bad("blob resource limit"))?;
    blob.resize(end, 0);
    if section.kind != 8 {
        blob[offset..end].copy_from_slice(object.data(index)?);
    }
    Ok(offset)
}

// Validate and flatten sections, exports and relocations into one NASM include.
pub fn encode(bytes: &[u8]) -> io::Result<Vec<u8>> {
    let object = Object::parse(bytes)?;
    let mut blobs = [Vec::new(), Vec::new()];
    let mut alignments = [1, 1];
    let mut locations = vec![None; object.sections.len()];
    for (index, section) in object.sections.iter().enumerate() {
        if section.flags & 2 == 0 {
            continue;
        }
        let segment = match (section.name.as_str(), section.kind, section.flags & !2) {
            (".text", 1, 4) => 1,
            (".rodata", 1, 0) | (".data", 1, 1) | (".bss", 8, 1) => 2,
            _ => return Err(bad("unsupported allocated section")),
        };
        let offset = append(&mut blobs[segment - 1], &object, index)?;
        alignments[segment - 1] = alignments[segment - 1].max(section.align.max(1));
        locations[index] = Some(Location { segment, offset });
    }
    let tables: Vec<_> = object
        .sections
        .iter()
        .enumerate()
        .filter(|(_, s)| s.kind == 2)
        .collect();
    if tables.len() != 1 {
        return Err(bad("expected one symbol table"));
    }
    let (table_index, table) = tables[0];
    if table.entry != 24
        || table.size % 24 != 0
        || table.info > table.size / 24
        || object.sections.get(table.link).is_none_or(|s| s.kind != 3)
    {
        return Err(bad("invalid symbol table"));
    }
    let names = object.data(table.link)?;
    let symbols = object.data(table_index)?;
    let exports = [
        "lcc_rt_printf",
        "lcc_rt_scanf",
        "lcc_rt_getchar",
        "lcc_rt_exit",
    ];
    let mut exported = [None; 4];
    let mut targets = Vec::with_capacity(symbols.len() / 24);
    for (index, symbol) in symbols.chunks_exact(24).enumerate() {
        if index == 0 && symbol.iter().any(|&byte| byte != 0) {
            return Err(bad("invalid null symbol"));
        }
        let name = name_at(names, u32_at(symbol, 0)? as usize)?;
        let section = u16_at(symbol, 6)? as usize;
        let value = size_at(symbol, 8)?;
        let size = size_at(symbol, 16)?;
        let location = if section == 0 {
            if index != 0 || symbol.iter().any(|&byte| byte != 0) {
                return Err(bad("undefined runtime symbol"));
            }
            None
        } else if section == 0xfff1 {
            // NASM file-name and absolute metadata symbols cannot be relocation targets.
            None
        } else {
            let header = object
                .sections
                .get(section)
                .ok_or_else(|| bad("invalid symbol section"))?;
            if value > header.size || size > header.size - value {
                return Err(bad("symbol exceeds section"));
            }
            locations[section].map(|location| Location {
                segment: location.segment,
                offset: location.offset + value,
            })
        };
        if let Some(export) = exports.iter().position(|&export| export == name) {
            let location = location
                .filter(|location| location.segment == 1)
                .ok_or_else(|| bad("export is not code"))?;
            if value == object.sections[section].size || !matches!(symbol[4] & 15, 0 | 2) {
                return Err(bad("export is not a code entry"));
            }
            if symbol[4] >> 4 != 1 || exported[export].replace(location.offset).is_some() {
                return Err(bad("duplicate or non-global export"));
            }
        }
        targets.push(location);
    }
    if exported.iter().any(Option::is_none) {
        return Err(bad("missing required runtime export"));
    }
    let mut relocations = Vec::new();
    for (index, section) in object.sections.iter().enumerate() {
        if section.kind == 9 {
            return Err(bad("REL relocations are unsupported"));
        }
        if section.kind != 4 {
            continue;
        }
        if section.entry != 24 || section.size % 24 != 0 || section.link != table_index {
            return Err(bad("invalid relocation table"));
        }
        let source = locations
            .get(section.info)
            .copied()
            .flatten()
            .ok_or_else(|| bad("relocation source is not allocated"))?;
        let source_size = object.sections[section.info].size;
        for record in object.data(index)?.chunks_exact(24) {
            let offset = size_at(record, 0)?;
            let info = u64_at(record, 8)?;
            let width = match info as u32 {
                1 => 8,
                2 => 4,
                _ => return Err(bad("unsupported relocation type")),
            };
            if offset > source_size || width > source_size - offset {
                return Err(bad("relocation patch exceeds section"));
            }
            let target = targets
                .get((info >> 32) as usize)
                .copied()
                .flatten()
                .ok_or_else(|| bad("invalid relocation target"))?;
            let addend = u64_at(record, 16)? as i64;
            let addend = if width == 4 {
                addend
                    .checked_add(4)
                    .ok_or_else(|| bad("relocation addend overflow"))?
            } else {
                addend
            };
            if addend.checked_add(target.offset as i64).is_none() {
                return Err(bad("relocation target overflow"));
            }
            relocations.push(Relocation {
                source: Location {
                    segment: source.segment,
                    offset: source.offset + offset,
                },
                target,
                width,
                addend,
            });
        }
    }
    relocations.sort_by_key(|r| (r.source.segment, r.source.offset));
    for pair in relocations.windows(2) {
        if pair[0].source.segment == pair[1].source.segment
            && pair[0].source.offset + pair[0].width > pair[1].source.offset
        {
            return Err(bad("overlapping relocation patches"));
        }
    }
    let mut result = String::from(
        "; Generated runtime bytes and checked relocation records.\nsection .rodata\n",
    );
    for (index, name) in ["code", "data"].iter().enumerate() {
        writeln!(result, "%define runtime_{name}_size {}\n%define runtime_{name}_alignment {}\nruntime_{name}_bytes:", blobs[index].len(), alignments[index]).unwrap();
        for row in blobs[index].chunks(24) {
            result.push_str("    db ");
            for (index, byte) in row.iter().enumerate() {
                if index != 0 {
                    result.push(',');
                }
                write!(result, "{byte}").unwrap();
            }
            result.push('\n');
        }
    }
    for (name, offset) in ["printf", "scanf", "getchar", "exit"].iter().zip(exported) {
        writeln!(result, "%define runtime_{name}_offset {}", offset.unwrap()).unwrap();
    }
    writeln!(
        result,
        "%define runtime_relocation_count {}\nalign 8\nruntime_relocations:",
        relocations.len()
    )
    .unwrap();
    for relocation in relocations {
        writeln!(
            result,
            "    dq {},{},{},{},{},{}",
            relocation.source.segment,
            relocation.source.offset,
            relocation.target.segment,
            relocation.target.offset,
            relocation.width,
            relocation.addend
        )
        .unwrap();
    }
    if result.len() > elf::LIMIT * 4 {
        return Err(bad("generated include resource limit"));
    }
    Ok(result.into_bytes())
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
