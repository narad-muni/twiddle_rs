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

        // Check if the field type is numeric (you can customize this logic)
        if is_numeric_type(field_type) {
            Some(quote! {
                self.#field_name = self.#field_name.to_be();
            })
        } else {
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
                "u8" | "u16" | "u32" | "u64" | "u128" | "usize" |
                "f32" | "f64" => true,
                _ => false,
            }
        }
        _ => false,
    }
}