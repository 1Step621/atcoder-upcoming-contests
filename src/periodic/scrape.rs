use chrono::{DateTime, Utc};
use scraper::{Html, Selector};

use crate::state::{Contest, State};

pub async fn scrape(state: &State) {
    let res = reqwest::get("https://atcoder.jp/contests/")
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .text()
        .await
        .unwrap();
    let document = Html::parse_document(&res);
    let selector = Selector::parse("#contest-table-upcoming tbody tr").unwrap();
    let rows = document.select(&selector);

    let contests = rows
        .map(|row| {
            let parsed_as_text = row
                .child_elements()
                .map(|e| e.text().next().unwrap())
                .collect::<Vec<_>>();
            let [datetime, _, duration, rated_range] = parsed_as_text.as_slice() else {
                panic!("Failed to parse row");
            };

            let start_time = DateTime::parse_from_str(datetime, "%Y-%m-%d %H:%M:%S%z")
                .unwrap()
                .with_timezone(&Utc);

            let duration = duration.split(':').collect::<Vec<_>>();
            let [hours, minutes, ..] = duration.as_slice() else {
                panic!("Failed to parse duration");
            };
            let hours = hours.parse::<u32>().unwrap();
            let minutes = minutes.parse::<u32>().unwrap();
            let duration = hours * 60 + minutes;

            let rated_range = rated_range.to_string();

            let name_element = row.select(&Selector::parse("a").unwrap()).nth(1).unwrap();
            let name = name_element.text().next().unwrap().to_string();
            let url = format!("https://atcoder.jp{}", name_element.value().attr("href").unwrap().to_string());

            Contest {
                start_time,
                name,
                duration,
                rated_range,
                url,
            }
        })
        .collect::<Vec<_>>();

    *state.contests.lock().unwrap() = contests;
}
