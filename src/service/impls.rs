use std::{convert::{TryFrom, TryInto}, error::Error};
use chrono::{Duration, prelude::*};

use crate::{
    model::Oppai,
    presenter::types::{OppaiAnalysePresenter, OppaiAnalysePresenterInput},
    repository::OppaiRepository
};

use super::OppaiAnalyseService;

struct SafeRepositoryData<'a> {
    total: u64,
    latest: Option<&'a Oppai>,
    latest_duration: Vec<&'a Oppai>
}

impl<R, P> OppaiAnalyseService<R, P>
    where
        R: OppaiRepository,
        P: OppaiAnalysePresenter
{
    pub fn analyse(&self) {
        let repo_data = self.get_from_repository();
        if let Err(e) = repo_data {
            self.presenter.output(Err(e));
            return;
        }
        let repo_data = repo_data.unwrap();
        let latest_times = repo_data.latest_duration.iter().map(|x| x.created).collect();

        let average = self.calc_duration_average(&latest_times);
        let histogram = self.calc_histogram(&latest_times);

        let output = OppaiAnalysePresenterInput {
            latest: repo_data.latest.map(|x| x.created),
            total: repo_data.total,
            interval_average: average,
            time_histogram: histogram
        };

        self.presenter.output(Ok(output));
    }

    fn get_from_repository(&self) -> Result<SafeRepositoryData, Box<dyn Error>> {
        let latest_times = self.repository.get_latest_duration(Duration::weeks(3))?;
        let latest = self.repository.get_latest()?;
        let total = self.repository.count()?;

        Ok(SafeRepositoryData { total, latest, latest_duration: latest_times })
    }

    fn calc_duration_average(&self, times: &Vec<DateTime<Utc>>) -> Duration {
        let sum = times
            .windows(2)
            .map(|t| t[1] - t[0])
            .fold(Duration::zero(), |c, x| c + x);

        // XXX: This unwrap is NOT safety, but I'm not sure what case can cause panic
        sum / times.len().try_into().unwrap()
    }

    fn calc_histogram(&self, times: &Vec<DateTime<Utc>>) -> [u64; 24] {
        let mut histogram: [u64; 24] = Default::default();
        for time in times {
            // safety(unwrap): The return value from hour() won't exceed 23 so it's okay i guess
            histogram[usize::try_from(time.hour()).unwrap()] += 1;
        }

        histogram
    }
}
