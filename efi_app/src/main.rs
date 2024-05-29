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
use uefi::{
    helpers::system_table,
    prelude::*,
    table::runtime::{ResetType, Time, TimeCapabilities},
    CStr16, CString16,
};
use uefi::{Guid, Identify};

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

fn get_firmware_vendor(system_table: &SystemTable<Boot>) -> &CStr16 {
    return system_table.firmware_vendor();
}
fn get_firmware_revision(system_table: &SystemTable<Boot>) -> u32 {
    let firmware_revision = system_table.firmware_revision();
    return firmware_revision;
}
fn get_firmware_revision_str(system_table: &SystemTable<Boot>) -> &str {
    let firmware_revision = system_table.firmware_revision();
    let g: Option<&str> =
        format_args!("{}.{}", firmware_revision >> 16, firmware_revision & 0xffff).as_str();
    match g {
        Some(g) => return g,
        None => "0.0",
    }
}
fn get_uefi_revision(system_table: &SystemTable<Boot>) -> uefi::table::Revision {
    system_table.uefi_revision()
}

mod chip_black_and_white;

fn fps_to_nanoseconds(fps: u32) -> usize {
    return 1_000_000_000 / fps as usize;
}
#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi::helpers::init(&mut system_table).unwrap();

    //print to the console

    //loop until the user presses a key
    loop {
        for i in 1..=8 {
            match i {
                1 => info!("{}", chip_black_and_white::ASCII_ART_1),
                2 => info!("{}", chip_black_and_white::ASCII_ART_2),
                3 => info!("{}", chip_black_and_white::ASCII_ART_3),
                4 => info!("{}", chip_black_and_white::ASCII_ART_4),
                5 => info!("{}", chip_black_and_white::ASCII_ART_5),
                6 => info!("{}", chip_black_and_white::ASCII_ART_6),
                7 => info!("{}", chip_black_and_white::ASCII_ART_7),
                8 => info!("{}", chip_black_and_white::ASCII_ART_8),
                _ => (),
            }
            // Sleep for the remaining frame time
            system_table.boot_services().stall(100000);
            //clear the console
            let _ = system_table.stdout().reset(false);
        }
    }

    //system_table.boot_services().stall(10_000_000);
    Status::SUCCESS
}
