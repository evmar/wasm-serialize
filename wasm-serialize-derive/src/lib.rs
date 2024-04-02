use quote::{format_ident, quote, ToTokens};

/// Hacky way to decide whether to pass a type verbatim or via JsValue.
fn map_type(ty: &syn::Type) -> Option<proc_macro2::TokenStream> {
    // Returning here disables the 'pass u32 as int' optimization, for benchmarking purposes.
    // return None;
    match ty {
        syn::Type::Path(syn::TypePath {
            qself: None,
            path: p,
        }) => {
            if let Some(ident) = p.get_ident() {
                match ident.to_string().as_str() {
                    "u32" => return Some(quote!(u32)),
                    _ => {}
                }
            }
        }
        _ => {}
    }
    None
}

fn gen_struct(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let generics = input.generics.into_token_stream();
    let st = match input.data {
        syn::Data::Struct(ref s) => s,
        _ => unimplemented!(),
    };

    let js_name = format_ident!("__waser_{name}");
    let js = {
        let params = st
            .fields
            .iter()
            .map(|f| format!("{}", f.ident.as_ref().expect("field name")))
            .collect::<Vec<_>>()
            .join(",");
        format!("export function {js_name}({params}){{return{{{params}}}}}")
    };
    let extern_decl = {
        let params = st.fields.iter().map(|f| {
            let name = f.ident.as_ref().unwrap();
            let ty = map_type(&f.ty).unwrap_or(quote!(&JsValue));
            quote!(#name: #ty)
        });
        quote! {
            #[wasm_bindgen(inline_js = #js)]
            extern "C" {
                pub type JsType;
                fn #js_name(#(#params),*) -> JsValue;
            }
        }
    };

    let call = {
        let args = st.fields.iter().map(|f| {
            let name = f.ident.as_ref().unwrap();
            if map_type(&f.ty).is_some() {
                quote!(self.#name)
            } else {
                quote!(&self.#name.to_wasm())
            }
        });
        quote!(#js_name(#(#args),*))
    };

    quote! {
        const _: () = {
        #extern_decl
        impl #generics WasmSerialize for #name #generics {
            type JsType = JsType;
            #[inline(never)]
            fn to_wasm(&self) -> JsValue {
                #call
            }
        }
        impl #generics ::wasm_bindgen::convert::IntoWasmAbi for #name #generics {
            type Abi = u32;
            fn into_abi(self) -> u32 { self.to_wasm().into_abi() }
        }
        impl #generics ::wasm_bindgen::describe::WasmDescribe for #name #generics {
            #[inline]
            fn describe() {
                JsValue::describe()
            }
        }
    };
    }
}

fn gen_wrapper(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let generics = input.generics.into_token_stream();
    let st = match input.data {
        syn::Data::Struct(ref s) => s,
        _ => unimplemented!(),
    };
    let fields = match &st.fields {
        syn::Fields::Unnamed(f) => &f.unnamed,
        _ => todo!(),
    };
    if fields.len() != 1 {
        todo!();
    }
    let field = fields.first().unwrap();
    let jstype = &field.ty;

    quote! {
        impl #generics WasmSerialize for #name #generics {
            type JsType = #jstype;
            #[inline(never)]
            fn to_wasm(&self) -> JsValue {
                self.0.to_wasm()
            }
        }
        impl #generics ::wasm_bindgen::convert::IntoWasmAbi for #name #generics {
            type Abi = u32;
            fn into_abi(self) -> u32 { self.0.to_wasm().into_abi() }
        }
        impl #generics ::wasm_bindgen::describe::WasmDescribe for #name #generics {
            #[inline]
            fn describe() {
                JsValue::describe()
            }
        }
    }
}

#[proc_macro_derive(WasmSerialize)]
pub fn wasm_serialize(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    let st = match input.data {
        syn::Data::Struct(ref s) => s,
        _ => unimplemented!(),
    };

    let t = match &st.fields {
        syn::Fields::Unnamed(_) => gen_wrapper(input),
        syn::Fields::Named(_) => gen_struct(input),
        _ => todo!(),
    };
    println!("{t}");
    t.into()
}
