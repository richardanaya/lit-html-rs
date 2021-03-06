use lit_html::*;

#[template("<div>Hello ${name}!!</div>")]
pub struct HelloWorldTemplate {
    pub name: String,
}

#[no_mangle]
pub fn main() {
    let template = HelloWorldTemplate {
        name: "Richard".to_string(),
    };
    render(template.execute(), js::DOM_BODY);
}
