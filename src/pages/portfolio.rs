use crate::components::animated_view::AnimatedView;
use crate::{
    components::{gh_activity::GhActivity, projects::Projects},
    data::resume::get_resume_data,
};
use chrono::Datelike;
use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn Portfolio() -> impl IntoView {
    let resume_data = get_resume_data();
    let personal_info = resume_data.personal_info;
    let professional_summary = resume_data.professional_summary;

    view! {
      <Title text="Portfolio" />
      <div class="min-h-screen bg-[#111]">
        <div class="p-4 mx-auto space-y-10 max-w-screen-lg">
          <AnimatedView>
            <div class="pb-8 mb-4 border-b border-gray-800">
              <h1 class="mb-4 text-4xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-blue-500 to-purple-600 md:text-6xl text-gradient">
                {personal_info.name}
              </h1>
              <div class="flex flex-col gap-2 mb-8 text-gray-400 md:flex-row md:gap-6 md:items-center">
                <a
                  href=format!("mailto:{}", personal_info.email.clone())
                  class="transition-colors hover:text-blue-400"
                >
                  {personal_info.email.clone()}
                </a>
                <span class="hidden md:inline">|</span>
                <a
                  href=personal_info.linkedin
                  target="_blank"
                  rel="noopener noreferrer"
                  class="transition-colors hover:text-blue-400"
                >
                  LinkedIn
                </a>
                <span class="hidden md:inline">|</span>
                <a
                  href=format!("tel:{}", personal_info.phone.clone())
                  class="transition-colors hover:text-blue-400"
                >
                  {personal_info.phone.clone()}
                </a>
              </div>
            </div>
          </AnimatedView>

          <section class="py-7 mb-16 space-y-3 md:pb-16">
            <h1 class="text-xl font-semibold">"GitHub Activity"</h1>
            <GhActivity />
          </section>

          <AnimatedView>
            <section class="mb-16">
              <div class="mb-8 w-fit">
                <h2 class="inline-block pb-1 text-2xl font-semibold">"Professional Summary"</h2>
                <div aria-hidden class="animate-underline" />
              </div>
              <p class="leading-relaxed text-gray-300">{professional_summary}</p>
            </section>
          </AnimatedView>

          <section class="space-y-3">
            <h1 class="text-xl font-semibold">"Work Experience"</h1>
            <Projects />
          </section>

          <footer class="py-8 mt-16 text-center text-gray-400 border-t border-gray-800">
            <p>"© " {chrono::Utc::now().year()} " Najam. All rights reserved."</p>
            <div class="mt-2">
              <a
                href="https://github.com/najmiter/najmiter.github.io"
                target="_blank"
                class="text-blue-500 hover:underline"
              >
                {"Source code (it's free if you want to use it ♥️)"}
              </a>
            </div>
          </footer>
        </div>
      </div>
    }
}
