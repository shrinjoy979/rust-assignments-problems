use intermediate::hard::solana_pda::derive_pda;

#[test]
fn test_basic_derivation() {
    let program_id = [1u8; 32];
    let seeds: &[&[u8]] = &[&[2u8], &[3u8]];
    let pda = derive_pda(program_id, seeds);
    // 1 ^ 2 ^ 3 = 0 for the first byte, 1 for the rest
    assert_eq!(pda[0], 0);
    assert_eq!(pda[1], 1);
}

#[test]
fn test_no_seeds() {
    let program_id = [0xAA; 32];
    assert_eq!(derive_pda(program_id, &[]), program_id);
}

#[test]
fn test_long_seeds() {
    let program_id = [0; 32];
    let seeds: &[&[u8]] = &[&[0xFF; 32]];
    assert_eq!(derive_pda(program_id, seeds), [0xFF; 32]);
}

#[test]
fn test_multiple_seeds() {
    let program_id = [0; 32];
    let seeds: &[&[u8]] = &[&[0x0F], &[0xF0]];
    let pda = derive_pda(program_id, seeds);
    assert_eq!(pda[0], 0xFF);
    assert_eq!(pda[1], 0);
}
