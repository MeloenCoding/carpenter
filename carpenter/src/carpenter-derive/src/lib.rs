use proc_macro::TokenStream;
use syn::{self, Ident, parse_macro_input, DeriveInput, Fields};
use quote::{quote, ToTokens};


#[proc_macro_derive(ConfigManager)] 
pub fn config_derive_macro(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(item as DeriveInput);
    return impl_config_trait(ast);
}

fn impl_config_trait(ast: syn::DeriveInput) -> TokenStream {
    // Data check
    let data: syn::DataStruct = match ast.data {
        syn::Data::Struct(data) => data,
        syn::Data::Enum(_) => panic!("Enums are not supported by Carpenter"),
        syn::Data::Union(_) => panic!("Unions are not supported by Carpenter"),
    };

    // Props
    let config_id: &Ident = &ast.ident; 

    let settings_name: String = format!("{}Factory", config_id);
    let setting_id: Ident = syn::Ident::new(&settings_name, config_id.span());

    let fields = match data.fields {
        Fields::Named(fields) => fields.named,
        _ => panic!("Only named fields are supported by Carpenter")
    };

    let field_read_statements = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().expect("Field name not found");
        let field_type = &field.ty;

        if field_type.to_token_stream().to_string() == "String" {
            quote! {
                #field_name: {
                    let mut string_buffer = Vec::new();
                    loop {
                        let mut byte = [0; 1];
                        if stream.read_exact(&mut byte).is_err() || byte[0] == 0 {
                            break;
                        }
                        string_buffer.push(byte[0]);
                    }
                    String::from_utf8_lossy(&string_buffer).to_string()
                },
            }
        } else {
            quote! {
                #field_name: #field_type::read_from(stream, order)?,
            }
        }
    });

    let mut field_write_statements = Vec::new();
    for field in &fields {
        let field_name = field.ident.as_ref().expect("Field name not found");

        let field_write_expr;
        if field.ty.to_token_stream().to_string() == "String" {
            field_write_expr = quote! {
                for byte in self.#field_name.bytes() {
                    stream.write_all(&[byte])?;
                }
                stream.write_all(&[0])?;
            };
        } 
        else {
            field_write_expr = quote! {
                self.#field_name.write_to(stream, order)?;
            };

        }
        
        field_write_statements.push(field_write_expr);
    }
    
    
    // Expansion
    let expanded = quote!{
        use carpenter::ConfigPath;
        use std::fs::File;
        use std::io::{Cursor, Write, Read};
        use std::path::PathBuf;
        use bytestream::{StreamWriter, ByteOrder, StreamReader};
        

        struct #setting_id {
            path: PathBuf,
            username: String, 
            application_name: String,
            config_name: String,
        }

        impl #setting_id {
            /// Tries to create all of the directories.
            fn create_dir(&self) -> Result<(), std::io::Error> {
                std::fs::create_dir_all(&self.path)?;
                Ok(())
            }
            
            /// Tries to create the file.
            fn create_file(&self) -> Result<(), std::io::Error> {
                let config_file_path = self.path.join(&self.config_name);
                let config_file = File::create(config_file_path)?;
                Ok(())
            }

            /// Tries to save the struct to the config file.
            fn save(&self, config_struct: &#config_id) -> Result<(), std::io::Error> {
                self.create_dir()?;
                self.create_file()?;

                let config_file_path = self.path.join(&self.config_name);
                let mut buffer = Vec::<u8>::new();
                config_struct.write_to(&mut buffer, ByteOrder::BigEndian)?;

                std::fs::write(config_file_path, buffer)?;
                Ok(())
            }

            /// Tries to read the config file and parse it to the struct.
            fn read(&self) -> Result<#config_id, std::io::Error> {
                let config_file_path = self.path.join(&self.config_name);
                let mut buffer = std::fs::read(config_file_path)?;
                let mut cursor = Cursor::new(buffer);
                return Ok(#config_id::read_from(&mut cursor, ByteOrder::BigEndian)?);
            }
        }

        impl #config_id {
            /// Creates a Builder for this struct.
            /// 
            /// # Example
            /// ```rust
            /// let config_factory = Config::init_config(
            ///    "meloencoding", // Username
            ///    "config-rs-test", // Application name
            ///    "test.bin" // Config file name. File extention is optional
            /// );
            /// ```
            /// On this Builder struct you can call `save()` and `read()`
            /// # Examples
            /// ```rust
            /// // To save your config
            /// let sample_config = Config {
            ///     a: 400,
            ///     b: true,
            ///     c: String::from("Hey"),
            /// };
            /// 
            /// config_factory.save(&sample_config)?;
            /// 
            /// // To read the saved config
            /// assert_eq!(sample_config, config_factory.read()?);
            /// ```
            pub fn init_config(username: &str, application_name: &str, config_name: &str) -> #setting_id {
                let builder = #setting_id {
                    path: PathBuf::from(ConfigPath::new(username, application_name).inner.clone()),
                    username: username.to_string(), 
                    application_name: application_name.to_string(), 
                    config_name: config_name.to_string()
                };
                return builder;
            }
        }
        
        impl StreamReader for #config_id {
            fn read_from<R: Read>(stream: &mut R, order: ByteOrder) -> Result<#config_id, std::io::Error> {
                Ok(#config_id {
                    #(#field_read_statements)*
                })
            }
        }

        impl StreamWriter for #config_id {
            fn write_to<W: Write>(&self, stream: &mut W, order: bytestream::ByteOrder) -> Result<(), std::io::Error> {
                #(#field_write_statements)*
                Ok(())
            }
        }
    };

    return expanded.into();
}