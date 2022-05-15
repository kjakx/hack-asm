mod parser;
mod code;

use std::env;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fmt::Write as FmtWrite;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("usage: hack-assembler <filename>.asm"); }
    
    let fin_path = Path::new(&args[1]);
    let fin = File::open(fin_path)?;
    let mut p = parser::Parser::new(fin);

    let fout_path = fin_path.with_extension("hack");
    let fout = File::create(fout_path)?;
    let mut writer = BufWriter::<File>::new(fout);
    
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
                (0b111 << 13) | (c << 6) | (d << 3) | j
            },
            _ => unimplemented!()
        };
        let mut s = String::new();
        writeln!(&mut s, "{:016b}", bit_code);
        writer.write(s.as_bytes()).unwrap();
    }
    writer.flush().unwrap();
    Ok(())
}
