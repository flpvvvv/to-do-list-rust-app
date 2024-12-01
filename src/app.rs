use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Todo {
    id: String,
    text: String,
    completed: bool,
}

#[component]
pub fn App() -> impl IntoView {
    let (todos, set_todos) = create_signal(Vec::<Todo>::new());

    let fetch_todos = move || {
        spawn_local(async move {
            let fetched: Vec<Todo> = reqwest::get("/api/list")
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            set_todos.set(fetched);
        });
    };

    let add_todo = move |text: String| {
        let new_todo = Todo {
            id: uuid::Uuid::new_v4().to_string(),
            text,
            completed: false,
        };

        spawn_local(async move {
            reqwest::Client::new()
                .post("/api/add")
                .json(&new_todo)
                .send()
                .await
                .unwrap();
            fetch_todos();
        });
    };

    // Fetch todos on mount
    fetch_todos();

    view! {
        <div>
            <h1>"To-Do List"</h1>
            <ul>
                {move || todos.get().iter().map(|todo| {
                    view! { <li>{&todo.text}</li> }
                }).collect::<Vec<_>>()}
            </ul>
            <input
                id="new-todo"
                placeholder="Add a new task"
                on:keypress=move |ev| {
                    let input = ev.target().unwrap().unchecked_into::<HtmlInputElement>();
                    if ev.key() == "Enter" {
                        add_todo(input.value());
                        input.set_value("");
                    }
                }
            />
        </div>
    }
}

pub fn render_app() -> String {
    leptos::ssr::render_to_string(|| App()).to_string()
}
