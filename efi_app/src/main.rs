#![no_main]
#![no_std]
//this line 37,func get_firmware_vendor,
#![feature(vec_into_raw_parts)]
#![feature(ascii_char)]

use log::info;
use uefi::{
    prelude::*, table::runtime::{ResetType, Time, TimeCapabilities}, CString16, Guid
};

//https://rust-osdev.github.io/uefi-rs/HEAD/concepts/variables.html
//https://docs.rs/uefi/latest/uefi/table/runtime/struct.ResetType.html#associatedconstant.SHUTDOWN
//https://wiki.osdev.org/Main_Page



fn util_shutdown(system_table: &SystemTable<Boot>) {
    system_table
        .runtime_services()
        .reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
}
fn get_firmware_vendor(system_table: &SystemTable<Boot>) -> &str { 
    let _firmware_vendor: CString16 = CString16::from(system_table.firmware_vendor()).clone();

    if (_firmware_vendor.is_empty()){return "Unknown";}

    if _firmware_vendor.is_ascii(){
        let length = _firmware_vendor.to_u16_slice_with_nul().len();
        let mut final_data:&[&str] = &["";];
        //u16 slice with nul  to   


        return final_data
    } else {
        return "Unknown";
    }

 
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
#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi::helpers::init(&mut system_table).unwrap();

    info!("Firmware Vendor: {}", get_firmware_vendor(&system_table));

    util_shutdown(&system_table);
    return Status::SUCCESS;
}
