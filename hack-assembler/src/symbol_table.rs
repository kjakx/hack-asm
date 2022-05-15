use std::collections::HashMap;

pub struct SymbolTable(HashMap<String, u16>);

impl SymbolTable {
    pub fn new() -> Self {
        let mut symbol_table = SymbolTable(HashMap::new());
        let registered_symbols = [
            ("SP"    ,     0),
            ("LCL"   ,     1),
            ("ARG"   ,     2),
            ("THIS"  ,     3),
            ("THAT"  ,     4),
            ("R0"    ,     0),
            ("R1"    ,     1),
            ("R2"    ,     2),
            ("R3"    ,     3),
            ("R4"    ,     4),
            ("R5"    ,     5),
            ("R6"    ,     6),
            ("R7"    ,     7),
            ("R8"    ,     8),
            ("R9"    ,     9),
            ("R10"   ,    10),
            ("R11"   ,    11),
            ("R12"   ,    12),
            ("R13"   ,    13),
            ("R14"   ,    14),
            ("R15"   ,    15),
            ("SCREEN", 16384),
            ("KBD"   , 24576),
        ];
        for (sym, addr) in registered_symbols.into_iter() {
            symbol_table.0.insert(sym.to_string(), addr);
        }
        symbol_table
    }

    pub fn add_entry(&mut self, symbol: String, address: u16) {
        self.0.insert(symbol, address);
    }

    pub fn contains(&self, symbol: String) -> bool {
        self.0.contains_key(&symbol)
    }

    pub fn get_address(&self, symbol: String) -> u16 {
        self.0[&symbol]
    }
}