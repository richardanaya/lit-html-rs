# lit-html-rs

A Rust library for using the HTML template library [lit-html](https://lit-html.polymer-project.org/).

```rust
use js::*;
use lit_html::*;

#[no_mangle]
pub fn main() {
    let data = TemplateData::new();
    data.set("name", "Ferris");
    render(html!(r#"<h1>Hello ${_.name}</h1>"#, &data), DOM_BODY);
}
```
