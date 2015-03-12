use core::prelude::*;
use core::num::from_str_radix;
use core::default::Default;

use collections::vec::Vec;
use collections::string::String;

use modules::Usart;

const NL: u8 = '\n' as u8;
const CR: u8 = '\r' as u8;
const BS: u8 = 8u8;


#[derive(Debug)]
pub enum Cmd {
    Read(u32),
    Write(u32),
    Unknown
}

pub fn get_command() -> Cmd {

    print_prompt();
    parse(get_line())

}

fn print_prompt() {
    let usart: Usart = Default::default();
    usart.write_line("> ");
}

fn get_line() -> String {

    let usart: Usart = Default::default();
    let mut line = String::with_capacity(8);

    loop {
        let ch = usart.getc();

        if ch == CR {
            usart.putc(NL);
            usart.putc(CR);
            break;
        } else if ch == BS {
            usart.putc(BS);
            usart.putc(BS);

            line.pop();
        } else {

            if ch < ' ' as u8 || ch > '~' as u8 {
                continue;
            }

            line.push(ch as char);

            usart.putc(ch);
        }
    }

    line

}

fn parse(line: String) -> Cmd {

    let tokens: Vec<&str> = line.as_slice().split(' ').collect();

    match tokens.as_slice() {
        ["w", num] => match from_str_radix::<u32>(num, 10) {
            Ok(num) => Cmd::Write(num),
            _ => Cmd::Unknown
        },
        ["r", num] => match from_str_radix::<u32>(num, 10) {
            Ok(num) => Cmd::Read(num),
            _ => Cmd::Unknown
        },
        _ => Cmd::Unknown
    }
}
