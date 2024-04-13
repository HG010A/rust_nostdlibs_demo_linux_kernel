#![no_std]

extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::string::String;

use linux_kernel_module::{println};

struct HelloWorldModule {
    message: String,
}

use rhai::{Engine};
use rhai::{Dynamic, ImmutableString, INT};

impl linux_kernel_module::KernelModule for HelloWorldModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        println!("Hello kernel module!");

        let mut engine = Engine::new();
        println!("Hello kernel module rhai!");

        // Normal function that returns a standard type
        // Remember to use 'ImmutableString' and not 'String'
        fn add_len(x: i64, s: ImmutableString) -> i64 {
            x + s.len() as i64
        }
        // Alternatively, '&str' maps directly to 'ImmutableString'
        fn add_len_count(x: i64, s: &str, c: i64) -> i64 {
            x + s.len() as i64 * c
        }
        // Function that returns a 'Dynamic' value
        fn get_any_value() -> Dynamic {
            42_i64.into() // standard types can use '.into()'
        }
        fn get_pid() -> i64 {
            66 as i64
        }
        fn get_my_name() -> String {
            String::from("maosy")
        }

        engine
            .register_fn("add", add_len)
            .register_fn("add", add_len_count)
            .register_fn("add", get_any_value)
            .register_fn("get_pid", get_pid)
            .register_fn("get_my_name", get_my_name)
            .register_fn("inc", |x: i64| {
                // closure is also OK!
                x + 1
            });
        let result = engine
            .eval_expression::<INT>(r#"40 + 2 + add(get_pid(), get_my_name())"#)
            .unwrap() as isize;

        println!("Hello kernel module rhai! {}", result);

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
    description: b"An extremely simple kernel module",
    license: b"GPL"
);
