use parity_scale_codec_derive::Decode;
use parity_scale_codec::{Decode, HasCompact};

#[derive(Decode)]
struct Unit;
#[test]
fn test_unit() {
	assert_eq!(Unit::exact_size(), Some(0))
}

#[derive(Decode)]
struct Field(u32);
#[test]
fn test_field() {
	assert_eq!(Field::exact_size(), Some(4))
}

#[derive(Decode)]
struct Fields2(u32, u64);
#[test]
fn test_fields2() {
	assert_eq!(Fields2::exact_size(), Some(12))
}

#[derive(Decode)]
struct FieldsNoSize(Vec<u32>);
#[test]
fn test_fieldsnosize() {
	assert_eq!(FieldsNoSize::exact_size(), None)
}

#[derive(Decode)]
struct FieldsNoSizeSkipped(#[codec(skip)] Vec<u32>);
#[test]
fn test_fieldsnosizeskipped() {
	assert_eq!(FieldsNoSizeSkipped::exact_size(), Some(0))
}

#[derive(Decode)]
struct FieldsNoSizeSkipped2(#[codec(skip)] Vec<u32>, i8);
#[test]
fn test_fieldsnosizeskipped2() {
	assert_eq!(FieldsNoSizeSkipped2::exact_size(), Some(1))
}

#[derive(Decode)]
enum Enum {}
#[test]
fn test_enum() {
	assert_eq!(Enum::exact_size(), None)
}

#[derive(Decode)]
enum EnumUnit {
	A,
	B,
	C,
}
#[test]
fn test_enumunit() {
	assert_eq!(EnumUnit::exact_size(), Some(1))
}

#[derive(Decode)]
enum EnumWithSkippedField {
	A(#[codec(skip)]()),
	B,
	C,
	#[codec(skip)]
	#[allow(dead_code)]
	D(u32),
}
#[test]
fn test_enumunitwithskippedfield() {
	assert_eq!(EnumWithSkippedField::exact_size(), Some(1))
}

#[derive(Decode)]
enum EnumWithField {
	A(()),
	B,
	C,
}
#[test]
fn test_enumunitwithfield() {
	assert_eq!(EnumWithField::exact_size(), None)
}

#[derive(Decode)]
enum EnumWithField2 {
	A,
	B,
	C,
	D(u32),
}
#[test]
fn test_enumunitwithfield2() {
	assert_eq!(EnumWithField2::exact_size(), None)
}

#[derive(Decode)]
enum EnumCompact {
	A(#[codec(compact)] u32)
}
#[test]
fn test_enumcompact() {
	assert_eq!(EnumCompact::exact_size(), None)
}

#[derive(Decode)]
enum EnumEncodedAs {
	A(#[codec(encoded_as = "<u32 as HasCompact>::Type")] u32)
}
#[test]
fn test_enumencodedas() {
	assert_eq!(EnumEncodedAs::exact_size(), None)
}
