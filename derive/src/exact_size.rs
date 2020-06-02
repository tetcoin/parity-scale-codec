// Copyright 2020 Parity Technologies
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

//! Provide the function to implement Decode::exact_size

use proc_macro2::{Span, TokenStream};
use syn::{
	spanned::Spanned,
	Data, Fields, Field, Error, FieldsNamed, FieldsUnnamed
};

use crate::utils;

/// Implement Decode::exact_size
pub fn quote(data: &Data) -> TokenStream {
	match *data {
		Data::Struct(ref data) => exact_size_fields(&data.fields),
		Data::Enum(ref data) => {
			let data_variants = || data.variants.iter().filter(|variant| crate::utils::get_skip(&variant.attrs).is_none());

			let count = data_variants().count();
			if count > 256 {
				return Error::new(
					data.variants.span(),
					"Currently only enums with at most 256 variants are encodable."
				).to_compile_error();
			}

			// All variants are unit or only with skipped fields
			let unit_or_skipped_fields = data_variants().all(|v| match &v.fields {
				Fields::Unit => true,
				Fields::Named(FieldsNamed { named: fields , .. })
					| Fields::Unnamed(FieldsUnnamed { unnamed: fields, .. })
				=> fields.iter().all(|field| utils::get_skip(&field.attrs).is_some())
			});

			if count != 0 && unit_or_skipped_fields {
				quote! { Some(1) }
			} else {
				quote! { None }
			}
		},
		Data::Union(_) => Error::new(Span::call_site(), "Union types are not supported.").to_compile_error(),
	}
}

// return expression that return exact size of field.
fn exact_size_field(field: &Field) -> TokenStream {
	let encoded_as = utils::get_encoded_as_type(field);
	let compact = utils::get_enable_compact(field);
	let skip = utils::get_skip(&field.attrs).is_some();

	if encoded_as.is_some() as u8 + compact as u8 + skip as u8 > 1 {
		return Error::new(
			field.span(),
			"`encoded_as`, `compact` and `skip` can only be used one at a time!"
		).to_compile_error();
	}

	if compact {
		let field_type = &field.ty;
		quote_spanned! { field.span() =>
			<
				<#field_type as _parity_scale_codec::HasCompact>::Type as _parity_scale_codec::Decode
			>::exact_size()
		}
	} else if let Some(encoded_as) = encoded_as {
		quote_spanned! { field.span() =>
			<#encoded_as as _parity_scale_codec::Decode>::exact_size()
		}
	} else if skip {
		quote_spanned! { field.span() => Some(0) }
	} else {
		let field_ty = &field.ty;
		quote_spanned! { field.span() =>
			<#field_ty as _parity_scale_codec::Decode>::exact_size()
		}
	}
}

// return expression that return total exact size of fields.
fn exact_size_fields(
	fields: &Fields,
) -> TokenStream {
	let span = fields.span();
	match fields {
		Fields::Named(FieldsNamed { named: fields , .. })
			| Fields::Unnamed(FieldsUnnamed { unnamed: fields, .. })
		=> {
			let recurse = fields.iter().map(|f| exact_size_field(f));

			quote_spanned! { span =>
				Some(0)
				#(
					.and_then(|total| #recurse.map(|part| total + part))
				)*
			}
		},
		Fields::Unit => quote_spanned! { span => Some(0) },
	}
}
