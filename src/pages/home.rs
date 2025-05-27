use crate::components::form::TodoForm;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::*;
use web_sys::HtmlButtonElement;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Todo {
    id: usize,
    title: RwSignal<String>,
    is_complete: RwSignal<bool>,
}

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let (counters, set_counters) = signal::<Vec<Todo>>(vec![
        Todo {
            id: 0,
            title: RwSignal::new("Todo 1".to_string()),
            is_complete: RwSignal::new(false),
        },
        Todo {
            id: 1,
            title: RwSignal::new("Todo 2".to_string()),
            is_complete: RwSignal::new(false),
        },
    ]);

    view! {
      <div class="grid gap-2 p-4 mx-auto max-w-2xl">

        <TodoForm on_submit={
          let set_counters = set_counters.clone();
          move |title| {
            if title.is_empty() {
              logging::log!("Title cannot be empty");
              return;
            }
            let new_todo = Todo {
              id: counters.get().len(),
              title: RwSignal::new(title),
              is_complete: RwSignal::new(false),
            };
            set_counters
              .update(move |todos| {
                todos.push(new_todo);
              });
          }
        } />

        <For
          each=move || counters.get()
          key=|counter| counter.id
          let(Todo { title, is_complete, id })
        >
          <div
            on:click=move |e: ev::MouseEvent| {
              if let Some(btn) = e.target().and_then(|t| t.dyn_into::<HtmlButtonElement>().ok()) {
                if let Ok(value) = btn.id().parse::<usize>() {
                  logging::log!("Button clicked with value: {value}");
                  set_counters
                    .update(move |todos| {
                      if let Some(todo) = todos.iter_mut().find(|todo| todo.id == value) {
                        *todo.is_complete.write() = !todo.is_complete.get();
                      }
                    });
                } else {
                  logging::log!("Failed to parse button value");
                }
              }
            }
            id=move || id.to_string()
            class="flex gap-2 justify-between items-center p-2 rounded-xl transition-colors bg-stone-900 hover:bg-gray-950"
          >
            <h1 class="text-lg font-semibold">{move || title.get()}</h1>
            <div class=move || {
              if is_complete.get() {
                "py-1 px-2 text-sm font-medium bg-green-100/20 rounded-full text-green-500 cursor-pointer"
              } else {
                "py-1 px-2 text-sm font-medium bg-red-100/20 rounded-full text-red-500 cursor-pointer"
              }
            }>{move || if is_complete.get() { " (Completed)" } else { " (Pending)" }}</div>
          </div>
        </For>

      </div>
    }
}
