use lit_html::*;
use serde::{Deserialize, Serialize};
use web::*;

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub completed: bool,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct TodoList {
    pub items: Vec<Todo>,
}

impl TodoList {
    pub fn save(&self) {
        match serde_json::to_string(self) {
            Ok(s) => local_storage_set_item("todos", &s),
            Err(_) => console_error("error serializing todos"),
        };
    }
    pub fn load() -> Option<TodoList> {
        return match local_storage_get_item("todos") {
            Some(s) => match serde_json::from_str(&s) {
                Ok(s) => Some(s),
                Err(_) => {
                    console_error("error parsing todos");
                    None
                }
            },
            None => None,
        };
    }
    pub fn add(&mut self, txt: &str) {
        self.items.push(Todo {
            text: txt.to_owned(),
            completed: false,
        });
    }
}

impl Default for TodoList {
    fn default() -> Self {
        match TodoList::load() {
            Some(tl) => tl,
            None => TodoList { items: vec![] },
        }
    }
}

enum Mode {
    All,
    Active,
    Completed,
}

struct AppState {
    mode: Mode,
}

impl Default for AppState {
    fn default() -> Self {
        AppState { mode: Mode::Active }
    }
}

fn todo_item(pos: usize, todo: &Todo) -> Template {
    let mut data = TemplateData::new();
    data.set("text", &*todo.text);
    data.set("completed", todo.completed);
    data.set(
        "toggle_done",
        MouseEventHandler::new(move |_e: MouseEvent| {
            let mut todos = globals::get::<TodoList>();
            todos.items[pos].completed = !todos.items[pos].completed;
            todos.save();
            rerender();
        }),
    );
    data.set(
        "delete",
        MouseEventHandler::new(move |_e: MouseEvent| {
            let mut todos = globals::get::<TodoList>();
            todos.items.remove(pos);
            todos.save();
            rerender();
        }),
    );
    html!(
        r#"<li>
        <div class="view">
        <input class="toggle" type="checkbox" @click="${_.toggle_done}" ?checked="${_.completed}"/>
        <label>${_.text}</label>
        <button class="destroy" @click="${_.delete}"></button>
        </div>
    </li>"#,
        &data
    )
}

fn app() -> Template {
    let todo_list = globals::get::<TodoList>();
    let app_state = globals::get::<AppState>();
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
                    let mut todos = globals::get::<TodoList>();
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
            .filter(|todo| match app_state.mode {
                Mode::All => true,
                Mode::Completed => todo.completed == true,
                Mode::Active => todo.completed == false,
            })
            .enumerate()
            .map(|(pos, todo)| todo_item(pos, todo))
            .collect::<Vec<Template>>(),
    );
    data.set(
        "toggle_filter_all",
        MouseEventHandler::new(move |_e: MouseEvent| {
            let mut app = globals::get::<AppState>();
            app.mode = Mode::All;
            rerender();
        }),
    );
    data.set(
        "toggle_filter_completed",
        MouseEventHandler::new(move |_e: MouseEvent| {
            let mut app = globals::get::<AppState>();
            app.mode = Mode::Completed;
            rerender();
        }),
    );
    data.set(
        "toggle_filter_active",
        MouseEventHandler::new(move |_e: MouseEvent| {
            let mut app = globals::get::<AppState>();
            app.mode = Mode::Active;
            rerender();
        }),
    );
    html!(
        r##"<!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <title>Rust Todo</title>
        <link
          rel="stylesheet"
          href="https://cdn.jsdelivr.net/npm/todomvc-app-css@2.3.0/index.css"
        />
      </head>
      <body>
        <section class="todoapp">
          <header class="header">
            <h1>todos</h1>
            <input
              @keydown="${_.todo_key_down}"
              class="new-todo"
              placeholder="What needs to be done?"
              autofocus
            />
          </header>
          <section class="main">
            <input id="toggle-all" class="toggle-all" type="checkbox" />
            <label for="toggle-all">Mark all as complete</label>
            <ul class="todo-list">
              ${_.todo_items}
            </ul>
          </section>
          ${_.content}
          <footer class="footer">
            <span class="todo-count"
              ><strong>${_.num_items_todo}</strong> item left</span
            >
            <ul class="filters">
              <li @click="${_.toggle_filter_all}">
                <a class="selected" href="#/">All</a>
              </li>
              <li @click="${_.toggle_filter_active}">
                <a href="#/active">Active</a>
              </li>
              <li @click="${_.toggle_filter_completed}">
                <a href="#/completed">Completed</a>
              </li>
            </ul>
          </footer>
        </section>
        <footer class="info">
          <p>Double-click to edit a todo</p>
        </footer>
      </body>
    </html>
    "##,
        &data
    )
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
