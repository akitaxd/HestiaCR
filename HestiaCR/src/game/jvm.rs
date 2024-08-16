use std::ptr::eq;
use crate::game::{GameProcess, Readable};
use crate::game::offsets::{JVM_COMPRESSED_CLASS_POINTERS_BASE, JVM_COMPRESSED_CLASS_POINTERS_SHIFT, JVM_COMPRESSED_OOPS_BASE, JVM_COMPRESSED_OOPS_SHIFT, JVM_CP_BASE, JVM_KLASS_CONSTANTS, JVM_KLASS_FIELDS, JVM_KLASS_FIELDS_COUNT, JVM_KLASS_INTERFACES, JVM_KLASS_JAVAMIRROR, JVM_KLASS_SUPER, JVM_SEED_SYMBOLTABLE, JVM_SYMBOLTABLE, JVM_SYSTEMCL, JVM_SYSTEMDICTIONARY, JVM_USE_COMPRESSED_CLASS_POINTERS, JVM_USE_COMPRESSED_OOPS};

pub fn hashcode(str: &str) -> u32 {
    let mut h = 0u32;
    for x in str.as_bytes() {
        h = h.wrapping_mul(31).wrapping_add(*x as u32);
    }
    h
}

pub trait JVM_Control {
    fn system_class_loader(&self) -> u64;
    fn find_symbol(&self,name:&str) -> u64;
    fn decode_oop(&self,oop:u32) -> u64;
    fn encode_oop(&self,oop:u64) -> u32;
    fn decode_klass(&self,oop:u32) -> u64;
    fn encode_klass(&self,oop:u64) -> u32;
    fn find_class_from_classloader(&self,name:&str,loader:u64) -> u64;
    fn find_class(&self,name:&str) -> u64;
    fn find_local_field(&self,klass:u64,namesym:u64,sigsym:u64) -> u16;
    fn find_interface_field(&self,klass:u64,namesym:u64,sigsym:u64) -> u16;
    fn find_field(&self,klass:u64,name_sym:u64,sig_sym:u64) -> u16;
    fn get_field_id(&self,klass:u64,name:&str,sig:&str) -> u16;
    fn get_static_object_field(&self,klass:u64,field_id:u16) -> u64;
    fn get_object_field(&self,oop:u64,field_id:u16) -> u64;
    fn get_static_value_field<typ>(&self,klass:u64,field_id:u16) -> typ;
    fn get_value_field<typ>(&self,klass:u64,field_id:u16) -> typ;


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
        let index:u64 = (hash as u64 % symbol_table_len);
        let mut listelem:u64 = self.read(hash_map+index*8);
        while listelem != 0 {
            let hashlem:u32 = self.read(listelem);
            if hashlem == hash as u32 {
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

    fn decode_oop(&self, oop: u32) -> u64 {
        let flag: bool = self.read(self.jvm_ptr + JVM_USE_COMPRESSED_OOPS);
        if flag {
            let base: u64 = self.read(self.jvm_ptr + JVM_COMPRESSED_OOPS_BASE);
            let shift: i32 = self.read(self.jvm_ptr + JVM_COMPRESSED_OOPS_SHIFT);
            return ((oop as u64) << (shift)) + base;
        }
        oop as u64
    }

    fn encode_oop(&self, oop: u64) -> u32 {
        let flag: bool = self.read(self.jvm_ptr + JVM_USE_COMPRESSED_OOPS);
        if flag {
            let base: u64 = self.read(self.jvm_ptr + JVM_COMPRESSED_OOPS_BASE);
            let shift: i32 = self.read(self.jvm_ptr + JVM_COMPRESSED_OOPS_SHIFT);
            return (oop + base) as u32 >> shift;
        }
        oop as u32
    }

    fn decode_klass(&self, klass: u32) -> u64 {
        let flag: bool = self.read(self.jvm_ptr + JVM_USE_COMPRESSED_CLASS_POINTERS);
        if flag {
            let base: u64 = self.read(self.jvm_ptr + JVM_COMPRESSED_CLASS_POINTERS_BASE);
            let shift: i32 = self.read(self.jvm_ptr + JVM_COMPRESSED_CLASS_POINTERS_SHIFT);
            return ((klass as u64) << shift) + base;
        }
        klass as u64
    }

    fn encode_klass(&self, klass: u64) -> u32 {
        let flag: bool = self.read(self.jvm_ptr + JVM_USE_COMPRESSED_CLASS_POINTERS);
        if flag {
            let base: u64 = self.read(self.jvm_ptr + JVM_COMPRESSED_CLASS_POINTERS_BASE);
            let shift: i32 = self.read(self.jvm_ptr + JVM_COMPRESSED_CLASS_POINTERS_SHIFT);
            return (klass + base) as u32 >> shift;
        }
        klass as u32
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

    fn find_local_field(&self, klass: u64, namesym: u64, sigsym: u64) -> u16 {
        let size_of_unsigned_short = size_of_val(&0u16) as u64;

        let constants:u64 = self.read(klass + JVM_KLASS_CONSTANTS);
        let mut fields:u64 = self.read(klass + JVM_KLASS_FIELDS);
        let field_count:u64 = self.read(klass + JVM_KLASS_FIELDS_COUNT);
        fields+=4;
        for i in 0..field_count {
            let field:u64 = (fields + i * 6 * size_of_unsigned_short);
            let name_id:u16 = self.read(field + size_of_unsigned_short);
            let sig_id:u16 = self.read(field + 2*size_of_unsigned_short);
            let name_ptr:u64 = self.read(constants + JVM_CP_BASE + (name_id as u64)*8);
            let sig_ptr:u64 = self.read(constants + JVM_CP_BASE + (sig_id as u64)*8);
            if namesym == name_ptr && sigsym == sig_ptr
            {
                let mut offset:u16 = self.read(field+4*size_of_unsigned_short);
                offset >>= 2;
                return offset;
            }

        }
        0
    }

    fn find_interface_field(&self, klass: u64, namesym: u64, sigsym: u64) -> u16 {
        let mut interfaces:u64 = self.read(klass + JVM_KLASS_INTERFACES);
        let mut offset:u16  = 0;
        let interfaces_size:i32 = self.read(interfaces);
        interfaces+=8;
        for i in 0..interfaces_size {
            let interface:u64 = self.read(interfaces + (i as u64) * 8);
            offset = self.find_local_field(interface,namesym,sigsym);
            if offset != 0 {
                return offset;
            }
            offset = self.find_interface_field(interface,namesym,sigsym);
            if offset != 0 {
                return offset;
            }
        }
        0
    }

    fn find_field(&self, klass: u64, name_symbol: u64, sig_symbol: u64) -> u16 {
        let mut offset:u16 = self.find_local_field(klass,name_symbol,sig_symbol);
        if offset == 0 {
            offset = self.find_interface_field(klass,name_symbol,sig_symbol);
            if offset == 0 {
                let super_klass:u64 = self.read(klass + JVM_KLASS_SUPER);
                if super_klass != 0 {
                    return self.find_field(super_klass,name_symbol,sig_symbol);
                }
            }
        }
        offset
    }

    fn get_field_id(&self, klass: u64, name: &str, sig: &str) -> u16 {
        let sig_symbol = self.find_symbol(sig);
        let name_symbol = self.find_symbol(name);
        let id = self.find_field(klass,name_symbol,sig_symbol);
        if id == 0 {
            panic!("Field Not Found {name} {sig}");
        }
        id
    }

    fn get_static_object_field(&self, klass: u64, field_id: u16) -> u64 {
        let class_oop:u64 = self.read(klass + JVM_KLASS_JAVAMIRROR);
        let value:u32 = self.read(class_oop + (field_id as u64));
        self.decode_oop(value)
    }

    fn get_object_field(&self, oop: u64, field_id: u16) -> u64 {
        let value:u32 = self.read(oop + (field_id as u64));
        self.decode_oop(value)
    }

    fn get_static_value_field<typ>(&self, klass: u64, field_id: u16) -> typ {
        let class_oop:u64 = self.read(klass + JVM_KLASS_JAVAMIRROR);
        self.read(class_oop + (field_id as u64))
    }

    fn get_value_field<typ>(&self, oop: u64, field_id: u16) -> typ {
        self.read(oop + (field_id as u64))
    }

}