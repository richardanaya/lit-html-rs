#![no_std]
use js::*;

pub fn render<T, R>(template_result: T, dom: R)
where
    T: Into<f64>,
    R: Into<f64>,
{
    let r = js!("function(template,dom){
        template = this.getObject(template);
        dom = this.getObject(dom);
        window.LitHtml.render(template,dom);
    }");
    r.invoke_2(template_result.into(), dom.into());
}

#[macro_export]
macro_rules! html {
    ($e:expr,$d:expr) => {{
        js!(&[
            r#"function(_){
                _ = this.getObject(_);
                return this.storeObject(window.LitHtml.html`"#,
            $e,
            r#"`);
            }"#
        ]
        .concat())
        .invoke_1($d)
    }};
}
