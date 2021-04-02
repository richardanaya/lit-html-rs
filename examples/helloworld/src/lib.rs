use js::*;
use lit_html::*;

#[no_mangle]
pub fn main() {
    let mut data = TemplateData::new();
    data.set("name", "Ferris");
    render(&html!(r#"<h1>Hello ${_.name}</h1>"#, &data), DOM_BODY);
}
