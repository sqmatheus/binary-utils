use std::ops::Deref;
use std::convert::TryInto;

pub trait IBufferRead {
     fn read_byte(&self) -> u8;
     fn read_signed_byte(&self) -> u8;
     fn read_bool(&self) -> bool;
     fn read_short(&self) -> u8;
     fn read_signed_short(&self) -> u8;
     fn read_short_le(&self) -> u8;
     fn read_signed_short_le(&self) -> u8;
     fn read_triad(&self) -> usize;
     fn read_triad_le(&self) -> usize;
     fn read_int(&self) -> usize;
     fn read_int_le(&self) -> usize;
     fn read_float(&self) -> f32;
     fn read_float_le(&self) -> f32;
     fn read_double(&self) -> f64;
     fn read_double_le(&self) -> f64;
     fn read_long(&self) -> usize;
     fn read_long_le(&self) -> usize;
     fn read_var_int(&self) -> usize;
     fn read_signed_var_int(&self) -> usize;
     fn read_var_long(&self) -> usize;
     fn read_signed_var_long(&self) -> usize;
}

pub trait IBufferWrite {
     fn write_byte(&self, v: u8);
     fn write_signed_byte(&self, v: u8);
     fn write_bool(&self, v: bool);
     fn write_short(&self, v: u8);
     fn write_signed_short(&self, v: u8);
     fn write_short_le(&self, v: u8);
     fn write_signed_short_le(&self, v: u8);
     fn write_triad(&self, v: usize);
     fn write_triad_le(&self, v: usize);
     fn write_int(&self, v: usize);
     fn write_int_le(&self, v: usize);
     fn write_float(&self, v: f32);
     fn write_float_le(&self, v: f32);
     fn write_double(&self, v: f64);
     fn write_double_le(&self, v: f64);
     fn write_long(&self, v: usize);
     fn write_long_le(&self, v: usize);
     fn write_var_int(&self, v: usize);
     fn write_signed_var_int(&self, v: usize);
     fn write_var_long(&self, v: usize);
     fn write_signed_var_long(&self, v: usize);
}


/// Buffer implementation on Array (im lazy someone pls)
impl<T> IBufferRead for T where T: Deref<Target = [u8]> {
     fn read_byte(&self) -> u8 {
          0
     }

     fn read_signed_byte(&self) -> u8 {
          0
     }

     fn read_bool(&self) -> bool {
          false
     }

     fn read_short(&self) -> u8 {
          0
     }

     fn read_signed_short(&self) -> u8 {
          0
     }

     fn read_short_le(&self) -> u8 {
          0
     }

     fn read_signed_short_le(&self) -> u8 {
          0
     }

     fn read_triad(&self) -> usize {
          0
     }

     fn read_triad_le(&self) -> usize {
          0
     }

     fn read_int(&self) -> usize {
          0
     }

     fn read_int_le(&self) -> usize {
          0
     }

     fn read_float(&self) -> f32 {
          0.0
     }

     fn read_float_le(&self) -> f32 {
          0.0
     }

     fn read_double(&self) -> f64 {
          0.0
     }

     fn read_double_le(&self) -> f64 {
          0.0
     }

     fn read_long(&self) -> usize {
          0
     }

     fn read_long_le(&self) -> usize {
          0
     }

     fn read_var_int(&self) -> usize {
          0
     }

     fn read_signed_var_int(&self) -> usize {
          0
     }

     fn read_var_long(&self) -> usize {
          0
     }

     fn read_signed_var_long(&self) -> usize {
          0
     }
}