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
        <div class="flex flex-col gap-2 justify-center items-center w-full sm:flex-row">
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
            class="p-2 w-full rounded border focus:ring-2 focus:outline-none border-secondary/70 bg-secondary focus:ring-primary"
          />
          <button
            type="submit"
            class="py-2 px-4 w-full text-white rounded focus:ring-2 focus:outline-none bg-secondary min-w-fit sm:w-fit hover:bg-secondary/70 focus:ring-secondary/60"
            aria-label="Add Todo"
            aria-describedby="add-todo-button"
          >
            "Add Todo"
          </button>
        </div>
      </Form>
    }
}
