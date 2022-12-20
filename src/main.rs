use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // It is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    println!("Hello, world!");

    loop {
        // The main loop needs a sleep of at least 10 ms somewhere. This is because 10 ms is the
        // threshold where FreeRTOS gets to do some background work, for example calming the watchdog
        // down. If the main thread is never sleeping for >=10 ms the watchdog will be upset and trigger.
        thread::sleep(Duration::from_millis(10));
    }
}
