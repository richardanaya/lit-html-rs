# lit-html-rs

A Rust library for using the HTML template library [lit-html](https://lit-html.polymer-project.org/).

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
    render(template.execute(), js::DOM_BODY);
}
```
