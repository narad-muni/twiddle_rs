use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type};

extern crate proc_macro;

#[proc_macro_derive(Twiddle)]
pub fn derive_twiddle(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = input.ident;

    // Generate code to reassign numeric fields
    let fields = if let Data::Struct(data_struct) = input.data {
        match data_struct.fields {
            Fields::Named(fields) => fields.named,
            _ => panic!("ToBe can only be derived for structs with named fields"),
        }
    } else {
        panic!("ToBe can only be derived for structs");
    };

    let assignments = fields.iter().filter_map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        let arr_type = get_arr_ty(&field.ty);

        // Check if the field type is numeric
        if is_numeric_type(field_type) {
            Some(quote! {
                self.#field_name = self.#field_name.to_be();
            })
        } else if is_floating_type(field_type) { // handle conversion for floating types
            Some(quote! {
                self.#field_name = f64::from_le_bytes(self.#field_name.to_be_bytes());
            })
        } else if arr_type.is_some() && is_floating_type(arr_type.unwrap()) { // handle conversion for floating type array
            Some(quote! {
                let mut #field_name = self.#field_name;

                #field_name.iter_mut().for_each(|el| {
                    *el = f64::from_le_bytes(el.to_be_bytes());
                });

                self.#field_name = #field_name;
            })
        } else if arr_type.is_some() && is_numeric_type(arr_type.unwrap()) { // Check if type is array and numeric
            Some(quote! {
                self.#field_name.iter_mut().for_each(|el| {
                    *el = el.to_be();
                });
            })
        } else if arr_type.is_some() && !is_primitive(arr_type.unwrap()) { // check if type is array but not primitive
            Some(quote! {
                self.#field_name.iter_mut().for_each(|el| {
                    el.twiddle();
                });
            })
        } else if !is_primitive(field_type) { // check if type is not primitive, this should be last because array is not primitive
            Some(quote! {
                self.#field_name.twiddle();
            })   
        }else {
            None
        }
    });

    let expanded = quote! {
        impl #struct_name {
            pub fn twiddle(&mut self) {
                #(#assignments)*
            }
        }
    };

    TokenStream::from(expanded)
 
}

fn is_numeric_type(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => {
            let type_segment = type_path.path.segments.last().unwrap();
            match type_segment.ident.to_string().as_str() {
                "i8" | "i16" | "i32" | "i64" | "i128" | "isize" |
                "u16" | "u32" | "u64" | "u128" | "usize" => true,
                _ => false,
            }
        }
        _ => false,
    }
}

fn is_floating_type(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => {
            let type_segment = type_path.path.segments.last().unwrap();
            match type_segment.ident.to_string().as_str() {
                "f64" | "f32"  => true,
                _ => false,
            }
        }
        _ => false,
    }
}

fn is_primitive(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => {
            let type_segment = type_path.path.segments.last().unwrap();
            match type_segment.ident.to_string().as_str() {
                "u8" | "u16" | "u32" | "u64" | "u128" |
                "i8" | "i16" | "i32" | "i64" | "i128" |
                "f32" | "f64" | "bool" | "char" | "str" |
                "usize" | "isize" => true,
                _ => false,
            }
        }
        _ => false,
    }
}

fn get_arr_ty(ty: &Type) -> Option<&Type> {
    match ty {
        Type::Array(arr) => {
            Some(&arr.elem)
        },
        _ => None,
    }
}