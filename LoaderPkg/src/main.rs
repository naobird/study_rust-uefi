#![feature(abi_efiapi)]
#![no_std]
#![no_main]

use uefi::prelude::*;
use uefi::table::boot::*;
use core::panic::PanicInfo;
use core::fmt::Write;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub struct MemoryMap<T> {
    map_size : usize,
    memory_map_key : MemoryMapKey,
    descriptor_iter : T,
}

impl<T> MemoryMap<T> {
    pub fn GetMemoryMap(map_size : usize, memory_map_info : (MemoryMapKey, T)) -> MemoryMap<T> {
        return MemoryMap {
            map_size : map_size,
            memory_map_key : memory_map_info.0,
            descriptor_iter : memory_map_info.1,
        };
    } 
}

#[entry]
fn efi_main(handle: Handle, system_table: SystemTable<Boot>) -> Status {
    let buffer: &mut [u8] = &mut [0; 4096 * 4];
    let boot_services = system_table.boot_services();
    // get memory map
    let memory_map_info = boot_services.memory_map(buffer).unwrap_success();
    let memory_map = MemoryMap::GetMemoryMap(boot_services.memory_map_size(), memory_map_info);

    // open root dir and a file

    // save memory map to the file
    
    // close the file

    for descriptor in memory_map.descriptor_iter {
    }


    writeln!(system_table.stdout(), "Hello, world!").unwrap();

    loop {}
    //Status::SUCCESS
}
