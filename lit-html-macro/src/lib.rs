extern crate proc_macro;
use proc_macro::{TokenStream};
use std::str::FromStr;
use syn::{parse_macro_input, ItemStruct,Fields};
use syn::export::ToTokens;

#[proc_macro_attribute]
pub fn template(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let template_exp = metadata.to_string();
    let original_struct = input.to_string();
    let input = parse_macro_input!(input as ItemStruct);
    let struct_name = input.ident.to_string();
    let mut _fields:Vec<(String,String)> = vec![];
    if let Fields::Named(n) = input.fields {
        _fields = n.named.iter().map(|x|(x.ident.as_ref().unwrap().to_string(),x.ty.clone().into_token_stream().to_string())).collect(); 
    } 

    let args = r#"a1,a2"#;


    let extractor_exprs = r#"const name = this.readUtf8FromMemory(a1,a2);"#;

    let execute_exprs = r#"
    let a1 = self.name.as_ptr() as u32;
    let a2 = self.name.len() as u32;"#;

    TokenStream::from_str(&format!(r#"
    {}
    
    struct {}Builder {{
        fn_builder: lit_html::js::JSInvoker,
    }}
    
    impl Default for {}Builder {{
        fn default() -> Self {{
            let mut fn_text = &["function(","{}","){{\n","{}","const result = window.LitHtml.html`",{},"`;\nreturn this.storeObject(result);\n}}"].join(",");
            {}Builder {{
                fn_builder: lit_html::js::register_function(&fn_text),
            }}
        }}
    }}
    
    impl lit_html::Template for {} {{
        fn execute(&self) -> f64 {{
            let builder = globals::get::<{}Builder>();
            {}
            builder.fn_builder.invoke_2(a1, a2)
        }}
    }}
    "#,original_struct,struct_name,struct_name,args,extractor_exprs,template_exp,struct_name,struct_name,struct_name,execute_exprs)).unwrap()
}