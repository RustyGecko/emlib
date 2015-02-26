use core::fmt::Arguments;

#[lang = "stack_exhausted"]
pub extern fn stack_exhausted() {}

#[lang = "eh_personality"]
pub extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[allow(unused_variables)]
pub extern fn rust_begin_unwind(msg: Arguments, file: &'static str, line: usize) -> ! {
   loop {}
}
