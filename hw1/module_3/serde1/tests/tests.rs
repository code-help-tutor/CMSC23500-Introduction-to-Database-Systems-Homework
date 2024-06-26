WeChat: cstutorcs
QQ: 749389476
Email: tutorcs@163.com
use solution::{serialize_to_string, serialize_to_bytes, deserialize_from_bytes};

#[test]
fn check_serialize_to_string() {
    let integer: u32 = 2147483647;
    let integer_in_string = serialize_to_string(integer);
    assert_eq!(integer_in_string, "2147483647");
}

#[test]
fn check_serialize_to_bytes() {
    let integer: u32 = 2147483647;
    let integer_in_string = serialize_to_bytes(integer);
    assert_eq!(integer_in_string, integer.to_be_bytes());
}

#[test]
fn check_deserialize_from_bytes() {
    let integer: u32 = 2147483647;
    let integer_deser = deserialize_from_bytes(integer.to_be_bytes());
    assert_eq!(integer_deser, integer);
}
