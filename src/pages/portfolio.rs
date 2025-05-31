use crate::components::gh_activity::GhActivity;
use leptos::prelude::*;

#[component]
pub fn Portfolio() -> impl IntoView {
    view! {
      <div class="p-4 mx-auto max-w-screen-lg">
        <GhActivity />
      </div>
    }
}
