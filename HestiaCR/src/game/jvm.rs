use crate::game::{GameProcess, Readable};
use crate::game::offsets::{JVM_SEED_SYMBOLTABLE, JVM_SYMBOLTABLE, JVM_SYSTEMCL};

pub fn hashcode(str: &str) -> u64 {
    let mut h = 0u32;
    for x in str.as_bytes() {
        h = h.wrapping_mul(31).wrapping_add(*x as u32);
    }
    h as u64
}

pub trait JVM_Control {
    fn system_class_loader(&self) -> u64;
    fn find_symbol(&self,name:&str) -> u64;
}
impl JVM_Control for GameProcess {
    fn system_class_loader(&self) -> u64 {
        self.read(self.jvm_ptr + JVM_SYSTEMCL)
    }

    fn find_symbol(&self, name: &str) -> u64 {
        let seed:u64 = self.read(self.jvm_ptr+JVM_SEED_SYMBOLTABLE);
        if seed != 0 {
            panic!("Seed should be zero")
        }
        let hash = hashcode(name);
        let symbol_table:u64 = self.read(self.jvm_ptr+JVM_SYMBOLTABLE);
        let symbol_table_len:u64 = self.read(symbol_table);
        let hash_map:u64 = self.read(symbol_table+8);
        println!("{symbol_table_len}");
        if symbol_table_len == 0 {
            panic!("Symbol table size must be greater than zero")
        }
        let index:u64 = (hash % symbol_table_len);
        let mut listelem:u64 = self.read(hash_map+index*8);
        while listelem != 0 {
            let hashlem:u64 = self.read(listelem);
            if hashlem == hash {
                let symbol:u64 = self.read(listelem+0x10);
                let symbol_len:u16 = self.read(symbol);
                if symbol_len == name.len() as u16
                {
                    let mut buffer = Vec::new();
                    let string_pointer:u64 = symbol+8;
                    for index in 0..symbol_len {
                        let byte:u8 = self.read(string_pointer + (index as u64));
                        buffer.push(byte);
                    }
                    let wrapper = String::from_utf8(buffer).unwrap();
                    if wrapper.eq(name) {
                        return symbol;
                    }

                }
            }
            listelem = self.read(listelem + 8);
        }
        panic!("Failed to find symbol {name}")
    }
}