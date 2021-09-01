use chrono::{DateTime, Duration, Utc};

pub trait OppaiAnalysePresenter {
    fn output(&self, input: OppaiAnalysePresenterInput);
}

pub struct OppaiAnalysePresenterInput {
    latest: DateTime<Utc>,
    total: u64,
    interval_average: Duration,
    time_histogram: [u64; 24]
}
