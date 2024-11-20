use EnumBitFlags::EnumBitFlags;


#[EnumBitFlags(bits : 8,empty=None)]
enum Test {
    V1 = 1,
    V2 = 2,
    V3 = 128,
}

#[EnumBitFlags(empty=NoBitsSet)]
pub enum Test2 {
    V1 = 1,
    V2 = 2,
    V3 = 4
}

#[EnumBitFlags]
pub enum Test3 {
    V1 = 1,
    V2 = 2,
    V3 = 4,
    Nothing = 0
}

#[EnumBitFlags(disable_empty_generation: true)]
pub enum Test4 {
    V1 = 1,
    V2 = 2,
    V3 = 4,
}

#[EnumBitFlags(bits=16,debug=true)]
pub enum Test_16bit {
    V1 = 1,
    V2 = 2,
    V3 = 4,
    V4 = 0x8000
}
#[EnumBitFlags(bits=32)]
pub enum Test_32bit {
    V1 = 1,
    V2 = 2,
    V3 = 4,
    V4 = 0x80000000
}
#[EnumBitFlags(bits=64)]
pub enum Test_64bit {
    V1 = 1,
    V2 = 2,
    V3 = 4,
    V4 = 0x8000000000000000
}

#[EnumBitFlags]
pub(crate) enum TestVisibilityEnum {
    V1 = 1,
    V2 = 2,
    V3 = 4,
    V4 = 8
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
fn test_bit_and() {
    let tmp = Test::V1 | Test::V2; 
    let t = tmp & Test::V1;
    assert!(t.contains(Test::V1));
    assert!(t.contains(Test::V2)==false);
    assert!(t.contains(Test::V3)==false);
}

#[test]
fn test_bitand_assign() {
    let mut t = Test::V1 | Test::V2; 
    t &= Test::V1;
    assert!(t.contains(Test::V1));
    assert!(t.contains(Test::V2)==false);
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
    assert!(std::mem::size_of::<Test_16bit>()==2);// u16
    assert!(std::mem::size_of::<Test_32bit>()==4);// u32
    assert!(std::mem::size_of::<Test_64bit>()==8);// u64
}

#[test]
fn test_clear_method() {
    let mut t = Test::V1 | Test::V2;
    assert!(t.is_empty()==false);
    t.clear();
    assert!(t.is_empty()==true);
}

#[test]
fn test_set_method() {
    let mut t = Test::V1;
    assert!(t.contains(Test::V2)==false);
    assert!(t.contains(Test::V1));
    t.set(Test::V2);
    assert!(t.contains(Test::V3)==false);
    assert!(t.contains(Test::V2));
    assert!(t.contains(Test::V1));
}

#[test]
fn test_remove_method() {
    let mut t = Test::V1|Test::V2;
    assert!(t.contains(Test::V3)==false);
    assert!(t.contains(Test::V2));
    assert!(t.contains(Test::V1));
    t.remove(Test::V2);
    assert!(t.contains(Test::V3)==false);
    assert!(t.contains(Test::V2)==false);
    assert!(t.contains(Test::V1));
    t.remove(Test::V3);
    assert!(t.contains(Test::V3)==false);
    assert!(t.contains(Test::V2)==false);
    assert!(t.contains(Test::V1));    
    t.remove(Test::V2);
    assert!(t.contains(Test::V3)==false);
    assert!(t.contains(Test::V2)==false);
    assert!(t.contains(Test::V1));     
    t.remove(Test::V1);
    assert!(t.contains(Test::V3)==false);
    assert!(t.contains(Test::V2)==false);
    assert!(t.contains(Test::V1)==false);   
    assert!(t.is_empty()==true);  
}

#[test]
fn test_default_implementation() {
    let t = Test::default();
    assert!(t.is_empty());  
}

#[test]
fn test_cmp_opes() {
    let mut t = Test::V1;
    assert!(t == Test::V1);
    assert!(t != Test::V1|Test::V2);
    t |= Test::V2;
    assert!(t != Test::V1);
    assert!(t == Test::V1|Test::V2);
}

#[test]
fn test_contains() {
    let t = Test::V1 | Test::V2; 
    assert!(t.contains(Test::V1));
    assert!(t.contains(Test::V2));
    assert!(t.contains(Test::V3)==false);
    assert!((t & Test::V1) == Test::V1);
    assert!((t & Test::V2) == Test::V2);
    assert!((t & Test::V3) == Test::None);
}

#[test]
fn test_display() {
    let t1 = Test::V1 | Test::V2; 
    assert_eq!(format!("{}",t1),"Test (V1 | V2)");
    let t2 = Test::default();
    assert_eq!(format!("{}",t2),"Test (None)");
}

#[test]
fn test_empty_value() {
    let t = Test3::default();
    assert_eq!(t,Test3::Nothing);
}

#[test]
fn test_get_value() {
    let mut t = Test3::default();
    assert_eq!(t.get_value(),0);
    t = Test3::V1 | Test3::V2;
    assert_eq!(t.get_value(),3);
    t.remove(Test3::V1);
    assert_eq!(t.get_value(),Test3::V2.get_value());
}
#[test]
fn test_const_get_value() {
    let t = (Test::V1 | Test::V2).get_value();
    assert_eq!(t,3);
    const CONST_T: u8 = Test::V1.get_value() | Test::V2.get_value();
    assert_eq!(CONST_T,3);
}

#[test]
fn test_visibility_group() {
    let t = (TestVisibilityEnum::V1 | TestVisibilityEnum::V2).get_value();
    assert_eq!(t,3);
    const CONST_T: u32 = TestVisibilityEnum::V1.get_value() | TestVisibilityEnum::V2.get_value();
    assert_eq!(CONST_T,3);
}

#[test]
fn test_new() {
    assert_eq!(Test::new(3), Some(Test::V1 | Test::V2));
    assert_eq!(Test::new(255), None);
    assert_eq!(Test4::new(3), Some(Test4::V1 | Test4::V2));
    assert_eq!(Test4::new(0), None);
}