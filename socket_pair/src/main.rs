use std::{
    env,
    ffi::CString,
    fs::File,
    io::Read,
    os::unix::{
        net::UnixStream,
        prelude::{AsRawFd, FromRawFd, RawFd},
    },
    process,
};

use nix::{
    cmsg_space,
    fcntl::{fcntl, FcntlArg, FdFlag},
    sys::{
        socket::{
            recvmsg, sendmsg, socketpair, AddressFamily, ControlMessage, ControlMessageOwned,
            MsgFlags, SockFlag,
        },
        uio::IoVec,
        wait::waitpid,
    },
    unistd::{execv, fork},
};
// use sendfd::{RecvWithFd, SendWithFd};
fn main() {
    let pairs = socketpair(
        AddressFamily::Unix,
        nix::sys::socket::SockType::Stream,
        None,
        SockFlag::empty(),
    )
    .unwrap();
    println!("p1 {}, p2 {}", pairs.0, pairs.1);

    match unsafe { fork() } {
        Ok(nix::unistd::ForkResult::Child) => match unsafe { fork() } {
            Ok(nix::unistd::ForkResult::Child) => {
                let _ = unsafe { UnixStream::from_raw_fd(pairs.1) };
                // unistd::close(pairs.1);
                fcntl(pairs.0, FcntlArg::F_SETFD(FdFlag::empty())).unwrap();

                let current_exec = std::env::current_exe().unwrap();
                // let s = CString::new(current_exec.as_os_str().as_bytes());
                // let s = s.as_ref().unwrap();
                let mount_cmd = CString::new(
                    "/Library/Filesystems/macfuse.fs/Contents/Resources/mount_macfuse".as_bytes(),
                )
                .unwrap_or_default();
                env::set_var("_FUSE_COMMFD", format!("{}", pairs.0));
                env::set_var("_FUSE_COMMVERS", "2");
                env::set_var("_FUSE_DAEMON_PATH", current_exec.to_str().unwrap());
                env::set_var("_FUSE_CALL_BY_LIB", "1");
                let mut args = vec![];
                args.push("/Library/Filesystems/macfuse.fs/Contents/Resources/mount_macfuse");
                // args.push("-o");
                // args.push("allow_other");
                args.push("/Users/sean/code/github.com/xujihui1985/learningrust/socket_pair/mnt");

                let argv: Vec<_> = args
                    .into_iter()
                    .map(|f| CString::new(f.as_bytes()).unwrap_or_default())
                    .collect();
                execv(&mount_cmd, &argv).unwrap();
                std::process::exit(1);
            }
            Ok(nix::unistd::ForkResult::Parent { child: _ }) => {
                std::process::exit(0);
            }
            Err(_) => {}
        },
        Ok(nix::unistd::ForkResult::Parent { child }) => {
            let mut buf = cmsg_space!([RawFd; 2]);
            let mut b = vec![0_u8; 1];
            let iov = [IoVec::from_mut_slice(&mut b)];
            // let us = unsafe { UnixStream::from_raw_fd(pairs.1) };
            // let mut rawfds = [0; 1];
            println!("recv with fd...");
            // us.recv_with_fd(&mut b, &mut rawfds).unwrap();
            let res = recvmsg(pairs.1, &iov, Some(&mut buf), MsgFlags::empty()).unwrap();
            let revfd = match res.cmsgs().next() {
                Some(ControlMessageOwned::ScmRights(fd)) => Some(fd),
                Some(_) => None,
                None => None,
            };
            let fd = revfd.unwrap()[0];
            println!("got fd {}", fd);
            // let mut f = unsafe { File::from_raw_fd(fd) };
            // let mut str = String::new();
            // f.read_to_string(&mut str).unwrap();
            // println!("file content {}", str);
            waitpid(child, None).unwrap();
        }
        Err(_) => {
            let _ = unsafe { UnixStream::from_raw_fd(pairs.0) };
            let _ = unsafe { UnixStream::from_raw_fd(pairs.1) };
            // unistd::close(pairs.0);
            // unistd::close(pairs.1);
        }
    }
}
