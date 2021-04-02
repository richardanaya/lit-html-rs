#![no_std]
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
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
    fn set(self, data: &mut TemplateData, name: &str);
}

impl TemplateValue for &str {
    fn set(self, data: &mut TemplateData, name: &str) {
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
    fn set(self, data: &mut TemplateData, name: &str) {
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

impl TemplateValue for Template {
    fn set(self, data: &mut TemplateData, name: &str) {
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

impl TemplateValue for alloc::vec::Vec<Template> {
    fn set(self, data: &mut TemplateData, name: &str) {
        let a = JSObject::from(
            js!("function(){
            return this.storeObject([]);
        }")
            .invoke_0(),
        );
        for t in self {
            js!("function(o,v){
                this.getObject(o).push(this.getObject(v));
            }")
            .invoke_2(a.handle, t.handle);
        }
        js!("function(o,n,nlen,v){
            this.getObject(o)[this.readUtf8FromMemory(n,nlen)] = this.getObject(v);
        }")
        .invoke_4(
            data.obj.handle,
            name.as_ptr() as u32,
            name.len() as u32,
            a.handle,
        );
    }
}

impl TemplateValue for &Template {
    fn set(self, data: &mut TemplateData, name: &str) {
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
    fn set(self, data: &mut TemplateData, name: &str) {
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
    fn set(self, data: &mut TemplateData, name: &str) {
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
    fn set(self, data: &mut TemplateData, name: &str) {
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
    fn set(self, data: &mut TemplateData, name: &str) {
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

impl TemplateValue for KeyEventHandler {
    fn set(mut self, data: &mut TemplateData, name: &str) {
        let mut f = self.handler.take().unwrap();
        js!("function(o,n,nlen,v){
            this.getObject(o)[this.readUtf8FromMemory(n,nlen)] = this.createCallback(v);
        }")
        .invoke_4(
            data.obj.handle,
            name.as_ptr() as u32,
            name.len() as u32,
            create_callback_1(move |v| f(KeyEvent::new(v))),
        );
    }
}

impl TemplateValue for MouseEventHandler {
    fn set(mut self, data: &mut TemplateData, name: &str) {
        let mut f = self.handler.take().unwrap();
        js!("function(o,n,nlen,v){
            this.getObject(o)[this.readUtf8FromMemory(n,nlen)] = this.createCallback(v);
        }")
        .invoke_4(
            data.obj.handle,
            name.as_ptr() as u32,
            name.len() as u32,
            create_callback_1(move |v| f(MouseEvent::new(v))),
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

    pub fn set(&mut self, name: &str, value: impl TemplateValue) {
        value.set(self, name);
    }
}

impl Into<f64> for &TemplateData {
    fn into(self) -> f64 {
        self.obj.handle
    }
}

pub struct KeyEventHandler {
    handler: Option<Box<dyn Sync + FnMut(KeyEvent) + 'static + Send>>,
}

impl KeyEventHandler {
    pub fn new(f: impl Sync + FnMut(KeyEvent) + 'static + Send) -> KeyEventHandler {
        KeyEventHandler {
            handler: Some(Box::new(f)),
        }
    }
}

pub struct KeyEvent {
    obj: JSObject,
}

impl KeyEvent {
    pub fn new(o: f64) -> KeyEvent {
        KeyEvent {
            obj: JSObject::from(o),
        }
    }

    pub fn key_code(&self) -> usize {
        js!("function(o){
            return this.getObject(o).keyCode;
        }")
        .invoke_1(self.obj.handle) as usize
    }

    pub fn target(&self) -> JSObject {
        let r = js!("function(o){
            return this.storeObject(this.getObject(o).target);
        }")
        .invoke_1(self.obj.handle);
        JSObject::from(r)
    }
}

pub struct InputElement {
    obj: JSObject,
}

impl InputElement {
    pub fn new(o: f64) -> InputElement {
        InputElement {
            obj: JSObject::from(o),
        }
    }

    pub fn from(o: JSObject) -> InputElement {
        InputElement { obj: o }
    }

    pub fn value(&self) -> Option<String> {
        get_property_string(&self.obj, "value")
    }

    pub fn set_value(&mut self, s: &str) {
        set_property_string(&self.obj, "value", s)
    }
}

pub fn get_property_string(el: impl Into<f64>, name: &str) -> Option<alloc::string::String> {
    let attr = js!(r#"function(o,strPtr,strLen){
        o = this.getObject(o);
        const a = o[this.readUtf8FromMemory(strPtr,strLen)];
        if(a === null){
            return -1;
        } 
        return this.writeCStringToMemory(a);
    }"#)
    .invoke_3(el.into(), name.as_ptr() as u32, name.len() as u32);
    if attr == -1.0 {
        return None;
    } else {
        Some(cstr_to_string(attr as i32))
    }
}

pub fn set_property_string(el: impl Into<f64>, name: &str, txt: &str) {
    js!(r#"function(o,strPtr,strLen,valPtr,valLen){
        o = this.getObject(o);
        o[this.readUtf8FromMemory(strPtr,strLen)] = this.readUtf8FromMemory(valPtr,valLen);
    }"#)
    .invoke_5(
        el.into(),
        name.as_ptr() as u32,
        name.len() as u32,
        txt.as_ptr() as u32,
        txt.len() as u32,
    );
}

pub struct MouseEventHandler {
    handler: Option<Box<dyn Sync + FnMut(MouseEvent) + 'static + Send>>,
}

impl MouseEventHandler {
    pub fn new(f: impl Sync + FnMut(MouseEvent) + 'static + Send) -> MouseEventHandler {
        MouseEventHandler {
            handler: Some(Box::new(f)),
        }
    }
}

pub struct MouseEvent {
    obj: JSObject,
}

impl MouseEvent {
    pub fn new(o: f64) -> MouseEvent {
        MouseEvent {
            obj: JSObject::from(o),
        }
    }
    pub fn target(&self) -> JSObject {
        let r = js!("function(o){
            return this.storeObject(this.getObject(o).target);
        }")
        .invoke_1(self.obj.handle);
        JSObject::from(r)
    }
}
