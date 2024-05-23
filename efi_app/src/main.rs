#![no_main]
#![no_std]

use uefi::{prelude::*, table::runtime::ResetType};
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
    //get cpu ghx
    let mut c = 0;
    while c < 10 {
        info!("Hello, UEFI!");
        if c == 5 {
            Shutdown(&system_table);
        }
        c += 1;
    }

    system_table.runtime_services().reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
}