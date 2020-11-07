use lit_html::*;

pub struct HelloWorldTemplate {
    pub name: String,
}

struct HelloWorldTemplateBuilder {
    fn_builder: js::JSInvoker,
}

impl Default for HelloWorldTemplateBuilder {
    fn default() -> Self {
        HelloWorldTemplateBuilder {
            fn_builder: js::register_function(
                "function(a1,a2){
                const name = this.readUtf8FromMemory(a1,a2);
                const result = window.LitHTML.html`<div>Hello ${name}</div>`;
                return this.storeObject(result);
            }",
            ),
        }
    }
}

impl lit_html::Template for HelloWorldTemplate {
    fn execute(&self) -> f64 {
        let builder = globals::get::<HelloWorldTemplateBuilder>();
        let a1 = self.name.as_ptr() as u32;
        let a2 = self.name.len() as u32;
        builder.fn_builder.invoke_2(a1, a2)
    }
}


#[no_mangle]
pub fn main() {
    let template = HelloWorldTemplate {
        name: "Richard".to_string(),
    };
    let template_result = template.execute();
    render(template_result, js::DOM_BODY);
}
