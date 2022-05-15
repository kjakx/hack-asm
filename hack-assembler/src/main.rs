mod parser;
mod code;
mod symbol_table;

use std::env;
use std::fs::File;
use std::path::Path;
use std::io::BufWriter;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("usage: hack-assembler <filename>.asm"); }
    
    let fin_path = Path::new(&args[1]);
    let fin = File::open(fin_path)?;
    let mut p = parser::Parser::new(fin);

    let mut sym_table = symbol_table::SymbolTable::new();
    // make symbol table for L command
    let mut label_addr = 0;
    while p.has_more_commands() {
        p.advance();
        if p.command_type() == parser::CommandType::L {
            sym_table.add_entry(p.symbol(), label_addr);
        } else {
            label_addr += 1;
        }
    }

    let fin_path = Path::new(&args[1]);
    let fin = File::open(fin_path)?;
    let mut p = parser::Parser::new(fin);

    let fout_path = fin_path.with_extension("hack");
    let fout = File::create(fout_path)?;
    let mut writer = BufWriter::<File>::new(fout);
    
    // transforming .asm file into .hack machine code making symbol table for A command
    let mut symbol_addr = 16;
    while p.has_more_commands() {
        p.advance();
        let bit_code: u16 = match p.command_type() {
            parser::CommandType::A => {
                match p.symbol().parse::<u16>() {
                    Ok(n) => n,
                    Err(_) => {
                        if !sym_table.contains(p.symbol()) {
                            sym_table.add_entry(p.symbol(), symbol_addr);
                            symbol_addr += 1;
                        }
                        sym_table.get_address(p.symbol())
                    }
                }
            },
            parser::CommandType::C => {
                let c = code::comp(&p.comp());
                let d = code::dest(&p.dest());
                let j = code::jump(&p.jump());
                (0b111 << 13) | (c << 6) | (d << 3) | j
            },
            parser::CommandType::L => continue,
        };
        let mut w = Vec::new();
        writeln!(&mut w, "{:016b}", bit_code).unwrap();
        writer.write(&w).unwrap();
    }
    writer.flush().unwrap();
    Ok(())
}
