use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Channel {
    // START, SGL/DIF, PCH0, PCH2, PCH1, X, X, X
    SGL0 = 0b1_1_000_000,
    SGL1 = 0b1_1_100_000,
    SGL2 = 0b1_1_001_000,
    SGL3 = 0b1_1_101_000,
    SGL4 = 0b1_1_010_000,
    SGL5 = 0b1_1_110_000,
    SGL6 = 0b1_1_011_000,
    SGL7 = 0b1_1_111_000,
    DIF01 = 0b1_0_000_000,
    DIF10 = 0b1_0_100_000,
    DIF23 = 0b1_0_001_000,
    DIF32 = 0b1_0_101_000,
    DIF45 = 0b1_0_010_000,
    DIF54 = 0b1_0_110_000,
    DIF67 = 0b1_0_011_000,
    DIF76 = 0b1_0_111_000,
}

pub struct TLV0838<SPI, CS>
where
    SPI: Transfer<u8>,
    CS: OutputPin,
{
    spi: SPI,
    cs: CS,
}

impl<SPI, E, CS> TLV0838<SPI, CS>
where
    SPI: Transfer<u8, Error = E>,
    CS: OutputPin<Error = E>,
{
    pub fn new(spi: SPI, cs: CS) -> Self {
        Self { spi, cs }
    }

    pub fn read_channel(&mut self, ch: Channel) -> Result<u8, E> {
        let mut master_buf = [ch as u8, 0, 0];
        self.cs.set_low()?;
        let slave_buf = self.spi.transfer(&mut master_buf)?;
        self.cs.set_high()?;
        let out = ((slave_buf[0] & 0b00000011) << 6) | ((slave_buf[1] & 0b11111100) >> 2);
        return Ok(out & 0xFF);
    }
}
