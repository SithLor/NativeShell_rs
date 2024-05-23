#![no_main]
#![no_std]

use uefi::{prelude::*, table::{runtime::{ResetType, Time, TimeCapabilities}}};
use log::info;

fn Shutdown(system_table: &SystemTable<Boot>){
    system_table.runtime_services().reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
}
//https://rust-osdev.github.io/uefi-rs/HEAD/concepts/variables.html
//https://docs.rs/uefi/latest/uefi/table/runtime/struct.ResetType.html#associatedconstant.SHUTDOWN
#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi::helpers::init(&mut system_table).unwrap();

    info!("Hello, UEFI!");
    


    let firmware_rev: u32 = system_table.firmware_revision();
    let uefi_rev: uefi::table::Revision = system_table.uefi_revision();


    let time: Result<(Time, TimeCapabilities), uefi::Error> = system_table.runtime_services().get_time_and_caps();
    match time {
        Ok((time, capabilities)) => {
            info!("Time: {}.{}.{} {}:{}:{}", time.year(), time.month(), time.day(), time.hour(), time.minute(), time.second());
            info!("Time capabilities: {:?}", capabilities);
        }
        Err(e) => {
            info!("Failed to get time: {:?}", e);
        }
    }
    info!("Firmware revision: {}.{}", firmware_rev >> 16, firmware_rev & 0xffff);
    info!("UEFI revision: {}.{}", uefi_rev.major(), uefi_rev.minor());

    let bios_info = uefi::proto::device_path::DevicePath:
    //let mut c = 0;
    //while c < 4 {
    //    if (c == 0){
    //        info!("CPU: {}", 1);
    //    }
    //    
    //}

    system_table.runtime_services().reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
}