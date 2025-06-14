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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
struct GetGithubActivityResponse {
    length: usize,
    data: Option<Vec<IGitHubActivity>>,
    error: Option<String>,
}

async fn fetch_gh_activity(year: i32) -> Option<GetGithubActivityResponse> {
    Request::get(format!("http://najmiter.vercel.app/api/activity?year={}", year).as_str())
        .send()
        .await
        .ok()?
        .json::<GetGithubActivityResponse>()
        .await
        .ok()
}

#[component]
pub fn GhActivity() -> impl IntoView {
    let (year, _set_year) = signal(2025);

    // Note: pass year directly, not year.get()
    let activity_data = LocalResource::new(move || fetch_gh_activity(year.get()));

    view! {
      <div>
        <Suspense fallback=move || {
          view! { <div class="loading">"Loading GitHub activity..."</div> }
        }>
          <div class="grid gap-4">
            {move || {
              activity_data
                .get()
                .map(|data| {
                  let res = data.unwrap_or_default().data.unwrap_or_default();
                  res
                    .into_iter()
                    .map(|week| {
                      view! {
                        <div class="activity-year">
                          {week
                            .contribution_days
                            .iter()
                            .map(|day| {
                              view! {
                                <div class="activity-day">
                                  <span class="date">{day.date.clone()}</span>
                                  <span class="count">{day.contribution_count}</span>
                                </div>
                              }
                            })
                            .collect::<Vec<_>>()}
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
