#![no_std]

use core::cmp::min;
use core::convert::TryInto;
use core::sync::atomic::{AtomicBool, Ordering};

use serde::Serialize;

use linux_kernel_module::cstr;
use linux_kernel_module::sysctl::Sysctl;
use linux_kernel_module::Mode;

static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);
static C: AtomicBool = AtomicBool::new(false);


extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::string::String;

use linux_kernel_module::println;
use core::str;
struct HelloWorldModule {
    message: String,
}


#[derive(Serialize, Debug)]
struct Output {
    a: bool,
    b: bool,
    c: bool,
}

impl linux_kernel_module::KernelModule for HelloWorldModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {

        println!("Hello kernel json!");
        let o = Output {
            a: A.load(Ordering::Relaxed),
            b: B.load(Ordering::Relaxed),
            c: C.load(Ordering::Relaxed),
        };
        let s = serde_json_core::to_vec::<typenum::U32, _>(&o)
                .map_err(|_| linux_kernel_module::Error::ENOMEM)?;

        let mut s = str::from_utf8(&s)
            .map_err(|_| linux_kernel_module::Error::ENOMEM)?;
        let mut string = String::from(s);
        println!("123 {:?}", s);
        println!("123 {:?}", string);
        string.clear();

        println!("123 {:?}", string);

        Ok(HelloWorldModule {
            message: "on the heap!".to_owned(),
        })
    }
}

impl Drop for HelloWorldModule {
    fn drop(&mut self) {
        println!("My message is {}", self.message);
        println!("Goodbye kernel module!");
    }
}


linux_kernel_module::kernel_module!(
    HelloWorldModule,
    author: b"Fish in a Barrel Contributors",
    description: b"Use JSON serialization in kernelspace",
    license: b"GPL"
);
