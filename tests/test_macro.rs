use EnumBitFlags::EnumBitFlags;


#[EnumBitFlags]
enum Test {
    V1 = 1,
    V2 = 2,
    V3 = 4
}

#[test]
fn test_1() {
    let t = Test::V1 | Test::V2;
    assert!(t.contains(Test::V1));
    assert!(t.contains(Test::V2));
    assert!(t.contains(Test::V3)==false);
}