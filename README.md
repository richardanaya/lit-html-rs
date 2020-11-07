# lit-html-rs

A library for using the HTML template library [lit-html](https://lit-html.polymer-project.org/).

```rust
use lit_html::*;

#[template("<div>Hello ${name}</div>")]
pub struct HelloWorldTemplate {
    pub name: String,
}

#[no_mangle]
pub fn main() {
    let template = HelloWorldTemplate {
        name: "Richard".to_string(),
    };
    let template_result = template.execute();
    render(template_result, js::DOM_BODY);
}
```