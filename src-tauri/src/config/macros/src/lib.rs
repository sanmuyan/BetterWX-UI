use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;
use syn::parse_macro_input;

#[proc_macro_derive(ImpConfigVecIsEmptyTrait)]
pub fn impl_config_vec_is_empty_trait(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl utils::empty::Empty for #name {
            fn is_empty(&self) -> bool {
                self.0.is_empty()
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(FieldNameGetters)]
pub fn derive_name_getters(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    TokenStream::from(quote! {
        impl #name {
            pub fn get_name(&self) -> &str {
                 if self.name.is_empty() {
                    self.code.as_str()
                } else {
                    self.name.as_str()
                }
            }
        }
    })
}

#[proc_macro_derive(FieldDescGetters)]
pub fn derive_desc_getters(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    TokenStream::from(quote! {
        impl #name {
            pub fn get_description(&self) -> &str {
                 if self.description.is_empty() {
                    self.get_name()
                } else {
                    self.description.as_str()
                }
            }
        }
    })
}

#[proc_macro_derive(ImpConfigVecWrapperTrait)]
pub fn derive_impl_config_vec_wrapper_trait(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // 获取内部类型名称（元组结构体只有一个字段）
    // 提取元组结构体的Vec内部类型
    let item_type = match extract_vec_item_type(&input.data) {
        Ok(ty) => ty,
        Err(err) => panic!("{}", err),
    };

    TokenStream::from(quote! {
        impl crate::ConfigVecWrapperTrait for #name {
            type Item = #item_type;
            fn get(&self, code: &str) -> crate::errors::Result<&Self::Item> {
                self.find(code).ok_or(crate::errors::ConfigError::GetVecItemNotFindByCode(code.to_string()).into())
            }

            fn get_mut(&mut self, code: &str) -> crate::errors::Result<&mut Self::Item> {
                self.find_mut(code).ok_or(crate::errors::ConfigError::GetVecItemNotFindByCode(code.to_string()).into())
            }

            fn find(&self, code: &str) -> Option<&Self::Item> {
                self.0.iter().find(|f| f.code == code)
            }

            fn find_mut(&mut self, code: &str) -> Option<&mut Self::Item> {
                self.0.iter_mut().find(|f| f.code == code)
            }

            fn take(&mut self, code: &str) -> crate::errors::Result<Self::Item> {
                let index = self.0.iter().position(|f| f.code == code).ok_or(crate::errors::ConfigError::GetVecItemNotFindByCode(code.to_string()))?;
                Ok(self.0.remove(index))
            }

            fn push(&mut self, item: Self::Item) {
                self.0.push(item)
            }

            fn len(&self) -> usize {
                self.0.len()
            }

            fn clear(&mut self)  {
                self.0.clear()
            }
        }
    })
}

#[proc_macro_derive(SortedSerializeByIndex)]
pub fn derive_sorted_serialize_by_index(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let item_type = match extract_vec_item_type(&input.data) {
        Ok(ty) => ty,
        Err(err) => panic!("{}", err),
    };

    let expanded = quote! {
        impl<'de> serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let mut items: Vec<#item_type> = Vec::deserialize(deserializer)?;
                items.sort_by_key(|item| item.index);
                Ok(#name(items))
            }
        }

        impl serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut sorted = Vec::with_capacity(self.0.len());
                sorted.extend(self.0.iter());
                sorted.sort_by_key(|item| item.index);
                sorted.serialize(serializer)
            }
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(SortedDeserializeByVersionDesc)]
pub fn derive_sorted_deserialize_by_version(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let item_type = match extract_vec_item_type(&input.data) {
        Ok(ty) => ty,
        Err(err) => panic!("{}", err),
    };

    let expanded = quote! {
         impl #name {
            pub fn take_first_less_by_version(&mut self, version: &str) -> crate::errors::Result<#item_type> {
                let v = utils::version::Version::new(version);
                // 找到第一个版本小于等于目标版本的group
                if let Some(index) = self.0.iter().position(|group| v >= group.version) {
                    // 移除并返回匹配的group
                    Ok(self.0.remove(index))
                } else {
                    Err(crate::errors::ConfigError::TakeByVersionError(version.to_string()).into())
                }
            }
        }

        impl<'de> serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) ->std::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let mut items: Vec<#item_type> = Vec::deserialize(deserializer)?;
                items.sort_by(|a, b| b.version.cmp(&a.version));
                Ok(#name(items))
            }
        }

        impl utils::empty::Empty for #item_type {
            fn is_empty(&self) -> bool {
                self.version.is_empty()
            }
        }
    };
    TokenStream::from(expanded)
}
// 提取Vec内部类型的辅助函数
fn extract_vec_item_type(data: &syn::Data) -> Result<syn::Type, &'static str> {
    let fields = match data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Unnamed(fields),
            ..
        }) => fields,
        _ => return Err("只能用于元组结构体"),
    };

    let field = fields.unnamed.first().ok_or("元组结构体必须包含一个字段")?;

    let path = match &field.ty {
        syn::Type::Path(path) => path,
        _ => return Err("无法解析类型"),
    };

    let segment = path.path.segments.last().ok_or("无法解析类型路径")?;
    if segment.ident != "Vec" {
        return Err("元组结构体的字段必须是 Vec 类型");
    }

    match &segment.arguments {
        syn::PathArguments::AngleBracketed(args) => {
            if let Some(syn::GenericArgument::Type(item_ty)) = args.args.first() {
                Ok(item_ty.clone())
            } else {
                Err("Vec 必须指定类型参数")
            }
        }
        _ => Err("Vec 必须指定类型参数"),
    }
}
