struct LitHTML {
    render:js::JSInvoker
}
  
impl Default for LitHTML {
    fn default() -> Self {
        LitHTML {
            render: js::register_function("function(template,dom){
                template = this.getObject(template);
                dom = this.getObject(dom);
                window.LitHTML.render(template,dom);
            }")
        }
    }
}

pub fn render<T,R>(template_result:T,dom:R) where T:Into<f64>,R:Into<f64>{
    let lit_html = globals::get::<LitHTML>();
    lit_html.render.invoke_2(template_result.into(),dom.into());
}


pub trait Template {
    fn execute(&self) -> f64;
}


pub mod js;
pub mod globals;