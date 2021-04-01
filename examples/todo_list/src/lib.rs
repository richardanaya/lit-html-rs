use lit_html::*;
use web::*;

static mut COUNT: u32 = 0;

enum Mode {
    All,
    Active,
    Completed,
}

struct AppState {
    cleared: bool,
    mode: Mode,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            cleared: false,
            mode: Mode::Active,
        }
    }
}

fn todo_item(i: &todo::Todo) -> Template {
    let data = TemplateData::new();
    data.set("text", &*i.text);
    html!(include_str!("./todo_item.html"), &data)
}

fn app() -> Template {
    let app_state = globals::get::<AppState>();
    let todo_list = globals::get::<todo::TodoList>();
    let data = TemplateData::new();
    data.set("num_items_todo", todo_list.items.len() as f64);
    data.set(
        "todo_key_down",
        KeyHandler::new(|e: KeyEvent| {
            let input = InputElement::from(e.target());
            if e.key_code() == 13 {
                let v = input.value();
                if let Some(txt) = v {
                    let mut todos = globals::get::<todo::TodoList>();
                    todos.add(&txt);
                    todos.save();
                }
            }
            rerender();
        }),
    );
    data.set(
        "todo_items",
        todo_list
            .items
            .iter()
            .map(|i| todo_item(i))
            .collect::<Vec<Template>>(),
    );
    if app_state.cleared {
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
