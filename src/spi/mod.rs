const SPI_READ_BIT: u8 = 1 << 7;

pub(crate) const SPI_DUMMY_BYTE: u8 = 0x00;

pub(crate) fn read_command(register_address: u8) -> u8 {
    register_address | SPI_READ_BIT
}

#[cfg(test)]
mod tests;
