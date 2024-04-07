use region::{protect, Protection};
use std::{env, error::Error, process::Command};

use mmap::MemoryMap;

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = env::args().nth(1).expect("usage: elk FILE");
    let input = std::fs::read(&input_path)?;
    let f = match delf::File::parse_or_print_error(&input[..]) {
        Some(f) => f,
        None => std::process::exit(1),
    };

    println!("Dynamic entries:");
    if let Some(ds) = f
        .program_headers
        .iter()
        .find(|ph| ph.r#type == delf::SegmentType::Dynamic)
    {
        if let delf::SegmentContents::Dynamic(ref table) = ds.contents {
            for entry in table {
                println!(" - {:?}", entry);
            }
        }
    }

    let rela_entries = f.read_rela_entries()?;
    for e in rela_entries {
        println!("{:#?}", e);
        if let Some(seg) = file.segment_at(e.offset) {
            println!("... for {:#?}", seg);
        }
    }

    if let Some(dynseg) = f.segment_of_type(delf::SegmentType::Dynamic) {
        if let delf::SegmentContents::Dynamic(ref dyntab) = dynseg.contents {
            println!("dynamic table entries:");
            match e.tag {
                delf::DynamicTag::Needed | delf::DynamicTag::RPath => {
                    println!(" => {:?}", f.get_string(e.addr)?)
                },
                _ => {},
            }
        }
    }

    for sh in f.section_headers.iter() {
        println!("section header: {:?}", sh);
    }

    let syms = f.read_syms().unwrap();
    println!(
        "Symbol table @ {:?} contains {} entries",
        file.dynamic_entry(delf::DynamicTag::SymTab).unwrap(),
        syms.len()
    );
    println!(
        "  {:6}{:12}{:10}{:16}{:16}{:12}{:12}",
        "Num", "Value", "Size", "Type", "Bind", "Ndx", "Name"
    );
    for (num, s) in syms.iter().enumerate() {
        println!(
            "  {:6}{:12}{:10}{:16}{:16}{:12}{:12}",
            format!("{}", num),
            format!("{:?}", s.value),
            format!("{:?}", s.size),
            format!("{:?}", s.r#type),
            format!("{:?}", s.bind),
            format!("{:?}", s.shndx),
            format!("{}", file.get_string(s.name).unwrap_or_default()),
        );
    }


    let base = 0x400000_usize;

    let mut mapping = Vec::new();
    for ph in f
        .program_headers
        .iter()
        .filter(|ph| ph.r#type == delf::SegmentType::Load)
        .filter(|ph| ph.mem_range().end > ph.mem_range().start)
    {
        println!("mapping segment @ {:?} -> {:?}", ph.mem_range(), ph.flags);
        let mem_region = ph.mem_range();
        let len: usize = (mem_region.end - mem_region.start).into();

        let start = mem_region.start.0 as usize + base;
        let align_start = align_lo(start);
        let padding = start - align_start;
        let len = len + padding;

        let addr: *mut u8 = unsafe {std::mem::transmute(align_start)};
        let map = MemoryMap::new(
            len,
            &[mmap::MapOption::MapWritable, mmap::MapOption::MapAddr(addr)],
        );
        println!("copy segment data");
        unsafe {
            std::ptr::copy_nonoverlapping(ph.data.as_ptr(), addr.add(padding), len);
        }

        for reloc in &rela_entries {
            if mem_region.contains(&reloc.offset) {
                unsafe {
                    use std::mem::transmute as trans;
                    let real_segment_start = addr.add(padding);
                    let specified_reloc_offset = reloc.offset;
                    let specified_segment_start = mem_region.start;
                    let offset_into_segment = specified_reloc_offset = specified_segment_start;

                    println!("applying {:?} relocation @ {:?}", reloc.r#type, offset_into_segment);

                    let reloc_addr: *mut u64 = trans(real_segment_start.add(offset_into_segment.into()));
                    match reloc.r#type {
                        delf::RelType::Relative => {
                            let reloc_value = reloc.addent + delf::Addr(base as u64);
                            println!("replace value with {:?}", reloc_value);
                            *reloc_addr = reloc_value.0;
                        }
                        r#type => {
                            panic!("unsupported relocation type: {:?}", r#type);
                        }
                    }
                }
            }
        }

        // {
        //     let dst = unsafe { std::slice::from_raw_parts_mut(addr, ph.data.len()) };
        //     dst.copy_from_slice(&ph.data[..]);
        // }
        println!("adjuest permissions");
        let mut protection = Protection::NONE;
        for flag in ph.flags.iter() {
            protection |= match flag {
                delf::SegmentFlag::Read => Protection::READ,
                delf::SegmentFlag::Write => Protection::WRITE,
                delf::SegmentFlag::Execute => Protection::EXECUTE,
            }
        }
        unsafe {
            protect(addr, len, protection)?;
        }
        mapping.push(map)
    }

    
    // let code_ph = f
    //     .program_headers
    //     .iter()
    //     .find(|ph| ph.mem_range().contains(&f.entry_point))
    //     .expect("failed to fild code segment");

    // use region::{protect, Protection};
    // let code = &code_ph.data;
    // unsafe {
    //     protect(code.as_ptr(), code.len(), Protection::READ_WRITE_EXECUTE)?;
    // }
    // let entry_offset = f.entry_point - code_ph.vaddr;
    // let entry_point = unsafe {code.as_ptr().add(entry_offset.into())};
    // println!("       code  @ {:?}", code.as_ptr());
    // println!("entry offset @ {:?}", entry_offset);
    // println!("entry point  @ {:?}", entry_point);
    // let code = &input[0x1000..];
    // let code = &code[..std::cmp::min(0x25, code.len())];
    // let entrypoint = code.as_ptr();
    unsafe {
        // jmp(f.entry_point.0 as _);
        jum(std::mem::transmute(file.entry_point.0 as usize + base));
    }

    println!("input is supported elf {:#?}", f);
    let status = Command::new(input_path).status()?;
    if !status.success() {
        return Err("process not exit successfully".into());
    }
    Ok(())
}

unsafe fn jmp(addr: *const u8) {
    let fn_ptr: fn() = std::mem::transmute(addr);
    fn_ptr();
}

fn align_lo(x: usize) -> usize {
    x & !0xFFF
}
