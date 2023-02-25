use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Channel {
    SGL0 = 0b11000000,
    SGL1 = 0b11100000,
    SGL2 = 0b11100100,
    SGL3 = 0b11101000,
    SGL4 = 0b11010000,
    SGL5 = 0b11110000,
    SGL6 = 0b11011000,
    SGL7 = 0b11111000,
    DIF01 = 0b10000000,
    DIF10 = 0b10100000,
    DIF23 = 0b10001000,
    DIF32 = 0b10101000,
    DIF45 = 0b10010000,
    DIF54 = 0b10110000,
    DIF67 = 0b10011000,
    DIF76 = 0b10111000,
}

pub struct TLV0838<SPI, CS>
where
    SPI: Transfer<u8>,
    CS: OutputPin
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
        self.cs.set_high()?;
        let slave_buf = self.spi.transfer(&mut master_buf)?;
        self.cs.set_low()?;
        println!("{:?}", slave_buf);
        let out = ((slave_buf[0] & 0b00000011) << 6) | ((slave_buf[1] & 0b11111100) >> 2);
        return Ok(out & 0xFF);
    }
}
