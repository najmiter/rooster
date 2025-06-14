use leptos::prelude::*;

use crate::data::resume::get_resume_data;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
      <div class="mt-6 space-y-4">
        <For
          each=move || get_resume_data().work_experience
          key=|project| project.company.clone()
          let(project)
        >
          <div class="p-4 bg-gray-800 rounded-lg shadow-md">
            <h2 class="text-xl font-semibold">{project.company}</h2>
            <p class="text-gray-300">{project.description[0].clone().project_name}</p>
            <a
              href=project.description[0].clone().link
              target="_blank"
              class="text-blue-400 hover:underline"
            >
              "View Project"
            </a>
          </div>
        </For>
      </div>
    }
}
