use axum::{Form, extract::State};
use jiff::{Timestamp, ToSpan, Zoned, civil, tz::TimeZone};
use serde::{Deserialize, Serialize};

use crate::server::{Server, somehow};

#[derive(Serialize)]
struct Data {
    year: i16,
    month: i8,
    times: Vec<(String, String)>,
    feed: bool,
}

#[derive(Deserialize)]
pub struct FormData {
    pub latitude: f64,
    pub longitude: f64,
    pub year: Option<i16>,
    pub month: Option<i8>,
    pub feed: Option<bool>,
}

pub async fn post(server: State<Server>, Form(form): Form<FormData>) -> somehow::Result<()> {
    let now = Zoned::now();
    let now_date_utc = now.with_time_zone(TimeZone::UTC).date();
    let year = form.year.unwrap_or(now_date_utc.year());
    let month = form.month.unwrap_or(now_date_utc.month());

    let first = civil::Date::new(year, month, 1)?;
    let mut times = vec![];
    for day in 1..=first.days_in_month() {
        let date = first + day.days();
        let (rise, set) = sunrise::sunrise_sunset(
            form.latitude,
            form.longitude,
            date.year() as i32,
            date.month() as u32,
            date.day() as u32,
        );

        let rise = Timestamp::new(rise, 0)?
            .to_zoned(now.time_zone().clone())
            .strftime("%H:%M")
            .to_string();

        let set = Timestamp::new(set, 0)?
            .to_zoned(now.time_zone().clone())
            .strftime("%H:%M")
            .to_string();

        times.push((rise, set));
    }

    let data = Data {
        year,
        month,
        times,
        feed: form.feed.unwrap_or(true),
    };

    let typst = super::typst_with_lib()
        .with_json("/data.json", &data)
        .with_main_file(include_str!("main.typ"));

    server.print_typst(typst).await
}
