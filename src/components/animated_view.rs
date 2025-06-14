use leptos::prelude::*;

#[component]
pub fn AnimatedView(children: Children) -> impl IntoView {
    view! { <div class="come-into-view">{children()}</div> }
}
