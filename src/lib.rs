use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};
use pages::portfolio::Portfolio;

mod components;
mod data;
mod pages;

use crate::pages::todo::Todo;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
      <Html attr:lang="en" attr:dir="ltr" attr:data-theme="dark" attr:class="dark" />

      <Title text="Portfolio" />

      <Meta charset="UTF-8" />
      <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

      <Router>
        <Routes fallback=|| view! { NotFound }>
          <Route path=path!("/") view=Portfolio />
          <Route path=path!("/todo") view=Todo />
        </Routes>
      </Router>
    }
}
