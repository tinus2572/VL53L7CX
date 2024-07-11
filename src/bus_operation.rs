use consts::*;
use crate::{consts, Vl53l7cx, Error, SevenBitAddress, I2c, OutputPin, DelayNs};

pub trait BusOperation {
    type Error;
    #[allow(dead_code)]
    fn read(&mut self, rbuf: &mut [u8]) -> Result<(), Self::Error>; 
    fn write(&mut self, wbuf: &[u8]) -> Result<(), Self::Error>;
    fn write_read(&mut self, wbuf: &[u8], rbuf: &mut [u8]) -> Result<(), Self::Error>;
}

pub struct Vl53l7cxI2C<P> {
    i2c: P,
    address: SevenBitAddress,
}

impl<P: I2c> Vl53l7cxI2C<P> {
    pub fn new(i2c: P) -> Self {
        Vl53l7cxI2C { i2c: i2c, address: VL53L7CX_DEFAULT_I2C_ADDRESS }
    }
}

impl<P: I2c> BusOperation for Vl53l7cxI2C<P> {
    type Error = P::Error;

    #[inline]
    fn read(&mut self, rbuf: &mut [u8]) -> Result<(), Self::Error> {
        self.i2c.read(self.address, rbuf)?;
        
        Ok(())
    }
    
    #[inline]
    fn write(&mut self, wbuf: &[u8]) -> Result<(), Self::Error> {
        self.i2c.write(self.address, wbuf)?;

        Ok(())
    }
    
    #[inline]
    fn write_read(&mut self, wbuf: &[u8], rbuf: &mut [u8]) -> Result<(), Self::Error> {
        self.i2c.write_read(self.address, wbuf, rbuf)?;
        
        Ok(())
    }
}

impl<P, LPN, T> Vl53l7cx<Vl53l7cxI2C<P>, LPN, T>
    where
    P: I2c,
    LPN: OutputPin,
    T: DelayNs
{
    #[allow(dead_code)]
    pub fn new_i2c(i2c: P, lpn_pin: LPN, tim: T) -> Result<Self, Error<P::Error>> 
    {
        Ok(Vl53l7cx { 
            temp_buffer: [0; VL53L7CX_TEMPORARY_BUFFER_SIZE],
            offset_data: [0; VL53L7CX_OFFSET_BUFFER_SIZE],
            xtalk_data: [0; VL53L7CX_XTALK_BUFFER_SIZE],
            streamcount: 0,
            data_read_size: 0,
            is_auto_stop_enabled: false,
            lpn_pin: lpn_pin,
            bus: Vl53l7cxI2C::new(i2c),
            tim: tim,
            chunk_size: I2C_CHUNK_SIZE
        })
    }
    
    #[allow(dead_code)]
    pub fn set_i2c_address(&mut self, i2c_address: SevenBitAddress) -> Result<(), Error<P::Error>> {
        self.write_to_register(0x7fff, 0x00)?;
        self.write_to_register(0x4, i2c_address)?;
        self.bus.address = i2c_address;
        self.write_to_register(0x7fff, 0x02)?;
        
        Ok(())
    }

    #[allow(dead_code)]
    pub fn init_sensor(&mut self, address: u8) -> Result<(), Error<P::Error>>{
        self.off()?;
        self.on()?;
        if address != self.bus.address {
            self.set_i2c_address(address)?;
        }
        self.is_alive()?;
        self.init()?;
        Ok(())
    }
}


