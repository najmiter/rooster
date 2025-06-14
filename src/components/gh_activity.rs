use chrono::{Datelike, NaiveDate};
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
    let (year, set_year) = signal(chrono::Utc::now().year());
    let activity_data = LocalResource::new(move || fetch_gh_activity(year.get()));

    view! {
      <div class="space-y-2">
        <Suspense fallback=move || {
          view! { <div class="animate-pulse">"Loading GitHub activity..."</div> }
        }>
          <select
            on:change=move |e| {
              let value = event_target_value(&e);
              set_year.set(value.parse::<i32>().unwrap_or_default());
            }
            class="p-2 w-full max-w-xs rounded-md text-slate-200 bg-[#181818]"
          >
            {(2020..=chrono::Utc::now().year())
              .map(|y| {
                view! {
                  <option value=y.to_string() selected=year.get() == y>
                    {y}
                  </option>
                }
              })
              .collect::<Vec<_>>()}
          </select>

          <div class="grid overflow-auto gap-4 p-2 rounded-lg contain-paint bg-[#181818]">
            {move || {
              activity_data
                .get()
                .map(|data| {
                  let res = data.unwrap_or_default().data.unwrap_or_default();
                  let most_contributions = res
                    .iter()
                    .map(|activity| {
                      activity
                        .contribution_days
                        .iter()
                        .map(|day| day.contribution_count)
                        .max()
                        .unwrap_or(0)
                    })
                    .max()
                    .unwrap_or(0);

                  view! {
                    <div class="flex gap-1 justify-between items-end">
                      {res
                        .into_iter()
                        .map(|week| {
                          view! {
                            <div class="grid gap-1">
                              {week
                                .contribution_days
                                .iter()
                                .map(|day| {
                                  let amount = day.contribution_count as f32
                                    / most_contributions as f32;
                                  let mut opacity = amount;
                                  if amount < 1.0 && amount > 0.0 {
                                    opacity += 0.2;
                                  }
                                  view! {
                                    <div
                                      role="button"
                                      data-date=NaiveDate::parse_from_str(&day.date, "%Y-%m-%d")
                                        .map(|d| d.format("%B %d").to_string())
                                        .unwrap_or_else(|_| day.date.clone())
                                      data-amount=day.contribution_count
                                      class="relative w-3 h-3 rounded cursor-pointer hover:bg-green-700 activity-box border-green-950 border-[0.5px]"
                                      style=format!(
                                        "background-color: rgba(0, 128, 0, {});",
                                        opacity,
                                      )
                                    />
                                  }
                                })
                                .collect::<Vec<_>>()}
                            </div>
                          }
                        })
                        .collect::<Vec<_>>()}
                    </div>
                  }
                })
            }}
          </div>
        </Suspense>
      </div>
    }
}
