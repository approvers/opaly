use std::error::Error;

use chrono::{DateTime, Duration, Utc};

pub trait OppaiAnalysePresenter {
    fn output(&self, input: Result<OppaiAnalysePresenterInput, Box<dyn Error>>);
}

pub struct OppaiAnalysePresenterInput {
    pub latest: Option<DateTime<Utc>>,
    pub total: u64,
    pub interval_average: Duration,
    pub time_histogram: [u64; 24]
}
