use lit_html::{
    html, render, KeyEvent, KeyEventHandler, MouseEvent, MouseEventHandler, Template, TemplateData,
    TemplateValue,
};
use serde::{Deserialize, Serialize};
use web::{
    console_error, js, local_storage_get_item, local_storage_set_item, set_timeout, InputElement,
    JSObject, DOM_BODY,
};

// DATASTRUCTURES

// First we need a datatstructure for representing our todo list
// They use serde serialization/deserialization to convert to/from JSON

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub completed: bool,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct TodoList {
    pub items: Vec<Todo>,
}

// Let's add some helper functions to store/load from the browser's local storage
impl TodoList {
    pub fn save(&self) {
        match serde_json::to_string(self) {
            Ok(s) => local_storage_set_item("todos", &s),
            Err(_) => console_error("error saving todos to localstorage"),
        };
    }
    pub fn load() -> Option<TodoList> {
        return match local_storage_get_item("todos") {
            Some(s) => match serde_json::from_str(&s) {
                Ok(s) => Some(s),
                Err(_) => {
                    console_error("error loading todos from localstorage");
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

// Our initial todo list should have no item
impl Default for TodoList {
    fn default() -> Self {
        match TodoList::load() {
            Some(tl) => tl,
            None => TodoList { items: vec![] },
        }
    }
}

// Let's create a structure that represents our app's state
struct AppState {
    mode: Mode,
}

enum Mode {
    All,       // Show all todos
    Active,    // Show todos not completed
    Completed, // Show todos that are completed
}

// Our default state for app should show active items only
impl Default for AppState {
    fn default() -> Self {
        AppState { mode: Mode::Active }
    }
}

// FUNCTIONS

// lit-html-rs uses functions to generate a tree of html templates that will be rendered to the browsers DOM

// Okay, let's start our app, the first thing we need to do is render
#[no_mangle]
pub fn main() {
    rerender();
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

// Our first top most component is our app
fn app() -> Template {
    // Our app uses global state of our todo list and app state
    // This basically gets a mutex locked instance of the type
    // and instantiates it if it isn't already instantiated
    let todo_list = globals::get::<TodoList>();
    let app_state = globals::get::<AppState>();

    // lit-html-rs works by create templates and rendering them with data
    let mut data = TemplateData::new();
    // how many todos are there to do
    data.set(
        "num_items_todo",
        todo_list
            .items
            .iter()
            .filter(|todo| !todo.completed)
            .count() as f64,
    );
    // add a handler for whe user hits enter on input
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
    // add a list of child todo elements (see todo component below)
    data.set(
        "todo_items",
        todo_list
            .items
            .iter()
            .enumerate()
            .filter(|(_, todo)| match app_state.mode {
                Mode::All => true,
                Mode::Completed => todo.completed == true,
                Mode::Active => todo.completed == false,
            })
            .map(|(pos, todo)| todo_item(pos, todo))
            .collect::<Vec<Template>>(),
    );
    // handle when user clicks button to show all todos
    data.set(
        "toggle_filter_all",
        MouseEventHandler::new(move |_e: MouseEvent| {
            let mut app = globals::get::<AppState>();
            app.mode = Mode::All;
            rerender();
        }),
    );
    // handle when user clicks button to show only completed todos
    data.set(
        "toggle_filter_completed",
        MouseEventHandler::new(move |_e: MouseEvent| {
            let mut app = globals::get::<AppState>();
            app.mode = Mode::Completed;
            rerender();
        }),
    );
    // handle when user clicks button to show only non-completed todos
    data.set(
        "toggle_filter_active",
        MouseEventHandler::new(move |_e: MouseEvent| {
            let mut app = globals::get::<AppState>();
            app.mode = Mode::Active;
            rerender();
        }),
    );
    match app_state.mode {
        Mode::All => data.set("all_selected_class", "selected"),
        Mode::Active => data.set("active_selected_class", "selected"),
        Mode::Completed => data.set("completed_selected_class", "selected"),
    }
    // render the html with this data
    html!(
        r##"
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
                <a class="${_.all_selected_class}" href="#">All</a>
              </li>
              <li @click="${_.toggle_filter_active}">
                <a class="${_.active_selected_class}"  href="#">Active</a>
              </li>
              <li @click="${_.toggle_filter_completed}">
                <a class="${_.completed_selected_class}" href="#">Completed</a>
              </li>
            </ul>
          </footer>
        </section>
        <footer class="info">
          <p>Want to see the source code? Go <a href="https://github.com/richardanaya/lit-html-rs/blob/master/examples/todo_list/src/lib.rs">here</a></p>
        </footer>
    "##,
        &data
    )
}

// this component renders a todo list item
fn todo_item(pos: usize, todo: &Todo) -> Template {
    // again, its just rendering a template
    let mut data = TemplateData::new();
    // the todo's text
    data.set("text", &*todo.text);
    // should the check mark be checked
    if todo.completed {
        data.set("check_class", "checked");
    }
    // add click handler if the click the completed button
    data.set(
        "toggle_done",
        MouseEventHandler::new(move |_e: MouseEvent| {
            let mut todos = globals::get::<TodoList>();
            todos.items[pos].completed = !todos.items[pos].completed;
            todos.save();
            rerender();
        }),
    );
    // add handler for if they delete the item
    data.set(
        "delete",
        MouseEventHandler::new(move |_e: MouseEvent| {
            let mut todos = globals::get::<TodoList>();
            todos.items.remove(pos);
            todos.save();
            rerender();
        }),
    );
    //render it
    html!(
        r#"<li>
        <div class="view">
        <div class="toggle ${_.check_class}" @click="${_.toggle_done}"></div>
        <label>${_.text}</label>
        <button class="destroy" @click="${_.delete}"></button>
        </div>
    </li>"#,
        &data
    )
}
