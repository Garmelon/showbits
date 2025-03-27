use anyhow::anyhow;
use axum::{Form, extract::State};
use jiff::{Timestamp, ToSpan, Zoned, civil, tz::TimeZone};
use serde::{Deserialize, Serialize};
use sunrise::{Coordinates, SolarDay, SolarEvent};

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

    let coord = Coordinates::new(form.latitude, form.longitude)
        .ok_or_else(|| somehow::Error(anyhow!("Invalid coordinates")))?;

    let first = civil::Date::new(year, month, 1)?;
    let mut times = vec![];
    for day in 0..first.days_in_month() {
        let date = first + day.days();

        let date_chrono = chrono::NaiveDate::from_ymd_opt(
            date.year() as i32,
            date.month() as u32,
            date.day() as u32,
        )
        .unwrap();

        let solar_day = SolarDay::new(coord, date_chrono);

        let rise_chrono = solar_day.event_time(SolarEvent::Sunrise);
        let set_chrono = solar_day.event_time(SolarEvent::Sunset);

        let rise = Timestamp::new(rise_chrono.timestamp(), 0)?
            .to_zoned(now.time_zone().clone())
            .strftime("%H:%M")
            .to_string();

        let set = Timestamp::new(set_chrono.timestamp(), 0)?
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
