#![no_main]
#![no_std]
//this line 37,func get_firmware_vendor,
//#![feature(vec_into_raw_parts)]
//#![feature(ascii_char)]

//https://rust-osdev.github.io/uefi-rs/HEAD/concepts/variables.html
//https://docs.rs/uefi/latest/uefi/table/runtime/struct.ResetType.html#associatedconstant.SHUTDOWN
//https://wiki.osdev.org/Main_Page


use log::info;
use uefi::prelude::*;
use uefi::proto::device_path::text::{
    AllowShortcuts, DevicePathToText, DisplayOnly,
};
use uefi::proto::loaded_image::LoadedImage;
use uefi::table::boot::SearchType;
use uefi::{Identify, Result};

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi::helpers::init(&mut system_table).unwrap();
    let boot_services = system_table.boot_services();

    print_image_path(boot_services).unwrap();

    boot_services.stall(10_000_000);
    Status::SUCCESS
}

fn print_image_path(boot_services: &BootServices) -> Result {
    use uefi::table::boot::ScopedProtocol;
    let loaded_image: ScopedProtocol<LoadedImage> = boot_services
        .open_protocol_exclusive::<LoadedImage>(boot_services.image_handle())?;

    let device_path_to_text_handle = *boot_services
        .locate_handle_buffer(SearchType::ByProtocol(&DevicePathToText::GUID))?
        .first()
        .expect("DevicePathToText is missing");

    let device_path_to_text = boot_services
        .open_protocol_exclusive::<DevicePathToText>(
            device_path_to_text_handle,
        )?;

    let image_device_path =
        loaded_image.file_path().expect("File path is not set");
    let image_device_path_text = device_path_to_text
        .convert_device_path_to_text(
            boot_services,
            image_device_path,
            DisplayOnly(true),
            AllowShortcuts(false),
        )
        .expect("convert_device_path_to_text failed");

    info!("Image path: {}", &*image_device_path_text);
    Ok(())
}
