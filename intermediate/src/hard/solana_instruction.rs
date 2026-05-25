/*
  Problem 60: Solana-Style Instruction — Unpack

  Simulate unpacking a Solana instruction. Define an enum Instruction with variants
  Initialize, Mint { amount: u64 }, and Transfer { amount: u64 }.
  Write a function unpack(data: &[u8]) -> Result<Instruction, String>.
  Data format: [tag: 1 byte][data: remaining bytes LE].
  Tags: 0 = Initialize, 1 = Mint, 2 = Transfer.

  Run the tests for this problem with:
    cargo test --test solana_instruction_test
*/

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Initialize,
    Mint { amount: u64 },
    Transfer { amount: u64 },
}

pub fn unpack(data: &[u8]) -> Result<Instruction, String> {
    // Must have at least 1 byte for the tag
    let (&tag, rest) = data.split_first()
        .ok_or("Empty instruction data".to_string())?;

    match tag {
        0 => Ok(Instruction::Initialize),

        1 => {
            let amount = parse_u64_le(rest)?;
            Ok(Instruction::Mint { amount })
        }

        2 => {
            let amount = parse_u64_le(rest)?;
            Ok(Instruction::Transfer { amount })
        }

        _ => Err(format!("Unknown instruction tag: {}", tag)),
    }
}

fn parse_u64_le(data: &[u8]) -> Result<u64, String> {
    if data.len() < 8 {
        return Err(format!(
            "Not enough bytes for u64: expected 8, got {}",
            data.len()
        ));
    }
    // Copy exactly 8 bytes into a fixed array, then interpret as little-endian u64
    let bytes: [u8; 8] = data[..8].try_into()
        .map_err(|_| "Failed to read 8 bytes".to_string())?;
    Ok(u64::from_le_bytes(bytes))
}