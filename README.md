# lit-html-rs

A Rust library for using the HTML template library [lit-html](https://lit-html.polymer-project.org/) created by the Google Polymer project.

**this library is still in very early stages**

```toml
[dependencies]
lit-html = "0"
```

Here's a demo of the iconic [todo list](https://richardanaya.github.io/lit-html-rs/examples/todo_list/) example and it's [source](https://github.com/richardanaya/lit-html-rs/blob/master/examples/todo_list/src/lib.rs).

# Basics

`lit-html` works by creating templates that efficiently render to the DOM.  When you are building a `TemplateData` object your data is being moved from WebAssembly into an object in JavaScript that can be used by the `lit-html` template.

You can put the following data on TemplateData:
* strings
* numbers
* booleans
* callbacks functions

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

See it working [here](https://richardanaya.github.io/lit-html-rs/examples/helloworld/).

# Counter

You can build up complex UI by creating Templates that contain other data bound templates. `lit-html` efficiently manipulates the DOM when data changes.

```rust
use js::*;
use lit_html::*;

static mut COUNT: u32 = 0;

fn counter() -> Template {
    let mut data = TemplateData::new();
    data.set("count", unsafe { COUNT });
    data.set("increment", || {
        unsafe { COUNT += 1 };
        rerender();
    });
    html!(
        r#"The current count is ${_.count} <button @click="${_.increment}">+</button>"#,
        data
    )
}

fn app() -> Template {
    let mut data = TemplateData::new();
    data.set("content", counter());
    html!(
        r#"<div>This is a counter in Rust!</div><div>${_.content}</div>"#,
        data
    )
}

fn rerender() {
    render(&app(), DOM_BODY);
}

#[no_mangle]
pub fn main() {
    rerender();
}
```

See it working [here](https://richardanaya.github.io/lit-html-rs/examples/counter/).
