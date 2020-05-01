/// Implemented based on: https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/system/esp_timer.html
use crate::esp_idf::esp_timer_get_time;

pub fn micros() -> u64 {
    unsafe { esp_timer_get_time() as u64 }

    // (xtensa_lx6_rt::get_cycle_count() as u64 / (self.apb_freq as u64 / 1000_000)) as u64
}

pub fn millis() -> u64 {
    micros() / 1000

    // (xtensa_lx6_rt::get_cycle_count() as u64 / (self.apb_freq as u64 / 1000_000)) as u64
}
