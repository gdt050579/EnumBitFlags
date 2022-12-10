use EnumBitFlags::EnumBitFlags;


#[EnumBitFlags]
enum Test {
    V1 = 1,
    V2 = 2,
    V3 = 4
}

#[test]
fn test_1() {
    let t = Test::V1;
    println!("t = {:?}",t);
}