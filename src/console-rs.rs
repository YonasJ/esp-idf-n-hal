//! Print debug information to console via UART0
//! Usage:
//! use esp32_hal::println;
//! use esp32_hal::console::Console;
//!  Console::begin(19200);
//!  println!("Hellgit");
//! This uses the serial module to do the writing.

use alloc::boxed::Box;
#[cfg(not(feature = "prefer-esp-idf"))]
use embedded_hal::serial::Write;
#[cfg(not(feature = "prefer-esp-idf"))]
use esp32_hal::dport::Split;
#[cfg(not(feature = "prefer-esp-idf"))]
use esp32_hal::serial::config::{DataBits, Parity, StopBits};
#[cfg(not(feature = "prefer-esp-idf"))]
use esp32_hal::serial::{config::Config, NoRx, NoTx, Rx, Serial, Tx};
#[cfg(feature = "prefer-esp-idf")]
use crate::esp_idf::{uart_write_bytes, uart_config_t, uart_word_length_t_UART_DATA_8_BITS, uart_parity_t_UART_PARITY_DISABLE, uart_stop_bits_t_UART_STOP_BITS_1, uart_hw_flowcontrol_t_UART_HW_FLOWCTRL_DISABLE, uart_config_t__bindgen_ty_1, uart_param_config, uart_set_pin, uart_driver_install, gpio_num_t_GPIO_NUM_1, gpio_num_t_GPIO_NUM_3, UART_PIN_NO_CHANGE, uart_flush};
#[cfg(feature = "prefer-esp-idf")]
use core::ptr;
#[cfg(feature = "prefer-esp-idf")]
use crate::esp_idf::std::os::raw::c_int;

pub struct Console {
    #[cfg(not(feature = "prefer-esp-idf"))]
    pub rx: Rx<esp32::UART0>,
    #[cfg(not(feature = "prefer-esp-idf"))]
    pub tx: Tx<esp32::UART0>,
}
/// Global instance to address the serial port, used by the console.
pub static mut CONSOLE: *mut Console = 0 as *mut Console;

// Only used when we are using the IDF functions to write to the console.
const UART_NUM:i32=0;

/// Used to help create a standard console for printout out debug messages to the default serial which
/// most dev board support through the USB port.
impl Console {
    /// Use the defaults for most boards, that also works with the ESP-IDF default baud rate. Short hand for `begin_custom(115200);`.
    pub fn begin() {
        Console::begin_custom(115200)
    }

    /// Use a custom boad rate.
    pub fn begin_custom(baud: u32) {
        unsafe {
            if CONSOLE == 0 as *mut Console {
                let mut console = Self::new(baud);
                CONSOLE = &mut *console;
            } else if cfg!(feature = "training-wheels") {
                panic!("Called Console.begin(), two times.");
            }
        }
    }
    #[cfg(not(feature = "prefer-esp-idf"))]
    fn new(baud: u32) -> Box<Console> {
        let dp = unsafe { esp32::Peripherals::steal() };

        let (mut dport, dport_clock_control) = dp.DPORT.split();

        let clkcntrl = esp32_hal::clock_control::ClockControl::new(
            dp.RTCCNTL,
            dp.APB_CTRL,
            dport_clock_control,
            esp32_hal::clock_control::XTAL_FREQUENCY_AUTO,
        )
            .unwrap();

        let (clkcntrl_config, _watchdog) = clkcntrl.freeze().unwrap();

        let serial = Serial::uart0(
            dp.UART0,
            (NoTx, NoRx),
            Config {
                baudrate: esp32_hal::units::Hertz(baud),
                data_bits: DataBits::DataBits8,
                parity: Parity::ParityNone,
                stop_bits: StopBits::STOP1,
            }, // default configuration is 19200 baud, 8 data bits, 1 stop bit & no parity (8N1)
            clkcntrl_config,
            &mut dport,
        )
            .unwrap();

        let (tx, rx) = serial.split();
        Box::new(Console { tx, rx })
    }
    #[cfg(feature = "prefer-esp-idf")]
    fn new(baud: u32) -> Box<Console> {
        /* Configure parameters of an UART driver,
     * communication pins and install the driver */
        let uart_config = uart_config_t {
            baud_rate: baud as c_int,
            data_bits: uart_word_length_t_UART_DATA_8_BITS,
            parity: uart_parity_t_UART_PARITY_DISABLE,
            stop_bits: uart_stop_bits_t_UART_STOP_BITS_1,
            flow_ctrl: uart_hw_flowcontrol_t_UART_HW_FLOWCTRL_DISABLE,
            rx_flow_ctrl_thresh: 0,
            __bindgen_anon_1: uart_config_t__bindgen_ty_1 {
                use_ref_tick: false,
            }
        };

        unsafe {
            const ECHO_TEST_TXD: i32 = gpio_num_t_GPIO_NUM_1 as i32;
            const ECHO_TEST_RXD: i32 = gpio_num_t_GPIO_NUM_3 as i32;
            const ECHO_TEST_RTS: i32 = UART_PIN_NO_CHANGE;
            const ECHO_TEST_CTS: i32 = UART_PIN_NO_CHANGE;
            const BUF_SIZE: i32 = 1024;
            uart_param_config(UART_NUM, &uart_config);
            uart_set_pin(UART_NUM, ECHO_TEST_TXD, ECHO_TEST_RXD, ECHO_TEST_RTS, ECHO_TEST_CTS);
            uart_driver_install(UART_NUM, BUF_SIZE * 2, 0, 0, ptr::null_mut(), 0);
        }

        Box::new(Console { })
    }

    #[cfg(feature = "prefer-esp-idf")]
    pub fn count() -> u8 {
        0
    }
    #[cfg(not(feature = "prefer-esp-idf"))]
    pub fn count() -> u8 {
        unsafe { (*CONSOLE).tx.count() }
    }

    #[cfg(feature = "prefer-esp-idf")]
    pub fn flush() -> nb::Result<(), core::convert::Infallible> {
        unsafe {
            uart_flush(UART_NUM);
        }
        Ok(())
    }
    #[cfg(not(feature = "prefer-esp-idf"))]
    pub fn flush() -> nb::Result<(), core::convert::Infallible> {
        unsafe {
            while (*CONSOLE).tx.count() > 0 {
                (*CONSOLE).tx.flush()?;
            }
        }
        Ok(())
    }
    #[cfg(feature = "prefer-esp-idf")]
    pub fn write(byte: u8) -> nb::Result<(), core::convert::Infallible> {
        unsafe {
            let b = [byte as i8];
            uart_write_bytes(UART_NUM, &b as *const _, 1);
            Ok(())
        }
    }
    #[cfg(not(feature = "prefer-esp-idf"))]
    pub fn write(byte: u8) -> nb::Result<(), core::convert::Infallible> {
        unsafe {
            (*CONSOLE).tx.write(byte)
        }
    }
}

impl core::fmt::Write for Console {

    #[cfg(feature = "prefer-esp-idf")]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            uart_write_bytes(UART_NUM, s.as_ptr() as *const _, s.len() as u32);
            Ok(())
        }
    }
    #[cfg(not(feature = "prefer-esp-idf"))]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            s.as_bytes()
                .iter()
                .try_for_each(|c| nb::block!(Console::write(*c)))
                .map_err(|_| core::fmt::Error)
        }
    }

}

