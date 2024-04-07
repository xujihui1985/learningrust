#![feature(maybe_uninit_slice)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[non_exhaustive] // prevent the call to initlize the struct without call the construct
#[derive(Clone, Debug)]
pub struct Sodium;

impl Sodium {
    pub fn new() -> Result<Self, ()> {
        if unsafe {ffi::sodium_init()} < 0 {
            Err(())
        } else {
            Ok(Self)
        }
    }

    pub fn crypto_generichash<'a>(
        &self, 
        input: &[u8], 
        key: Option<&[u8]>, 
        out: &'a mut [MaybeUninit<u8>],
    ) -> Result<&'a mut[u8], ()> {
        assert!(out.len() >= usize::try_from(unsafe{ffi::crypto_generichash_bytes_min()}).unwrap());
        assert!(out.len() <= usize::try_from(unsafe{ffi::crypto_generichash_bytes_max()}).unwrap());
        if let Some(key) = key {
            assert!(key.len() >= usize::try_from(unsafe{ffi::crypto_generichash_keybytes_min()}).unwrap());
            assert!(key.len() <= usize::try_from(unsafe{ffi::crypto_generichash_keybytes_max()}).unwrap());
        }
        let (key, keylen) = if let Some(key) = key {
            (key.as_ptr(), key.len())
        } else {
            (std::ptr::null(), 0)
        };
        // SAFETY: xxxx
        let res = unsafe {
            ffi::crypto_generichash(
                //out.as_mut_ptr() as *mut u8, 
                MaybeUninit::slice_as_mut_ptr(out),
                out.len() as u64, 
                input.as_ptr(), 
                input.len() as u64, 
                key, 
                keylen as u64,
            )
        };
        if res < 0 {
            return Err(());
        }
        Ok(unsafe{MaybeUninit::slice_assume_init_mut(out)})
    }
}

use std::mem::MaybeUninit;

pub use ffi::sodium_init;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_hash() {
        let s = Sodium::new().unwrap();
        let mut out = vec![MaybeUninit::uninit(); unsafe{ffi::crypto_generichash_bytes()} as usize];
        let res = s.crypto_generichash(b"test string", None, &mut out).unwrap();
        println!("11111111111 {:?}", res);
    }
}
