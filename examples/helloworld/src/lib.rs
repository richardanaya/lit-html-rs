use js::*;
use lit_html::*;

#[no_mangle]
pub fn main() {
    render(
        html!(r#"<h1>Hello ${_.navigator.appCodeName}</h1>"#, 2),
        DOM_BODY,
    );
}
