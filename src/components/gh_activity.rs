use gloo_net::http::Request;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
struct ContributionDay {
    date: String,
    #[serde(rename = "contributionCount")]
    contribution_count: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
struct IGitHubActivity {
    #[serde(rename = "contributionDays")]
    contribution_days: Vec<ContributionDay>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
struct GetGithubActivityResponse {
    length: usize,
    data: Option<Vec<IGitHubActivity>>,
    error: Option<String>,
}

async fn fetch_gh_activity(year: RwSignal<i32>) -> Option<GetGithubActivityResponse> {
    Request::get(
        format!(
            "http://najmiter.vercel.app/api/activity?year={}",
            year.get()
        )
        .as_str(),
    )
    .send()
    .await
    .ok()?
    .json::<GetGithubActivityResponse>()
    .await
    .ok()
}

#[component]
pub fn GhActivity() -> impl IntoView {
    let (year, set_year) = signal(RwSignal::new(2025)); // Renamed setYear to set_year for Rust conventions
    let activity_data = LocalResource::new(move || fetch_gh_activity(year.get())); // Use year.get()

    view! {
      <div>
        <Suspense fallback=move || {
          view! { <div class="loading">"Loading GitHub activity..."</div> }
        }>
          <div class="grid gap-4">
            {move || {
              activity_data
                .read()
                .clone()
                .unwrap_or_default()
                .map(|response| {
                  response
                    .to_owned()
                    .data
                    .unwrap_or_default()
                    .iter()
                    .map(|activity| {
                      view! {
                        <div class="activity-year">
                          <ul class="flex flex-col gap-2">
                            {activity
                              .contribution_days
                              .iter()
                              .map(|day| {
                                view! {
                                  <li class="flex gap-3 items-center">
                                    <span class="date">{day.date.clone()}</span>
                                    <span class="count">{day.contribution_count.to_string()}</span>
                                  </li>
                                }
                              })
                              .collect::<Vec<_>>()}
                          </ul>
                        </div>
                      }
                    })
                    .collect::<Vec<_>>()
                })
            }}
          </div>
        </Suspense>
      </div>
    }
}
