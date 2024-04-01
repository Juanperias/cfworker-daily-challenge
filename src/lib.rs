use daily::get_daily;
use reqwest::ClientBuilder;
use time::{Date, Month, OffsetDateTime};
use worker::{
    console_debug, console_error, console_warn, event, Env, ScheduleContext, ScheduledEvent,
};

use crate::cangrebot::set_daily;

mod cangrebot;
mod challenge;
mod daily;

#[event(scheduled)]
pub async fn main(_e: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    // Custom panic
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(|info: &std::panic::PanicInfo| {
        console_error!("{info}")
    }));

    // calculate days
    // its used for enumerate daily challenges
    let Ok(start_date) = Date::from_calendar_date(2024, Month::March, 25) else {
        console_warn!("Cannot create Start Date");
        return;
    };
    let start_date = start_date
        .with_hms(0, 0, 0)
        .expect("Cannot set hms to start date")
        .assume_utc();
    let now = OffsetDateTime::now_utc().date();

    let diff = now - start_date.date();
    let days = diff.whole_days();

    if days < 0 {
        console_warn!("Time left: {days}");
        return;
    }

    let Ok(endpoint) = env.var("ENDPOINT").map(|e| e.to_string()) else {
        console_error!("Cannot get 'ENDPOINT' environment variable");
        return;
    };

    let client = ClientBuilder::default()
        .user_agent("Mozilla/5.0 LeetCode API")
        .build()
        .expect("Cannot build client reqwest");

    let challenge = get_daily(&client).await;

    console_debug!("Challenge response: {challenge:?}");

    set_daily(endpoint, days, challenge, &client).await;
}
