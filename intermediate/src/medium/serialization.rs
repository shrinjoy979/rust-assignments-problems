/*
  Problem 47: Serialization — Manual to_bytes / from_bytes

  Define a struct Record { id: u32, value: u16 }. Implement methods
  to_bytes(&self) -> Vec<u8> and from_bytes(data: &[u8]) -> Result<Self, String>
  using little-endian byte order. The serialized format should be
  [id: 4 bytes][value: 2 bytes] = 6 bytes total.

  Run the tests for this problem with:
    cargo test --test serialization_test
*/

#[derive(Debug, PartialEq)]
pub struct Record {
    pub id: u32,
    pub value: u16,
}

impl Record {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut v: Vec<u8> = self.id.to_le_bytes().to_vec();
        v.extend(&self.value.to_le_bytes());
        v
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        if data.len() != 6 {
            return Err("Invalid Length".to_string());
        }

        let id = u32::from_le_bytes(
            data[0..4].try_into().map_err(|_| "Invalid id bytes".to_string())?
        );

        let value = u16::from_le_bytes(
            data[4..6].try_into().map_err(|_| "Invalid value bytes".to_string())?
        );

        Ok(Record { id, value })
    }
}
