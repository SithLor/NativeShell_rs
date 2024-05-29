#![no_main]
#![no_std]
//this line 37,func get_firmware_vendor,
//#![feature(vec_into_raw_parts)]
//#![feature(ascii_char)]

use core::ffi::c_void;
use core::mem::MaybeUninit;

use log::info;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::table::boot::{OpenProtocolParams, SearchType};
use uefi::table::cfg::ACPI_GUID;
use uefi::{Guid, Identify};
use uefi::{
    helpers::system_table, prelude::*, table::runtime::{ResetType, Time, TimeCapabilities}, CStr16, CString16
};

//https://rust-osdev.github.io/uefi-rs/HEAD/concepts/variables.html
//https://docs.rs/uefi/latest/uefi/table/runtime/struct.ResetType.html#associatedconstant.SHUTDOWN
//https://wiki.osdev.org/Main_Page


fn util_shutdown(system_table: &SystemTable<Boot>) {
    system_table
        .runtime_services()
        .reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
}
fn util_stall(system_table: &SystemTable<Boot>, time: usize) {
    system_table.boot_services().stall(time);
}

fn get_firmware_vendor(system_table: &SystemTable<Boot>) ->&CStr16 {
    return system_table.firmware_vendor();
}
fn get_firmware_revision(system_table: &SystemTable<Boot>) -> u32{
    let firmware_revision = system_table.firmware_revision();
    return firmware_revision;
}
fn get_firmware_revision_str(system_table: &SystemTable<Boot>) -> &str{
    let firmware_revision = system_table.firmware_revision();
    let g: Option<&str> = format_args!("{}.{}", firmware_revision >> 16, firmware_revision & 0xffff).as_str();
    match g {
        Some(g) => return g,
        None => "0.0",
    }
    
}
fn get_uefi_revision(system_table: &SystemTable<Boot>) -> uefi::table::Revision{
    system_table.uefi_revision()
}


use uefi::table::boot::OpenProtocolAttributes;

fn load_image(file:&str){
    
}

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi::helpers::init(&mut system_table).unwrap();

    info!("Firmware Vendor: {}", system_table.firmware_vendor());
    info!("Firmware Revision String: {}", get_firmware_revision(&system_table));
    info!("Firmware Revision: {}", get_firmware_revision_str(&system_table));
    info!("UEFI Revision: {:?}", get_uefi_revision(&system_table));

    let bs: &BootServices = system_table.boot_services();

    let gop_guid = uefi::proto::console::gop::GraphicsOutput::GUID;
    let mut handles = [MaybeUninit::<Handle>::uninit(); 1];
    let locate_handle_result = bs.locate_handle(SearchType::ByProtocol(&gop_guid), Some(&mut handles));
    
    //ping google https serice 
    


    system_table.boot_services().stall(10_000_000);
    Status::SUCCESS
}