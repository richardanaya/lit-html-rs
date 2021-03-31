use lit_html::*;
use web::*;

static mut COUNT: u32 = 0;

fn counter() -> Template {
    let data = TemplateData::new();
    data.set("count", unsafe { COUNT });
    data.set("increment", || {
        unsafe {
            COUNT += 1;
            local_storage_set_item("count", &COUNT.to_string());
            let todos = globals::get::<todo::TodoList>();
            todos.save();
        };
        rerender();
    });
    html!(
        r#"The current count is ${_.count} <button @click="${_.increment}">+</button>"#,
        &data
    )
}

fn app() -> Template {
    let data = TemplateData::new();
    data.set("content", counter());
    data.set("num_items_todo", 42);
    if true {
        data.set(
            "cleared_content",
            html!(r#"<button class="clear-completed">Clear completed</button>"#),
        );
    }
    html!(include_str!("./app.html"), &data)
}

fn rerender() {
    render(&app(), DOM_BODY);
}

#[no_mangle]
pub fn main() {
    unsafe {
        if let Some(n) = local_storage_get_item("count") {
            COUNT = n.parse::<u32>().unwrap();
        }
    };
    rerender();
}

mod todo;
