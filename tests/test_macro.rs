use EnumBitFlags::EnumBitFlags;


#[EnumBitFlags(bits : 8,empty=None)]
enum Test {
    V1 = 1,
    V2 = 2,
    V3 = 4
}

#[EnumBitFlags(empty=NoBitsSet)]
enum Test2 {
    V1 = 1,
    V2 = 2,
    V3 = 4
}

#[test]
fn test_bit_or() {
    let t = Test::V1 | Test::V2;   
    assert!(t.contains(Test::V1));
    assert!(t.contains(Test::V2));
    assert!(t.contains(Test::V3)==false);
    assert!(t.contains(Test::V1|Test::V3)==false);
    assert!(t.contains_one(Test::V1|Test::V3));
}
#[test]
fn test_bit_or_assign() {
    let mut t = Test::V1;
    t |= Test::V1;
    t |= Test::V2;
    assert!(t.contains(Test::V1));
    assert!(t.contains(Test::V2));
    assert!(t.contains(Test::V3)==false);
}

#[test]
fn test_empty() {
    let mut t = Test2::NoBitsSet;
    assert!(t.is_empty());
    t = Test2::V1;
    assert!(t.is_empty()==false);
}

#[test]
fn test_size() {
    assert!(std::mem::size_of::<Test>()==1); // u8
    assert!(std::mem::size_of::<Test2>()==4);// u32
}