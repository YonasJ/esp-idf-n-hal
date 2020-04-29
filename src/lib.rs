#![no_std]
extern crate esp_idf_alloc;

extern crate alloc;

#[macro_use]
pub mod console;

pub mod esp_idf;

pub mod watchdog_disabler;
