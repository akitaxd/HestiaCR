use crate::game::{GameProcess, Readable};
use crate::game::offsets::{JVM_COMPRESSED_CLASS_POINTERS_BASE, JVM_COMPRESSED_CLASS_POINTERS_SHIFT, JVM_COMPRESSED_OOPS_BASE, JVM_COMPRESSED_OOPS_SHIFT, JVM_SEED_SYMBOLTABLE, JVM_SYMBOLTABLE, JVM_SYSTEMCL, JVM_SYSTEMDICTIONARY, JVM_USE_COMPRESSED_CLASS_POINTERS, JVM_USE_COMPRESSED_OOPS};

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
    fn decode_oop(&self,oop:u64) -> u64;
    fn encode_oop(&self,oop:u64) -> u64;
    fn decode_klass(&self,oop:u64) -> u64;
    fn encode_klass(&self,oop:u64) -> u64;
    fn find_class_from_classloader(&self,name:&str,loader:u64) -> u64;

    fn find_class(&self,name:&str) -> u64;


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

    fn decode_oop(&self, oop: u64) -> u64 {
        let flag:bool = self.read(self.jvm_ptr+JVM_USE_COMPRESSED_OOPS);
        if flag {
            let base:u64 = self.read(self.jvm_ptr+JVM_COMPRESSED_OOPS_BASE);
            let shift:i64 = self.read(self.jvm_ptr+JVM_COMPRESSED_OOPS_SHIFT);
            return (oop << shift) + base;
        }
        oop
    }

    fn encode_oop(&self, oop: u64) -> u64 {
        let flag:bool = self.read(self.jvm_ptr+JVM_USE_COMPRESSED_OOPS);
        if flag {
            let base:u64 = self.read(self.jvm_ptr+JVM_COMPRESSED_OOPS_BASE);
            let shift:i64 = self.read(self.jvm_ptr+JVM_COMPRESSED_OOPS_SHIFT);
            return (oop - base) >> shift;
        }
        oop
    }

    fn decode_klass(&self, klass: u64) -> u64 {
        let flag:bool = self.read(self.jvm_ptr+JVM_USE_COMPRESSED_CLASS_POINTERS);
        if flag {
            let base:u64 = self.read(self.jvm_ptr+JVM_COMPRESSED_CLASS_POINTERS_BASE);
            let shift:i64  = self.read(self.jvm_ptr+JVM_COMPRESSED_CLASS_POINTERS_SHIFT);
            return (klass << shift) + base;
        }
        klass
    }

    fn encode_klass(&self, klass: u64) -> u64 {
        let flag:bool = self.read(self.jvm_ptr+JVM_USE_COMPRESSED_CLASS_POINTERS);
        if flag {
            let base:u64 = self.read(self.jvm_ptr+JVM_COMPRESSED_CLASS_POINTERS_BASE);
            let shift:i64  = self.read(self.jvm_ptr+JVM_COMPRESSED_CLASS_POINTERS_SHIFT);
            return (klass - shift as u64) >> base;
        }
        klass
    }

    fn find_class_from_classloader(&self, name: &str, loader: u64) -> u64 {
        let mut symbol = self.find_symbol(name);
        if symbol == 0 {
            return 0;
        }
        let mut hash:u32 = self.read(symbol+4);
        if loader != 0 {
            let loader_data:u64 = self.read(loader);
            hash ^= ((loader_data >> 8) & 0x7fffffff) as u32;
        }
        let system_dictionary:u64 = self.read(self.jvm_ptr + JVM_SYSTEMDICTIONARY);
        let system_dictionary_len:u64 = self.read(system_dictionary);
        if system_dictionary_len == 0 {
            panic!("System dictionary size must be greater than zero");
        }
        let dictionary_ht:u64 = self.read(system_dictionary+8);
        let index = hash as u64 % system_dictionary_len;
        let mut entry:u64 = self.read(dictionary_ht+index*8);
        while entry != 0 {
            let entry_hash:u64 = self.read(entry);
            if entry_hash == hash as u64{
                return self.read(entry+0x10);
            }
            entry = self.read(entry+8);
        }
        panic!("Class Not Found")
    }

    fn find_class(&self, name: &str) -> u64 {
        self.find_class_from_classloader(name,self.system_class_loader())
    }
}