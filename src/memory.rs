use winapi::um::psapi::{EnumProcessModules, GetModuleFileNameExW};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winnt::{PROCESS_ALL_ACCESS, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use winapi::shared::minwindef::{DWORD, FALSE, HMODULE, MAX_PATH, TRUE};
use std::ffi::OsString;
use std::mem::size_of;
use std::ptr;
use std::os::windows::ffi::OsStringExt;

const MODULE_LIST_SIZE: usize = 1024;

pub fn get_process_pid_by_name(name: &str) -> Option<DWORD> {
    unsafe {
        let mut processes = [0u32; MODULE_LIST_SIZE];
        let mut bytes_needed = 0u32;
        if winapi::um::psapi::EnumProcesses(processes.as_mut_ptr(), (size_of::<u32>() * MODULE_LIST_SIZE) as u32, &mut bytes_needed) == FALSE {
            return None;
        }
        let num_processes = bytes_needed as usize / size_of::<u32>();
        for i in 0..num_processes {
            let process_id = processes[i];
            let process_handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, process_id);
            if process_handle.is_null() {
                continue;
            }

            let mut process_name: [u16; MAX_PATH] = [0; MAX_PATH];
            let len = GetModuleFileNameExW(process_handle, ptr::null_mut(), process_name.as_mut_ptr(), MAX_PATH as u32);

            if len > 0 {
                let name_buffer = &process_name[..len as usize];
                let process_name = OsString::from_wide(name_buffer);
                let process_name = process_name.to_string_lossy();
                if process_name.ends_with(name) {
                    return Some(process_id);
                }
            }
        }
    }
    None
}

pub fn get_module_address_by_name(process_id: DWORD, module_name: &str) -> Option<u64> {
    unsafe {
        let process_handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, process_id);
        if process_handle.is_null() {
            return None;
        }

        let mut modules: [HMODULE; MODULE_LIST_SIZE] = [ptr::null_mut(); MODULE_LIST_SIZE];
        let mut bytes_needed = 0u32;

        if EnumProcessModules(
            process_handle,
            modules.as_mut_ptr(),
            (std::mem::size_of::<HMODULE>() * MODULE_LIST_SIZE) as u32,
            &mut bytes_needed,
        ) == FALSE
        {
            return None;
        }
        let num_modules = bytes_needed as usize / size_of::<HMODULE>();
        for i in 0..num_modules {
            let module_handle = modules[i];
            let mut module_file_name: [u16; MAX_PATH] = [0; MAX_PATH];
            let len = GetModuleFileNameExW(
                process_handle,
                module_handle,
                module_file_name.as_mut_ptr(),
                MAX_PATH as u32,
            );
            if len > 0 {
                let module_name_buffer = &module_file_name[..len as usize];
                let module_file_name = OsString::from_wide(module_name_buffer);
                let module_file_name = module_file_name.to_string_lossy();
                if module_file_name.contains(module_name) {
                    return Some(module_handle as u64);
                }
            }
        }
    }
    None
}


