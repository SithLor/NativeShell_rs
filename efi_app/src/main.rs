#![no_main]
#![no_std]
//this line 37,func get_firmware_vendor,
//#![feature(vec_into_raw_parts)]
//#![feature(ascii_char)]

use core::ffi::c_void;

use log::info;
use uefi::table::cfg::ACPI_GUID;
use uefi::Guid;
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

struct efi_table {
    guid:[Guid; 11],
    address:[*const c_void; 11],
}
impl efi_table {
    fn new(_guid:[Guid; 11],_adress:[*const c_void; 11]) -> efi_table {
        efi_table {
            guid: _guid,
            address: _adress
        }
    }
}

fn create_efi_tabless(system_table: &SystemTable<Boot>) -> efi_table{
    let efi_table = system_table.config_table();
    let mut _c_voids: [*const c_void; 11] = [core::ptr::null();11];
    let mut _guids: [Guid; 11] = [ACPI_GUID;11];
    
    for (i, table) in efi_table.iter().enumerate() {
        info!("Table {}: GUID: {:#?} _c_voids: {:#?}", i, table.guid, table.address);
        _c_voids[i] = table.address;
        _guids[i] = table.guid;
    }
    let efi_table: efi_table = efi_table::new(_guids, _c_voids);

    return efi_table;
}


#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi::helpers::init(&mut system_table).unwrap();

    info!("Firmware Vendor: {}", system_table.firmware_vendor());
    info!("Firmware Revision: {}", get_firmware_revision(&system_table));
    info!("Firmware Revision: {}", get_firmware_revision_str(&system_table));
    info!("UEFI Revision: {:?}", get_uefi_revision(&system_table));

    let table: efi_table = create_efi_tabless(&system_table);

    //read f2fd1544-9794-4a2c-992e-e5bbcf20e394

    util_stall(&system_table, 10_000_000);
    util_shutdown(&system_table);
    return Status::SUCCESS;
}
