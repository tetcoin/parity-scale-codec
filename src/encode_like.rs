// Copyright 2019 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::Encode;

/// A marker trait that tells the compiler that two types encode to the same representation.
///
/// E.g. `Vec<u8>` has the same encoded representation as `&[u8]`.
///
/// # Example
///
/// ```
///# use tetsy_scale_codec::{EncodeLike, Encode};
/// fn encode_like<T: EncodeLike<R>, R: Encode>(data: &R) {
///     data.encode();
/// }
///
/// fn main() {
///     // Just pass the a reference to the normal tuple.
///     encode_like::<(u32, u32), _>(&(1u32, 2u32));
///     // Pass a tuple of references
///     encode_like::<(u32, u32), _>(&(&1u32, &2u32));
///     // Pass a tuple of a reference and a value.
///     encode_like::<(u32, u32), _>(&(&1u32, 2u32));
/// }
/// ```
pub trait EncodeLike<T: Encode = Self>: Sized + Encode {}

impl<T: Encode> EncodeLike<&T> for T {}
impl<T: Encode> EncodeLike<T> for &T {}

#[cfg(test)]
mod tests {
	use super::*;
	use std::collections::BTreeMap;

	struct ComplexStuff<T>(T);

	impl<T: Encode> ComplexStuff<T> {
		fn complex_method<R: Encode>(value: &R) -> Vec<u8> where T: EncodeLike<R> {
			value.encode()
		}
	}

	#[test]
	fn vec_and_slice_are_working() {
		let slice: &[u8] = &[1, 2, 3, 4];
		let data: Vec<u8> = slice.iter().copied().collect();

		let data_encoded = data.encode();
		let slice_encoded = ComplexStuff::<Vec<u8>>::complex_method(&slice);

		assert_eq!(slice_encoded, data_encoded);
	}

	#[test]
	fn btreemap_and_slice_are_working() {
		let slice: &[(u32, u32)] = &[(1, 2), (23, 24), (28, 30), (45, 80)];
		let data: BTreeMap<u32, u32> = slice.iter().copied().collect();

		let data_encoded = data.encode();
		let slice_encoded = ComplexStuff::<BTreeMap<u32, u32>>::complex_method(&slice);

		assert_eq!(slice_encoded, data_encoded);
	}

	#[test]
	fn interface_testing() {
		let value = 10u32;
		let data = (value, value, value);
		let encoded = ComplexStuff::<(u32, u32, u32)>::complex_method(&data);
		assert_eq!(data.encode(), encoded);
		let data = (&value, &value, &value);
		let encoded = ComplexStuff::<(u32, u32, u32)>::complex_method(&data);
		assert_eq!(data.encode(), encoded);
		let data = (&value, value, &value);
		let encoded = ComplexStuff::<(u32, u32, u32)>::complex_method(&data);
		assert_eq!(data.encode(), encoded);

		let vec_data: Vec<u8> = vec![1, 2, 3];
		ComplexStuff::<Vec<u8>>::complex_method(&vec_data);
		ComplexStuff::<&'static str>::complex_method(&String::from("test"));
		ComplexStuff::<&'static str>::complex_method(&"test");

		let slice: &[u8] = &vec_data;
		assert_eq!(
			ComplexStuff::<(u32, Vec<u8>)>::complex_method(&(1u32, slice.to_vec())),
			ComplexStuff::<(u32, Vec<u8>)>::complex_method(&(1u32, slice))
		);
	}
}
