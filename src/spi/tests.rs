extern crate std;

use super::{SPI_DUMMY_BYTE, read_command};
use crate::{Icm42688, registers::WHO_AM_I_ADDRESS};
use embedded_hal_mock::eh1::spi::{Mock as SpiMock, Transaction as SpiTransaction};

const EXPECTED_WHO_AM_I_READ_COMMAND: u8 = 0xF5;
const IGNORED_RESPONSE_BYTE: u8 = 0x00;
const EXPECTED_WHO_AM_I_VALUE: u8 = 0x47;

const EXPECTED_PWR_MGMT0_WRITE_COMMAND: u8 = 0x4E;
const EXPECTED_SIX_AXIS_LOW_NOISE_VALUE: u8 = 0x0F;

#[test]
fn read_command_sets_spi_read_bit() {
    assert_eq!(
        read_command(WHO_AM_I_ADDRESS),
        EXPECTED_WHO_AM_I_READ_COMMAND
    );
}

#[test]
fn read_register_transfers_command_and_returns_value() {
    let expectations = [
        SpiTransaction::transaction_start(),
        SpiTransaction::transfer_in_place(
            std::vec![EXPECTED_WHO_AM_I_READ_COMMAND, SPI_DUMMY_BYTE],
            std::vec![IGNORED_RESPONSE_BYTE, EXPECTED_WHO_AM_I_VALUE],
        ),
        SpiTransaction::transaction_end(),
    ];

    let spi = SpiMock::new(&expectations);
    let mut spi_handle = spi.clone();
    let mut icm42688 = Icm42688::new(spi);

    let value = icm42688.read_register(WHO_AM_I_ADDRESS).unwrap();

    assert_eq!(value, EXPECTED_WHO_AM_I_VALUE);
    spi_handle.done();
}

#[test]
fn write_register_transmits_address_and_value() {
    let expectations = [
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec(std::vec![
            EXPECTED_PWR_MGMT0_WRITE_COMMAND,
            EXPECTED_SIX_AXIS_LOW_NOISE_VALUE
        ]),
        SpiTransaction::transaction_end(),
    ];

    let spi = SpiMock::new(&expectations);
    let mut spi_handle = spi.clone();
    let mut icm42688 = Icm42688::new(spi);

    icm42688
        .write_register(
            EXPECTED_PWR_MGMT0_WRITE_COMMAND,
            EXPECTED_SIX_AXIS_LOW_NOISE_VALUE,
        )
        .unwrap();

    spi_handle.done();
}
