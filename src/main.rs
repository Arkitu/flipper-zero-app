//! Template project for Flipper Zero.
//! This app prints "Hello, Rust!" to the console then exits.

#![no_main]
#![no_std]

// Required for panic handler
extern crate flipperzero_rt;

use core::ffi::CStr;
use flipperzero::{dialogs::{DialogMessage, DialogsApp}, gui::canvas::Align, println};
use flipperzero_rt::{entry, manifest};

// Define the FAP Manifest for this application
manifest!(
    name = "Flipper Zero Rust",
    app_version = 1,
    has_icon = true,
    // See https://github.com/flipperzero-rs/flipperzero/blob/v0.7.2/docs/icons.md for icon format
    icon = "rustacean-10x10.icon",
);

// Define the entry function
entry!(main);

// Entry point
fn main(_args: Option<&CStr>) -> i32 {
    println!("Hello, Rust!");

    let mut app = DialogsApp::open();
    let mut msg = DialogMessage::new();
    msg.set_text(c"test", 10, 10, Align::Center, Align::Center);
    app.show_message(&msg);

    0
}
