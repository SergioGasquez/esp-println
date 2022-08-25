fn main() {
    // Ensure that only a single chip is specified
    let chip_features = [
        cfg!(feature = "esp32"),
        cfg!(feature = "esp32c2"),
        cfg!(feature = "esp32c3"),
        cfg!(feature = "esp32s2"),
        cfg!(feature = "esp32s3"),
        cfg!(feature = "esp8266"),
    ];

    match chip_features.iter().filter(|&&f| f).count() {
        1 => {}
        n => panic!("Exactly 1 chip must be enabled via its Cargo feature, {n} provided"),
    };

    // Ensure that only a single communication method is specified
    let method_features = [
        cfg!(feature = "uart"),
        cfg!(feature = "jtag_serial"),
        cfg!(feature = "rtt"),
    ];

    match method_features.iter().filter(|&&f| f).count() {
        1 => {}
        n => panic!(
            "Exactly 1 communication method must be enabled via its Cargo feature, {n} provided"
        ),
    }

    // Ensure that, if the `jtag_serial` communication method feature is
    // enabled, either the `esp32c3` or `esp32s3` chip feature is enabled.
    if cfg!(feature = "jtag_serial") && !(cfg!(feature = "esp32c3") || cfg!(feature = "esp32s3")) {
        panic!("The `jtag_serial` feature is only supported by the ESP32-C3 and ESP32-S3");
    }
}
