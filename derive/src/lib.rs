use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Meta, Expr, Lit};

#[proc_macro_derive(Packet, attributes(packet))]
pub fn derive_packet(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    // Vérifier que c'est une struct
    match &input.data {
        Data::Struct(_) => (),
        _ => {
            return syn::Error::new_spanned(&input, "Packet can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    let name = &input.ident;
    
    // Parser l'attribut #[packet(code = 0x01)]
    let packet_code = extract_packet_code(&input);
    
    let packet_code = match packet_code {
        Ok(code) => code,
        Err(err) => return err.to_compile_error().into(),
    };

    let expanded = quote! {
        impl crate::packets::Packet for #name {
            fn serialize(&self) -> Result<Vec<u8>, crate::packets::PacketError> {
                let mut data: Vec<u8> = Vec::new();
                let encoded_packet = match bincode::encode_to_vec(self, bincode::config::standard()) {
                    Ok(packet) => packet,
                    Err(e) => return Err(crate::packets::PacketError::EncodingError(e.to_string()))
                };
                data.push(#packet_code);
                data.extend(&encoded_packet);
                Ok(data)
            }

            fn deserialize(data: &[u8]) -> Result<Self, crate::packets::PacketError>
            where
                Self: Sized,
            {
                let (decoded, _) = match bincode::decode_from_slice(data, bincode::config::standard()) {
                    Ok(res) => res,
                    Err(e) => return Err(crate::packets::PacketError::DecodingError(e.to_string()))
                };
                Ok(decoded)
            }

            fn packet_code() -> u8 {
                #packet_code
            }
        }
    };

    TokenStream::from(expanded)
}

fn extract_packet_code(input: &DeriveInput) -> Result<u8, syn::Error> {
    for attr in &input.attrs {
        if attr.path().is_ident("packet") {
            match &attr.meta {
                Meta::List(meta_list) => {
                    let tokens = &meta_list.tokens;
                    let parsed: syn::Result<syn::MetaNameValue> = syn::parse2(tokens.clone());
                    
                    match parsed {
                        Ok(name_value) => {
                            if name_value.path.is_ident("code") {
                                match &name_value.value {
                                    Expr::Lit(expr_lit) => {
                                        match &expr_lit.lit {
                                            Lit::Int(lit_int) => {
                                                let token_str = lit_int.to_string();
                                                
                                                // Vérifier que c'est bien un code hexadécimal
                                                if !token_str.starts_with("0x") && !token_str.starts_with("0X") {
                                                    return Err(syn::Error::new_spanned(
                                                        lit_int, 
                                                        "Packet code must be in hexadecimal format (e.g., 0x01)"
                                                    ));
                                                }
                                                
                                                // Parser la valeur hexadécimale
                                                let hex_value = &token_str[2..]; // Retirer le préfixe "0x"
                                                let value = u8::from_str_radix(hex_value, 16)
                                                    .map_err(|_| syn::Error::new_spanned(
                                                        lit_int, 
                                                        "Invalid hexadecimal packet code - must be between 0x00 and 0xFF"
                                                    ))?;
                                                
                                                return Ok(value);
                                            }
                                            _ => return Err(syn::Error::new_spanned(&name_value.value, "Packet code must be a hexadecimal integer (e.g., 0x01)")),
                                        }
                                    }
                                    _ => return Err(syn::Error::new_spanned(&name_value.value, "Packet code must be a literal hexadecimal integer")),
                                }
                            }
                        }
                        Err(_) => return Err(syn::Error::new_spanned(attr, "Invalid packet attribute format. Use #[packet(code = 0x01)]")),
                    }
                }
                _ => return Err(syn::Error::new_spanned(attr, "Invalid packet attribute format. Use #[packet(code = 0x01)]")),
            }
        }
    }
    
    Err(syn::Error::new_spanned(input, "Missing #[packet(code = ...)] attribute"))
}