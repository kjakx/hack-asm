mod parser;
mod code;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("usage: hasm <filename>.asm"); }
    
    let f = File::open(&args[1])?;
    let mut p = parser::Parser::new(f);
    
    while p.has_more_commands() {
        p.advance();
        let bit_code: u16 = match p.command_type() {
            parser::CommandType::A => {
                p.symbol().parse::<u16>().unwrap()
            },
            parser::CommandType::C => {
                let c = code::comp(&p.comp());
                let d = code::dest(&p.dest());
                let j = code::jump(&p.jump());
                (0b111 << 13) | (c << 7) | (d << 3) | j
            },
            _ => unimplemented!()
        };
        println!("{:#018b}", bit_code);
    }
    Ok(())
}
