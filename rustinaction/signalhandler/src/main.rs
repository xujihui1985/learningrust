use std::{time::Duration, thread::sleep};

use libc::{SIGTERM, SIGUSR1, SIG_IGN, signal, raise, SIG_DFL};

static mut SHUT_DOWN: bool = false;

fn register_signal_handlers() {
    unsafe {libc::signal(SIGTERM, handle_sigterm as usize)};
    unsafe {libc::signal(SIGUSR1, handle_sigusr1 as usize)};
}

#[allow(dead_code)]
fn handle_sigterm(_signal: i32) {
    println!("got sigterm");

    unsafe {
        SHUT_DOWN = true;
    }
}

#[allow(dead_code)]
fn handle_sigusr1(_signal: i32) {
    println!("got sigusr1");
}

fn main() {
    unsafe {
        signal(SIGTERM, SIG_IGN);

        raise(SIGTERM);
    }
    println!("OK");

    unsafe {
        signal(SIGTERM, SIG_DFL);

        raise(SIGTERM);
    }
    println!("not OK");
    //register_signal_handlers();
     //let delay = Duration::from_secs(1);

     //for i in 1_usize.. {
        //println!("{}", i);
        //unsafe {
            //if SHUT_DOWN {
                //println!("*");
                //return;
            //}
        //}
        //sleep(delay);
        //let signal = if i > 2 {
            //SIGTERM
        //} else {
            //SIGUSR1
        //};

        //unsafe {
            //libc::raise(signal);
        //}
     //}
}
