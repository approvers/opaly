use chrono::{DateTime, Duration, Utc};

pub trait OppaiAnalysePresenter {
    fn output(&self);
}

pub struct OppaiAnalysePresenterOutput {
    latest: DateTime<Utc>,
    total: u64,
    interval_average: Duration,
    time_histogram: [u64; 24]
}
