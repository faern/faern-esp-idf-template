# faern-esp-idf-template

Just my own adaptation of https://github.com/esp-rs/esp-idf-template
with some updates and tweaks that fits my workflow and gets me started faster.

Non-exhaustive list of changes from the template

* Pre-select ESP32-C3 since that's the chip I use the most
* Pre-select ESP-IDF 4.4, why would I want something older?
* Add sdkconfig settings to enable flashing and monitoring via native USB
* Remove dependency on `anyhow` and just use `Result<(), Box<dyn std::error::Error>>` instead.
* Upgrade `esp-idf-sys` to latest version (at time of writing)
* Configure `esp-idf-sys` to build ESP-IDF in a global path, and handle workspaces better
* Add notes about how apps need to sleep at least 10ms in at least one place in the main loop.