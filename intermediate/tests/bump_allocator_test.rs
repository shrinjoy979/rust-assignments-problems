use intermediate::hard::bump_allocator::BumpAllocator;

#[test]
fn test_alloc_ok() {
    let mut bump = BumpAllocator::new();
    let slice = bump.alloc(10).unwrap();
    assert_eq!(slice.len(), 10);
    assert_eq!(bump.cursor, 10);
}

#[test]
fn test_multiple_allocs() {
    let mut bump = BumpAllocator::new();
    let _s1 = bump.alloc(100).unwrap();
    let _s2 = bump.alloc(50).unwrap();
    assert_eq!(bump.cursor, 150);
}

#[test]
fn test_out_of_memory() {
    let mut bump = BumpAllocator::new();
    let _s1 = bump.alloc(1000).unwrap();
    assert!(bump.alloc(50).is_err());
}

#[test]
fn test_exact_memory() {
    let mut bump = BumpAllocator::new();
    assert!(bump.alloc(1024).is_ok());
    assert_eq!(bump.cursor, 1024);
}
