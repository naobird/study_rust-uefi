/*
 * main.rs
 */
#![feature(abi_efiapi)]
#![no_std]
#![no_main]

use uefi::prelude::*;
use uefi::table::boot::*;
use uefi::proto::loaded_image::LoadedImage;
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::proto::media::file::{File, FileMode, FileAttribute, FileHandle, RegularFile, Directory};
use core::panic::PanicInfo;
use core::fmt::Write;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub struct RegularFileWriter<'a>(&'a mut RegularFile);
impl<'a> RegularFileWriter<'a> {
    pub unsafe fn new(regular_file : &'a mut RegularFile) -> Self {
        Self(regular_file)
    }
}
impl<'a> core::fmt::Write for RegularFileWriter<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.0.write(s.as_bytes()).map_err(|_| core::fmt::Error)?.unwrap();
        Ok(())
    }
}

pub struct MemoryMap<T> {
    map_size : usize,
    memory_map_key : MemoryMapKey,
    descriptor_iter : T,
}

impl<'a, T> MemoryMap<T> {
    pub fn GetMemoryMap(map_size : usize, memory_map_info : (MemoryMapKey, T)) -> MemoryMap<T> 
        where T: ExactSizeIterator<Item = &'a MemoryDescriptor> + Clone
        {
            return MemoryMap {
                map_size : map_size,
                memory_map_key : memory_map_info.0,
                descriptor_iter : memory_map_info.1,
            };
        } 

    pub fn SaveMemoryMap(&self, memmap_file_writer : &mut RegularFileWriter) -> uefi::Status 
        where T: ExactSizeIterator<Item = &'a MemoryDescriptor> + Clone
        { 
            // save memory map to the target file
            writeln!(memmap_file_writer, "Index, Type, Type(name), PhysicalStart, NumberOfPages, Attribute").unwrap();

            // dump each memory info.
            let iter = self.descriptor_iter.clone();
            let mem_index : u32 = 0;
            for (i, desc) in iter.enumerate() {
                writeln!(memmap_file_writer, "{}, {:x}, {:?}, {:08x}, {}, {:x}", 
                         i, desc.ty.0, desc.ty, desc.phys_start, desc.page_count, desc.att).unwrap();
            }
            memmap_file_writer.0.flush().unwrap_success();
            return uefi::Status::SUCCESS;
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
    let loaded_image : *mut LoadedImage = boot_services.handle_protocol::<LoadedImage>(handle).unwrap_success().get();
    let device : Handle;
    unsafe {
        device = (*loaded_image).device();
    }
    let file_system : *mut SimpleFileSystem = boot_services.handle_protocol::<SimpleFileSystem>(device).unwrap_success().get();
    let mut root_dir : Directory;
    unsafe {
        root_dir = (*file_system).open_volume().unwrap_success();
    }

    // open the target file
    let memmap_file_handle : FileHandle = root_dir.open("\\memmap", FileMode::CreateReadWrite, FileAttribute::empty()).unwrap_success();
    let mut memmap_file : RegularFile;
    let mut memmap_file_writer : RegularFileWriter;
    unsafe {
        memmap_file = RegularFile::new(memmap_file_handle);
        memmap_file_writer = RegularFileWriter::new(&mut memmap_file);
    }
    memory_map.SaveMemoryMap(&mut memmap_file_writer);

    writeln!(system_table.stdout(), "Hello, world!").unwrap();

    loop {}

    return uefi::Status::SUCCESS;
}
