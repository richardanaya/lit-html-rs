use lit_html::*;
use web::*;

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

fn todo_item(pos: usize, todo: &todo::Todo) -> Template {
    let mut data = TemplateData::new();
    data.set("text", &*todo.text);
    data.set("completed", todo.completed);
    data.set(
        "toggle_done",
        MouseEventHandler::new(move |_e: MouseEvent| {
            let mut todos = globals::get::<todo::TodoList>();
            todos.items[pos].completed = !todos.items[pos].completed;
            todos.save();
            rerender();
        }),
    );
    data.set(
        "delete",
        MouseEventHandler::new(move |_e: MouseEvent| {
            let mut todos = globals::get::<todo::TodoList>();
            todos.items.remove(pos);
            todos.save();
            rerender();
        }),
    );
    html!(include_str!("./todo_item.html"), &data)
}

fn app() -> Template {
    let app_state = globals::get::<AppState>();
    let todo_list = globals::get::<todo::TodoList>();
    let mut data = TemplateData::new();
    data.set("num_items_todo", todo_list.items.len() as f64);
    data.set(
        "todo_key_down",
        KeyEventHandler::new(|e: KeyEvent| {
            let mut input = InputElement::from(e.target());
            if e.key_code() == 13 {
                let v = input.value();
                if let Some(txt) = v {
                    input.set_value("");
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
            .enumerate()
            .map(|(pos, todo)| todo_item(pos, todo))
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
    // render next chance we get and prevent locks of global mutex
    // in theory you could get around doing this by making sure your global
    // state won't lock up
    set_timeout(
        || {
            render(&app(), DOM_BODY);
        },
        0,
    );
}

#[no_mangle]
pub fn main() {
    rerender();
}

mod todo;
