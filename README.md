# lit-html-rs

A Rust library for using the HTML template library [lit-html](https://lit-html.polymer-project.org/).

```rust
use js::*;
use lit_html::*;

#[no_mangle]
pub fn main() {
    let data = DataDictionary::new();
    data.setString("name","Richard");
    render(
        html!(r#"<h1>Hello ${_.navigator.appCodeName}</h1>"#, data),
        DOM_BODY,
    );
}
```
