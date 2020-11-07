extern crate proc_macro;
use proc_macro::TokenStream;
use std::str::FromStr;
use syn::export::ToTokens;
use syn::{parse_macro_input, Fields, ItemStruct};

#[proc_macro_attribute]
pub fn template(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let template_exp = metadata.to_string();
    let original_struct = input.to_string();
    let input = parse_macro_input!(input as ItemStruct);
    let struct_name = input.ident.to_string();
    let mut fields: Vec<(String, String)> = vec![];
    if let Fields::Named(n) = input.fields {
        fields = n
            .named
            .iter()
            .map(|x| {
                (
                    x.ident.as_ref().unwrap().to_string(),
                    x.ty.clone().into_token_stream().to_string(),
                )
            })
            .collect();
    }

    let mut args: Vec<String> = vec![];
    let mut extractor_exprs: Vec<String> = vec![];
    let mut execute_exprs: Vec<String> = vec![];
    let mut c = 0;
    for (field_name, field_type) in fields.iter() {
        if field_type == "String" {
            extractor_exprs.push(
                [
                    "const ",
                    field_name,
                    " = this.readUtf8FromMemory(a",
                    &c.to_string(),
                    ",a",
                    &(c + 1).to_string(),
                    ");",
                ]
                .join(""),
            );
            execute_exprs.push(
                [
                    "let a",
                    &c.to_string(),
                    " = self.",
                    field_name,
                    ".as_ptr() as u32;\nlet a",
                    &(c + 1).to_string(),
                    " = self.",
                    field_name,
                    ".len() as u32;",
                ]
                .join(""),
            );
            args.push(["a", &c.to_string()].join(""));
            c += 1;
            args.push(["a", &c.to_string()].join(""));
            c += 1;
        } else {
            panic!("unsupported type");
        }
    }
    let args = args.join(",");
    let extractor_exprs = extractor_exprs.join("\n");
    let execute_exprs = execute_exprs.join("\n");

    TokenStream::from_str(&format!(r#"
    {}
    
    struct {}Builder {{
        fn_builder: lit_html::js::JSInvoker,
    }}
    
    impl Default for {}Builder {{
        fn default() -> Self {{
            let mut fn_text = &["function builder(","{}","){{\n","{}","const result = window.LitHtml.html`",{},"`;\nreturn this.storeObject(result);\n}}"].join("");
            {}Builder {{
                fn_builder: lit_html::js::register_function(&fn_text),
            }}
        }}
    }}
    
    impl lit_html::Template for {} {{
        fn execute(&self) -> f64 {{
            let builder = globals::get::<{}Builder>();
            {}
            builder.fn_builder.invoke_{}({})
        }}
    }}
    "#,original_struct,struct_name,struct_name,args,extractor_exprs,template_exp,struct_name,struct_name,struct_name,execute_exprs,c,args)).unwrap()
}
