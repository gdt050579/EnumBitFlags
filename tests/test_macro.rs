use EnumBitFlags::EnumBitFlags;


#[EnumBitFlags]
enum Test {
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