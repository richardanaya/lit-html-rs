#![no_std]
extern crate alloc;
use js::*;

pub fn render<R>(template_result: &Template, dom: R)
where
    R: Into<f64>,
{
    let r = js!("function(template,dom){
        template = this.getObject(template);
        dom = this.getObject(dom);
        window.LitHtml.render(template,dom);
    }");
    r.invoke_2(template_result.handle, dom.into());
}

#[macro_export]
macro_rules! html {
    ($e:expr,$d:expr) => {{
        JSObject::from(js!(&[
            r#"function(_){
                _ = this.getObject(_);
                return this.storeObject(window.LitHtml.html`"#,
            $e,
            r#"`);
            }"#
        ]
        .concat())
        .invoke_1($d))
    }};
    ($e:expr) => {{
        JSObject::from(js!(&[
            r#"function(_){
                _ = this.getObject(_);
                return this.storeObject(window.LitHtml.html`"#,
            $e,
            r#"`);
            }"#
        ]
        .concat())
        .invoke_0())
    }};
}

pub type Template = JSObject;

pub trait TemplateValue {
    fn set(self, data: &TemplateData, name: &str);
}

impl TemplateValue for &str {
    fn set(self, data: &TemplateData, name: &str) {
        js!("function(o,n,nlen,v,vlen){
            this.getObject(o)[this.readUtf8FromMemory(n,nlen)] = this.readUtf8FromMemory(v,vlen);
        }")
        .invoke_5(
            data.obj.handle,
            name.as_ptr() as u32,
            name.len() as u32,
            self.as_ptr() as u32,
            self.len() as u32,
        );
    }
}

impl TemplateValue for f64 {
    fn set(self, data: &TemplateData, name: &str) {
        js!("function(o,n,nlen,v){
            this.getObject(o)[this.readUtf8FromMemory(n,nlen)] = v;
        }")
        .invoke_4(
            data.obj.handle,
            name.as_ptr() as u32,
            name.len() as u32,
            self,
        );
    }
}

impl TemplateValue for &Template {
    fn set(self, data: &TemplateData, name: &str) {
        js!("function(o,n,nlen,v){
            this.getObject(o)[this.readUtf8FromMemory(n,nlen)] = this.getObject(v);
        }")
        .invoke_4(
            data.obj.handle,
            name.as_ptr() as u32,
            name.len() as u32,
            self.handle,
        );
    }
}

impl TemplateValue for u32 {
    fn set(self, data: &TemplateData, name: &str) {
        js!("function(o,n,nlen,v){
            this.getObject(o)[this.readUtf8FromMemory(n,nlen)] = v;
        }")
        .invoke_4(
            data.obj.handle,
            name.as_ptr() as u32,
            name.len() as u32,
            self,
        );
    }
}

impl TemplateValue for bool {
    fn set(self, data: &TemplateData, name: &str) {
        js!("function(o,n,nlen,v){
            this.getObject(o)[this.readUtf8FromMemory(n,nlen)] = v>0;
        }")
        .invoke_4(
            data.obj.handle,
            name.as_ptr() as u32,
            name.len() as u32,
            if self { 1.0 } else { 0.0 },
        );
    }
}

impl TemplateValue for i32 {
    fn set(self, data: &TemplateData, name: &str) {
        js!("function(o,n,nlen,v){
            this.getObject(o)[this.readUtf8FromMemory(n,nlen)] = v;
        }")
        .invoke_4(
            data.obj.handle,
            name.as_ptr() as u32,
            name.len() as u32,
            self,
        );
    }
}

impl<T> TemplateValue for T
where
    T: Sync + FnMut() + 'static + Send,
{
    fn set(self, data: &TemplateData, name: &str) {
        js!("function(o,n,nlen,v){
            this.getObject(o)[this.readUtf8FromMemory(n,nlen)] = this.createCallback(v);
        }")
        .invoke_4(
            data.obj.handle,
            name.as_ptr() as u32,
            name.len() as u32,
            create_callback_0(self),
        );
    }
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

    pub fn set(&self, name: &str, value: impl TemplateValue) {
        value.set(self, name);
    }
}

impl Into<f64> for &TemplateData {
    fn into(self) -> f64 {
        self.obj.handle
    }
}
