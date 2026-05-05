/*
  Problem 53: Solana-Style Account Data — Discriminator

  Simulate a Solana-style account data structure. Define a trait AccountData
  with methods discriminator() -> [u8; 8], serialize(&self) -> Vec<u8> and
  deserialize(data) -> Result<Self, String>. Implement it for TokenAccount
  { owner: [u8; 32], amount: u64 }. The serialized format is
  [discriminator: 8 bytes][owner: 32 bytes][amount: 8 bytes LE].

  Run the tests for this problem with:
    cargo test --test solana_discriminator_test
*/

pub trait AccountData: Sized {
    fn discriminator() -> [u8; 8];
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &[u8]) -> Result<Self, String>;
}

#[derive(Debug, PartialEq)]
pub struct TokenAccount {
    pub owner: [u8; 32],
    pub amount: u64,
}

impl AccountData for TokenAccount {
    fn discriminator() -> [u8; 8] {
        // Use a fixed discriminator: "TOKENACC"
        [0x54, 0x4f, 0x4b, 0x45, 0x4e, 0x41, 0x43, 0x43]
    }

    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();

        // 8 bytes
        data.extend(&Self::discriminator());

        // 32 bytes
        data.extend(&self.owner);

        // 8 bytes
        data.extend(&self.amount.to_le_bytes());

        data
    }

    fn deserialize(data: &[u8]) -> Result<Self, String> {
        // total expected: 8 + 32 + 8 => 48
        if data.len() != 48 {
            return Err("Invalid inputs".to_string());
        }

        // check discriminator
        let disc: [u8; 8] = data[0..8]
            .try_into()
            .map_err(|_| "invalid discriminator".to_string())?;

        if disc != Self::discriminator() {
            return Err("discriminator mismatch".to_string());
        }

        // check owner
        let owner: [u8; 32] = data[9..40]
            .try_into()
            .map_err(|_| "Invalid owner byte")?;

        // check amount
        let amount = u64::from_le_bytes(
            data
                .try_into()
                .map_err(|_| "Invalid inputs".to_string())?
            );

        Ok(TokenAccount {owner, amount})
    }
}
