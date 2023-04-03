use std::{env, error::Error, process::Command};

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = env::args().nth(1).expect("usage: elk FILE");
    let input = std::fs::read(&input_path)?;
    let f = match delf::File::parse_or_print_error(&input[..]) {
        Some(f) => f,
        None => std::process::exit(1),
    };

    let code_ph = f
        .program_headers
        .iter()
        .find(|ph| ph.mem_range().contains(&f.entry_point))
        .expect("failed to fild code segment");

    use region::{protect, Protection};
    let code = &code_ph.data;
    unsafe {
        protect(code.as_ptr(), code.len(), Protection::READ_WRITE_EXECUTE)?;
    }
    let entry_offset = f.entry_point - code_ph.vaddr;
    let entry_point = unsafe {code.as_ptr().add(entry_offset.into())};
    println!("       code  @ {:?}", code.as_ptr());
    println!("entry offset @ {:?}", entry_offset);
    println!("entry point  @ {:?}", entry_point);
    // let code = &input[0x1000..];
    // let code = &code[..std::cmp::min(0x25, code.len())];
    // let entrypoint = code.as_ptr();
    unsafe {
        jmp(entry_point);
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
