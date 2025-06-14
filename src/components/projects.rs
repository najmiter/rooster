use leptos::prelude::*;

use crate::components::animated_view::AnimatedView;
use crate::data::resume::get_resume_data;

#[component]
pub fn Projects() -> impl IntoView {
    let resume_data = get_resume_data();
    let work_experience = resume_data.work_experience;
    let skills = resume_data.skills;
    let education = resume_data.education;

    view! {
      <div class="mt-6 space-y-4">
        <section class="mb-16">
          <div class="mb-8 w-fit">
            <h2 class="inline-block pb-1 text-2xl font-semibold">"Work Experience"</h2>
            <div aria-hidden class="animate-underline" />
          </div>
          <div class="flex flex-col gap-12">
            {work_experience
              .into_iter()
              .map(|job| {
                view! {
                  <div>
                    <div class="relative">
                      <div class="flex flex-col mb-3 md:flex-row md:justify-between md:items-start">
                        <h3 class="overflow-hidden relative text-xl font-semibold text-blue-600 company-name">
                          {job.company.clone()}
                        </h3>
                        <div class="text-gray-400">{job.period.clone()}</div>
                      </div>
                      <div class="mb-3 text-lg text-gray-300">
                        {job.position.clone()} " • " {job.location.clone()}
                      </div>
                      <div class="mt-4 space-y-6">
                        {job
                          .description
                          .into_iter()
                          .map(|project| {
                            let has_contributions = project.contributions.is_some();
                            let has_technologies = project.technologies.is_some();
                            let has_features = project.features.is_some();
                            view! {
                              <AnimatedView>
                                <div class="p-5 rounded-lg bg-[#181818]">
                                  <h4 class="mb-2 text-lg font-medium text-white">
                                    <a
                                      target="_blank"
                                      href=project.link.clone()
                                      class="flex gap-1.5 items-center transition-colors hover:text-blue-300 hover:underline underline-offset-2 group"
                                    >
                                      {project.project_name.clone()}

                                    </a>
                                  </h4>
                                  <p class="mb-3 text-gray-300">{project.details.clone()}</p>

                                  <div class=move || {
                                    if !has_technologies { "hidden" } else { "mb-3" }
                                  }>
                                    <span class="block mb-1 text-sm text-gray-400">
                                      "Technologies:"
                                    </span>
                                    <p class="text-gray-300">{project.technologies.clone()}</p>
                                  </div>

                                  <div class=move || {
                                    if !has_features { "hidden" } else { "mb-3" }
                                  }>
                                    <span class="block mb-1 text-sm text-gray-400">
                                      "Features:"
                                    </span>
                                    <ul class="space-y-1 list-disc list-inside text-gray-300">
                                      {project
                                        .features
                                        .into_iter()
                                        .flatten()
                                        .map(|feature| {
                                          view! { <Li>{feature}</Li> }
                                        })
                                        .collect::<Vec<_>>()}
                                    </ul>
                                  </div>

                                  <div class=move || {
                                    if !has_contributions { "hidden" } else { "" }
                                  }>
                                    <span class="block mb-1 text-sm text-gray-400">
                                      "Contributions:"
                                    </span>
                                    <ul class="space-y-1 list-disc list-inside text-gray-300">
                                      {project
                                        .contributions
                                        .into_iter()
                                        .flatten()
                                        .map(|contribution| view! { <Li>{contribution}</Li> })
                                        .collect::<Vec<_>>()}
                                    </ul>
                                  </div>
                                </div>
                              </AnimatedView>
                            }
                          })
                          .collect::<Vec<_>>()}

                      </div>
                    </div>
                  </div>
                }
              })
              .collect::<Vec<_>>()}
          </div>
        </section>

        <section class="mb-16">
          <div class="mb-8 w-fit">
            <h2 class="inline-block pb-1 text-2xl font-semibold">"Skills"</h2>
            <div aria-hidden class="animate-underline" />
          </div>
          <div class="flex flex-wrap gap-2">
            {skills
              .into_iter()
              .enumerate()
              .map(|(index, skill)| {
                view! { <Tag index=index as i32>{skill.clone()}</Tag> }
              })
              .collect::<Vec<_>>()}
          </div>
        </section>

        <AnimatedView>
          <section class="mb-16">
            <div class="mb-8 w-fit">
              <h2 class="inline-block pb-1 text-2xl font-semibold">Education</h2>
              <div aria-hidden class="animate-underline" />
            </div>
            <div class="relative pl-6 border-l-2 border-gray-800">
              <div class="absolute top-2 w-3 h-3 bg-blue-500 rounded-full -left-[7px]"></div>
              <div class="text-lg font-medium">{education[0].degree.clone()}</div>
              <div class="text-gray-300">{education[0].institution.clone()}</div>
              <div class="text-gray-400">{education[0].year.clone()}</div>
            </div>
          </section>
        </AnimatedView>
      </div>
    }
}

#[component]
fn Li(children: Children) -> impl IntoView {
    view! {
      <li class="flex gap-1 items-start">
        <span class="min-w-7 min-h-7">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="28"
            height="28"
            viewbox="0 0 24 24"
            fill="none"
          >
            <path
              fill="#3b82f6"
              d="M6 9.33v5.34c0 3.32 2.35 4.67 5.22 3.02l1.28-.74c.31-.18.5-.51.5-.87V7.92c0-.36-.19-.69-.5-.87l-1.28-.74C8.35 4.66 6 6.01 6 9.33z"
            ></path>
            <path
              fill="#3b82f6"
              d="M14 8.79v6.43c0 .39.42.63.75.43l1.1-.64c2.87-1.65 2.87-4.37 0-6.02l-1.1-.64a.503.503 0 00-.75.44z"
              opacity=".4"
            ></path>
          </svg>
        </span>
        <div>{children()}</div>
      </li>
    }
}

#[component]
fn Tag(index: i32, children: Children) -> impl IntoView {
    view! {
      <div
        style=format!("'--slide-in-delay': {}ms", index as f32 * 0.1)
        class="inline-block py-1 px-2 mr-2 mb-2 text-xs text-white rounded-md slide-in-animation bg-[#232323]"
      >
        {children()}
      </div>
    }
}
