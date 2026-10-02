#![no_std]

mod registers;
mod spi;

use embedded_hal::spi::SpiDevice;

use crate::{
    registers::WHO_AM_I_ADDRESS,
    spi::{SPI_DUMMY_BYTE, read_command},
};

pub struct Icm42688<SPI> {
    spi: SPI,
}

impl<SPI: SpiDevice<u8>> Icm42688<SPI> {
    pub fn new(spi: SPI) -> Self {
        Self { spi }
    }

    pub fn who_am_i(&mut self) -> Result<u8, SPI::Error> {
        self.read_register(WHO_AM_I_ADDRESS)
    }

    fn read_register(&mut self, register_address: u8) -> Result<u8, SPI::Error> {
        let mut buffer = [read_command(register_address), SPI_DUMMY_BYTE];

        self.spi.transfer_in_place(&mut buffer)?;

        Ok(buffer[1])
    }

    fn write_register(
        &mut self,
        register_address: u8,
        register_value: u8,
    ) -> Result<(), SPI::Error> {
        let buffer = [register_address, register_value];

        self.spi.write(&buffer)?;

        Ok(())
    }
}
