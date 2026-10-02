use embedded_hal_mock::eh1::spi::{Mock as SpiMock, Transaction as SpiTransaction};
use icm42688p::Icm42688;

const EXPECTED_WHO_AM_I_READ_COMMAND: u8 = 0xF5;
const SPI_DUMMY_BYTE: u8 = 0x00;
const IGNORED_RESPONSE_BYTE: u8 = 0x00;
const EXPECTED_WHO_AM_I_VALUE: u8 = 0x47;

#[test]
fn who_am_i_returns_device_id() {
    let expectations = [
        SpiTransaction::transaction_start(),
        SpiTransaction::transfer_in_place(
            vec![EXPECTED_WHO_AM_I_READ_COMMAND, SPI_DUMMY_BYTE],
            vec![IGNORED_RESPONSE_BYTE, EXPECTED_WHO_AM_I_VALUE],
        ),
        SpiTransaction::transaction_end(),
    ];

    let spi = SpiMock::new(&expectations);
    let mut spi_handle = spi.clone();
    let mut icm42688 = Icm42688::new(spi);

    let value = icm42688.who_am_i().unwrap();

    assert_eq!(value, EXPECTED_WHO_AM_I_VALUE);
    spi_handle.done();
}
