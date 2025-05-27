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
    let (counters, set_counters) = signal::<Vec<Todo>>(
        (1..=5)
            .map(|i| Todo {
                id: i,
                title: RwSignal::new(format!("Todo {}", i)),
                is_complete: RwSignal::new(i % 2 == 0),
            })
            .collect::<Vec<_>>(),
    );

    view! {
      <div class="grid gap-2 p-4 mx-auto max-w-2xl">

        <div class="sticky top-0 py-3 bg-background">
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
        </div>

        <Show when=move || counters.get().is_empty()>
          <div class="grid place-content-center h-96 text-center text-gray-500">
            "No todos available. Please add some!"
          </div>
        </Show>

        <For
          each=move || counters.get()
          key=|counter| counter.id
          let(Todo { title, is_complete, id })
        >
          <div
            on:click=move |e: ev::MouseEvent| {
              if let Some(btn) = e.target().and_then(|t| t.dyn_into::<HtmlButtonElement>().ok()) {
                if let Ok(value) = btn.id().parse::<usize>() {
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
            class="flex gap-2 justify-between items-center py-4 px-3 rounded-xl transition-colors bg-stone-900 hover:bg-gray-950"
          >
            <h1 class="text-lg font-semibold">{move || title.get()}</h1>
            <div class="flex gap-2 items-center">
              <div class=move || {
                if is_complete.get() {
                  "py-1 px-2 text-sm font-medium bg-green-100/20 rounded-full text-green-500 cursor-pointer"
                } else {
                  "py-1 px-2 text-sm font-medium bg-red-100/20 rounded-full text-red-500 cursor-pointer"
                }
              }>{move || if is_complete.get() { " Completed" } else { " Pending" }}</div>

              <button
                id=move || id.to_string()
                value=move || id.to_string()
                data-is-complete=move || is_complete.get().to_string()
                title=move || if is_complete.get() { "Undo" } else { "Done" }
                class="py-1 px-2 text-sm font-medium rounded-xl cursor-pointer focus:ring-2 focus:outline-none bg-secondary/20 data-[is-complete=true]:text-red-500 data-[is-complete=false]:text-green-500 hover:bg-secondary/30 focus:ring-secondary/60"
                aria-label="Toggle Completion"
                aria-describedby="toggle-completion-button"
                on:click=move |e: ev::MouseEvent| {
                  e.stop_propagation();
                  if let Ok(value) = event_target_value(&e).parse::<usize>() {
                    set_counters
                      .update(move |todos| {
                        if let Some(todo) = todos.iter_mut().find(|todo| todo.id == value) {
                          *todo.is_complete.write() = !todo.is_complete.get();
                        }
                      });
                  }
                }
              >
                {move || if is_complete.get() { "Undo" } else { "Done" }}
              </button>
              <button
                value=move || id.to_string()
                id=move || id.to_string()
                title="Delete Todo"
                on:click=move |e: ev::MouseEvent| {
                  e.stop_propagation();
                  if let Ok(value) = event_target_value(&e).parse::<usize>() {
                    set_counters
                      .update(move |todos| {
                        todos.retain(|todo| todo.id != value);
                      });
                  }
                }
                aria-label="Delete Todo"
                aria-describedby="delete-todo-button"
                class="p-1 w-7 h-7 text-sm text-red-500 rounded-full bg-red-100/10"
              >
                X
              </button>
            </div>
          </div>
        </For>

      </div>
    }
}
