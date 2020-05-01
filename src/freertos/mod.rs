//! # FreeRTOS for Rust / ESP32
//!
//! Rust interface for the FreeRTOS embedded operating system. Requires beta Rust.
//! It is assumed that dynamic memory allocation is provided on the target system.
//!
//! This library interfaces with FreeRTOS using a C shim library which provides function
//! wrappers for FreeRTOS macros. The compiled Rust application should be linked to the
//! base C/C++ firmware binary. Check the subdirectory ``shim``. Copy the source file to
//! your firmware's sources directory and modify it to include the appropriate headers for
//! target your system.
//!
//! For a complete example, check the enclosed GCC ARM/Rust/QEMU based unit tests. The project
//! ``qemu_runner`` cross-compiles this library, compiles the main firmware using GCC ARM and links
//! in the appropriate entry points for unit tests. [GNU ARM Eclipse QEMU](http://gnuarmeclipse.github.io/qemu/)
//! is used to run the test binaries.
//!
//! Be sure to check the [FreeRTOS documentation](http://www.freertos.org/RTOS.html).
//!
//! # Samples
//!
//! Spawning a new task
//!
//! ```rust
//! # use esp_idf_n_hal::freertos::*;
//! Task::new().name("hello").stack_size(128).start(|| {
//! 	loop {
//! 		println!("Hello world!");
//! 		CurrentTask::delay(Duration::infinite());
//! 	}
//! }).unwrap();
//! ```
//!
//! Queue
//!
//! ```rust
//! # use esp_idf_n_hal::freertos::{Queue, Duration};
//! let q = Queueu32::new(10).unwrap();
//! q.send(10, Duration::ms(5)).unwrap();
//! q.receive(Duration::infinite()).unwrap();
//! ```
//!
//! Mutex
//!
//! ```rust
//! # use esp_idf_n_hal::freertos::{Mutex, Duration};
//! let m = Mutex::new(0).unwrap();
//! {
//! 	let mut v = m.lock(Duration::infinite()).unwrap();
//! 	*v += 1;
//! }
//! ```

// #![no_std]
//
// #![cfg_attr(feature = "core_collections", feature(collections))]
//
// #[cfg_attr(feature = "default", macro_use)]
// extern crate alloc;
//
// #[cfg(feature = "core_collections")]
// #[macro_use]
// extern crate collections;

mod prelude;
mod shim;

mod base;
mod box_queue;
mod critical;
mod delays;
mod isr;
mod mutex;
mod queue;
mod semaphore;
mod task;
mod timers;
mod units;
mod utils;

pub mod patterns;

pub use base::FreeRtosError;
pub use box_queue::*;
pub use critical::*;
pub use delays::*;
pub use isr::*;
pub use mutex::*;
pub use queue::*;
pub use semaphore::*;
pub use task::*;
pub use timers::*;
pub use units::*;

pub use crate::freertos::utils::shim_sanity_check;
