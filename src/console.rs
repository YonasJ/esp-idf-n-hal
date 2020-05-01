#[cfg(not(feature = "prefer-esp-idf"))]
include!("console-rs.rs");
#[cfg(feature = "prefer-esp-idf")]
include!("console-idf.rs");

/// Macro for sending a formatted string to console (via UART0) for debugging
#[macro_export]
macro_rules! print {
    ($s:expr) => {
        unsafe {
            if $crate::console::CONSOLE != 0 as *mut $crate::console::Console {
                use core::fmt::Write;
                write!((*$crate::console::CONSOLE), $s).unwrap();
            } else if cfg!(feature = "training-wheels") {
                panic!("print! the console, which has not been opened yet. Call Console.begin().");
            }
        }
    };
    ($($arg:tt)*) => {
        unsafe {
            if $crate::console::CONSOLE != 0 as *mut $crate::console::Console {
                use core::fmt::Write;
                write!((*$crate::console::CONSOLE), $($arg)*).unwrap();
            } else if cfg!(feature = "training-wheels") {
                panic!("print! the console, which has not been opened yet. Call Console.begin().");
            }
        }
    };
}

/// Macro for sending a formatted string to the console (via UART0) for debugging, with a newline.
#[macro_export]
macro_rules! println {
    () => {
        unsafe {
            if $crate::console::CONSOLE != 0 as *mut $crate::console::Console {
                use core::fmt::Write;
                write!((*$crate::console::CONSOLE), "\n").unwrap();
            } else if cfg!(feature = "training-wheels") {
                panic!("println! the console, which has not been opened yet. Call Console.begin().");
            }
        }
    };
    ($fmt:expr) => {
        unsafe {
            if $crate::console::CONSOLE != 0 as *mut $crate::console::Console {
                // let &mut console = console;
                use core::fmt::Write;
                writeln!((*$crate::console::CONSOLE), $fmt).unwrap();
            } else if cfg!(feature = "training-wheels") {
                panic!(
                    "println! the console, which has not been opened yet. Call Console.begin()."
                );
            }
        }
    };
    ($fmt:expr, $($arg:tt)*) => {
        unsafe {
            if $crate::console::CONSOLE != 0 as *mut $crate::console::Console {
                use core::fmt::Write;
                writeln!((*$crate::console::CONSOLE), $fmt, $($arg)*).unwrap();
            } else if cfg!(feature = "training-wheels") {
                panic!("println! the console, which has not been opened yet. Call Console.begin().");
            }
        }
    };
}

/// Macro for flushing the console (via UART0).
#[macro_export]
macro_rules! flush {
    () => {
        unsafe {
            if $crate::console::CONSOLE != 0 as *mut $crate::console::Console {
                (*$crate::console::CONSOLE).flush().unwrap();
            }
        }
    };
}
