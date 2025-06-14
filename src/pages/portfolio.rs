use crate::components::{gh_activity::GhActivity, projects::Projects};
use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn Portfolio() -> impl IntoView {
    view! {
      <Title text="Portfolio" />
      <div class="min-h-screen bg-[#111]">
        <div class="p-4 mx-auto space-y-10 max-w-screen-lg">
          <section>
            <header class="space-y-2">
              <h1 class="text-3xl font-bold">"Najam ul Hassan"</h1>
              <p class="text-gray-400">
                "This is my portfolio page. It showcases my GitHub activity and other projects."
              </p>
            </header>
          </section>
          <section class="space-y-3">
            <h1 class="text-xl font-semibold">"GitHub Activity"</h1>

            <GhActivity />
          </section>
          <section class="space-y-3">
            <h1 class="text-xl font-semibold">"Work Experience"</h1>

            <Projects />
          </section>
        </div>
      </div>
    }
}
