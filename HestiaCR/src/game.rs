pub mod jvm;
pub mod offsets;

use std::{mem, ptr};
use winapi::shared::minwindef::{FALSE, LPCVOID, LPVOID};
use winapi::um::memoryapi::ReadProcessMemory;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winnt::{HANDLE, PROCESS_ALL_ACCESS};
use crate::memory::{get_module_address_by_name, get_process_pid_by_name};

pub struct GameProcess {
    pub hProcess:HANDLE,
    pub jvm_ptr:u64
}
pub trait Readable {
   fn read<typ>(&self,addr:u64) -> typ;
}
impl GameProcess {
    pub fn custom(process:&str) -> Option<GameProcess> {
        let pid =
            get_process_pid_by_name(process)?;
        let jvm =
            get_module_address_by_name(pid,"jvm.dll")?;
        unsafe {
            Some(GameProcess {
                hProcess: OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid),
                jvm_ptr: jvm,
            })
        }
    }
    pub fn craftrise() -> Option<GameProcess> {
        let pid =
            get_process_pid_by_name("craftrise-x64.exe")?;
        let jvm =
            get_module_address_by_name(pid,"jvm.dll")?;
        unsafe {
            Some(GameProcess {
                hProcess: OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid),
                jvm_ptr: jvm,
            })
        }
    }
}
impl Readable for GameProcess {
    fn read<typ>(&self,addr:u64) -> typ
    {
        unsafe {
            let mut buffer: typ = mem::zeroed();
            if ReadProcessMemory(self.hProcess,addr as LPCVOID,&mut buffer as *mut _ as LPVOID, mem::size_of_val(&buffer),ptr::null_mut()) == FALSE {
                panic!("Read fail");
            }
            buffer
        }
    }
}
