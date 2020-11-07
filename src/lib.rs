struct LitHtml {
    render:js::JSInvoker
}
  
impl Default for LitHtml {
    fn default() -> Self {
        LitHtml {
            render: js::register_function("function(template,dom){
                template = this.getObject(template);
                dom = this.getObject(dom);
                window.LitHtml.render(template,dom);
            }")
        }
    }
}

pub fn render<T,R>(template_result:T,dom:R) where T:Into<f64>,R:Into<f64>{
    let lit_html = globals::get::<LitHtml>();
    lit_html.render.invoke_2(template_result.into(),dom.into());
}


pub trait Template {
    fn execute(&self) -> f64;
}


pub mod js;
pub mod globals;