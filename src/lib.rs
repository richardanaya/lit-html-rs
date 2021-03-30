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

pub struct TemplateData {
    obj: JSObject,
}

impl TemplateData {
    pub fn new() -> TemplateData {
        TemplateData {
            obj: JSObject::from(js!("function(){return this.storeObject({});}").invoke_0()),
        }
    }

    pub fn set(&self, name: &str, value: &str) {
        js!("function(o,n,nlen,v,vlen){
            this.getObject(o)[this.readUtf8FromMemory(n,nlen)] = this.readUtf8FromMemory(v,vlen);
        }")
        .invoke_5(
            self.obj.handle,
            name.as_ptr() as u32,
            name.len() as u32,
            value.as_ptr() as u32,
            value.len() as u32,
        );
    }
}

impl Into<f64> for &TemplateData {
    fn into(self) -> f64 {
        self.obj.handle
    }
}
