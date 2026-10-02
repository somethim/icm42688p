# icm42688p

A platform-independent, `no_std` Rust driver for the TDK InvenSense
ICM-42688-P six-axis IMU, built on `embedded-hal` 1.x.

## Status

Early development. SPI register access and reading the `WHO_AM_I` register are
implemented and tested with `embedded-hal-mock`. Hardware testing has not yet
been completed.

## Usage

Provide an `embedded_hal::spi::SpiDevice` implementation from your board's HAL:

```rust,ignore
use icm42688p::Icm42688;

let mut imu = Icm42688::new(spi_device);
let device_id = imu.who_am_i()?;
```

## Datasheet

[ICM-42688-P datasheet, revision 1.9](https://www.invensense.tdk.com/en-us/download-resource/ds-000347-icm-42688-p-datasheet)

## License

Licensed under either the [Apache License, Version 2.0](LICENSE-APACHE) or the
[MIT License](LICENSE-MIT), at your option.
