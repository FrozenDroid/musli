use musli::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct OtherStruct {
    field1: u32,
    field2: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Encode, Decode)]
pub enum OtherEnum {
    Variant1,
    Variant2 { field: u32 },
    Variant3(u32),
}

#[cfg(feature = "test")]
const OTHER: OtherStruct = OtherStruct {
    field1: 10,
    field2: 20,
};

#[cfg(feature = "test")]
const ENUM1: OtherEnum = OtherEnum::Variant1;
#[cfg(feature = "test")]
const ENUM2: OtherEnum = OtherEnum::Variant2 { field: 10 };
#[cfg(feature = "test")]
const ENUM3: OtherEnum = OtherEnum::Variant3(10);

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct SimpleStructFrom {
    pub field: String,
    pub interior: u32,
    pub option: Option<u32>,
    pub other: OtherStruct,
    #[musli(rename = 4)]
    pub other_enum: OtherEnum,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct SimpleStructTo {
    pub field: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct SimpleStructEnum {
    #[musli(rename = 4)]
    pub value: OtherEnum,
}

#[derive(Debug, PartialEq, Encode, Decode)]
pub struct SimpleStructToEmpty;

#[test]
#[cfg(feature = "test")]
fn test_simple_struct_compat() {
    let to = tests::wire::transcode::<_, SimpleStructTo>(SimpleStructFrom {
        field: String::from("Aristotle"),
        interior: 42,
        option: Some(108),
        other: OTHER,
        other_enum: ENUM1,
    });

    assert_eq!(
        to,
        SimpleStructTo {
            field: String::from("Aristotle"),
        }
    );
}

#[test]
#[cfg(feature = "test")]
fn test_simple_struct_to_enum_compat() {
    for expected in [ENUM1, ENUM2, ENUM3] {
        let to = tests::wire::transcode::<_, SimpleStructEnum>(SimpleStructFrom {
            field: String::from("Aristotle"),
            interior: 42,
            option: Some(108),
            other: OTHER,
            other_enum: expected,
        });

        assert_eq!(to.value, expected);
    }
}

#[test]
#[cfg(feature = "test")]
fn test_simple_struct_compat_to_empty() {
    let to = tests::wire::transcode::<_, SimpleStructToEmpty>(SimpleStructFrom {
        field: String::from("Aristotle"),
        interior: 42,
        option: Some(108),
        other: OTHER,
        other_enum: ENUM1,
    });

    assert_eq!(to, SimpleStructToEmpty);
}
