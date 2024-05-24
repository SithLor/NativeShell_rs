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

fn ascii_char_to_single_str(i:u16) -> &'static str{
    if i < 32 || i > 126 {
        return " ";
    }
    let mut final_data: &str;
    match i {
        32 => final_data = " ",
        33 => final_data = "!",
        34 => final_data = "\"",
        35 => final_data = "#",
        36 => final_data = "$",
        37 => final_data = "%",
        38 => final_data = "&",
        39 => final_data = "'",
        40 => final_data = "(",
        41 => final_data = ")",
        42 => final_data = "*",
        43 => final_data = "+",
        44 => final_data = ",",
        45 => final_data = "-",
        46 => final_data = ".",
        47 => final_data = "/",
        48 => final_data = "0",
        49 => final_data = "1",
        50 => final_data = "2",
        51 => final_data = "3",
        52 => final_data = "4",
        53 => final_data = "5",
        54 => final_data = "6",
        55 => final_data = "7",
        56 => final_data = "8",
        57 => final_data = "9",
        58 => final_data = ":",
        59 => final_data = ";",
        60 => final_data = "<",
        61 => final_data = "=",
        62 => final_data = ">",
        63 => final_data = "?",
        64 => final_data = "@",
        65 => final_data = "A",
        66 => final_data = "B",
        67 => final_data = "C",
        68 => final_data = "D",
        69 => final_data = "E",
        70 => final_data = "F",
        71 => final_data = "G",
        72 => final_data = "H",
        73 => final_data = "I",
        74 => final_data = "J",
        75 => final_data = "K",
        76 => final_data = "L",
        77 => final_data = "M",
        78 => final_data = "N",
        79 => final_data = "O",
        80 => final_data = "P",
        81 => final_data = "Q",
        82 => final_data = "R",
        83 => final_data = "S",
        84 => final_data = "T",
        85 => final_data = "U",
        86 => final_data = "V",
        87 => final_data = "W",
        88 => final_data = "X",
        89 => final_data = "Y",
        90 => final_data = "Z",
        91 => final_data = "[",
        92 => final_data = "\\",
        93 => final_data = "]",
        94 => final_data = "^",
        95 => final_data = "_",
        96 => final_data = "`",
        97 => final_data = "a",
        98 => final_data = "b",
        99 => final_data = "c",
        100 => final_data = "d",
        101 => final_data = "e",
        102 => final_data = "f",
        103 => final_data = "g",
        104 => final_data = "h",
        105 => final_data = "i",
        106 => final_data = "j",
        107 => final_data = "k",
        108 => final_data = "l",
        109 => final_data = "m",
        110 => final_data = "n",
        111 => final_data = "o",
        112 => final_data = "p",
        113 => final_data = "q",
        114 => final_data = "r",
        115 => final_data = "s",
        116 => final_data = "t",
        117 => final_data = "u",
        118 => final_data = "v",
        119 => final_data = "w",
        120 => final_data = "x",
        121 => final_data = "y",
        122 => final_data = "z",
        123 => final_data = "{",
        124 => final_data = "|",
        125 => final_data = "}",
        126 => final_data = "~",
        _ => final_data[i] = "Unknown",
    }
    return final_data;    
}
macro_rules! copy_str_array_to_str {
    ($arr:expr, $sep:expr) => {{
        let mut arr_out = [""; 20];
        for i in 0..$arr.len() {
            arr_out[i] = $arr[i];
        }
        //let final_str:&str = arr_out[0..$arr.len()].join($sep);
        let final_data: &str = arr_out.join($sep).as_str();
        final_str
    }};
}

macro_rules! u16_slice_with_nul_to_str_arry {
    ($u16_slice_wth_nul:expr,$target_arr:expr,$target_arr_len:expr,) => {{

    }};
}

fn util_shutdown(system_table: &SystemTable<Boot>) {
    system_table
        .runtime_services()
        .reset(ResetType::SHUTDOWN, Status::SUCCESS, None);
}
fn get_firmware_vendor(system_table: &SystemTable<Boot>) -> &str {
    //clone the firmware vendor CString16 so rust can drop the value after the function is done
    let _firmware_vendor: CString16 = CString16::from(system_table.firmware_vendor()).clone();
    //check if the firmware vendor is empty
    if _firmware_vendor.is_empty(){return "Unknown";}
    //check if the firmware vendor is ascii
    if _firmware_vendor.is_ascii(){
        //this hellish code

        //get len for the for loop
        //create a mutable array of &str
        //iterate over the CString16 that get convert to u16 and then 
        //pass in the ascii_char_to_single_str func tranlaste num ascii to single str set it 
        // final_data[] array  
        let length = _firmware_vendor.to_u16_slice_with_nul().len();
        let mut final_data: &mut [&str; 20] = &mut ["";20]; // Change the type to &mut [&str;20]
        for i in 0..length {
            let mut data: u16 = _firmware_vendor.to_u16_slice_with_nul()[i];

            final_data[i] = ascii_char_to_single_str(data);
        }

        // create temp dir called __f
        // iterate over the final_data array and set the value to __f to so i dont have to deal with the temparry value in cpu stack error
        // join the array and convert it to a string and return it
        let mut __f = [""; 20];
        for i in 0..length {
            __f[i] = final_data[i];
        }
        let mut final_str: &str = __f.join("").as_str();

        //if i were just did this return final_data.join("").as_str(); it would have return a error
 //       error[E0515]: cannot return value referencing temporary value
//  --> src/main.rs:159:16
//    |
//159 |         return final_data.join("").as_str();
//    |                -------------------^^^^^^^^^
//    |                |
//    |                returns a value referencing data owned by the current function
//    |                temporary value created here
        return final_str;
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
