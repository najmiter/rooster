use leptos::prelude::*;
use leptos_router::components::Form;

#[component]
pub fn TodoForm<F>(on_submit: F) -> impl IntoView
where
    F: Fn(String) + 'static + Clone,
{
    let (title, set_title) = signal(String::new());

    view! {
      <Form
        action=""
        on:submit=move |e| {
          e.prevent_default();
          on_submit.clone()(title.get());
          set_title.set(String::new());
        }
      >
        <div class="flex gap-2 justify-center items-center w-full">
          <input
            type="text"
            placeholder="Enter a todo title"
            value=title
            prop:value=move || title.get()
            on:input=move |e| {
              let val = event_target_value(&e);
              set_title.set(val);
            }
            required
            class="p-2 w-full max-w-xs rounded border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none"
          />
          <button
            type="submit"
            class="py-2 px-4 text-white bg-blue-500 rounded hover:bg-blue-600 focus:ring-2 focus:ring-blue-500 focus:outline-none"
            aria-label="Add Todo"
            aria-describedby="add-todo-button"
          >
            "Add Todo"
          </button>
        </div>
      </Form>
    }
}
