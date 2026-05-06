/*
  Problem 54: Data Packing — Bitfield Struct

  Define a struct PackedFlags that stores 8 boolean flags in a single u8.
  Implement methods new(), set(index, value), get(index), and as_byte().
  Index ranges from 0–7.

  Run the tests for this problem with:
    cargo test --test packed_flags_test
*/

#[derive(Debug, PartialEq)]
pub struct PackedFlags {
    pub bits: u8,
}

impl PackedFlags {
    // all flags = false (0)
    pub fn new() -> Self {
        PackedFlags { bits: 0 }
    }

    // set or clear a bit at index
    pub fn set(&mut self, index: u8, value: bool) {
        let mask = 1 << index;

        if value {
            // set bit (make 1)
            self.bits |= mask;
        } else {
            // clear bit (make 0)
            self.bits &= !mask;
        }
    }

    // read a bit at index
    pub fn get(&self, index: u8) -> bool {
        let mask = 1 << index;
        (self.bits & mask) != 0
    }

    // return raw byte
    pub fn as_byte(&self) -> u8 {
        self.bits
    }
}
