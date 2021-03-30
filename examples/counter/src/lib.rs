use js::*;
use lit_html::*;

static mut COUNT: u32 = 0;

fn counter() -> Template {
    let data = TemplateData::new();
    data.set("count", unsafe { COUNT });
    data.set("increment", || {
        unsafe { COUNT += 1 };
        rerender();
    });
    html!(
        r#"The current count is ${_.count} <button @click="${_.increment}">+</button>"#,
        &data
    )
}

fn app() -> Template {
    let data = TemplateData::new();
    data.set("content", &counter());
    html!(
        r#"<div>This is a counter in Rust!</div><div>${_.content}</div>"#,
        &data
    )
}

fn rerender() {
    render(&app(), DOM_BODY);
}

#[no_mangle]
pub fn main() {
    rerender();
}
