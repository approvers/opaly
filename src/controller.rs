use chrono::{DateTime, Utc};

use crate::presenter::types::OppaiAnalysePresenter;
use crate::service::{OppaiAnalyseService, OppaiRegistryService};
use crate::repository::OppaiRepository;

pub struct OppaiController<R, P>
    where
        R: OppaiRepository,
        P: OppaiAnalysePresenter
{
    oppaiRegistryService: OppaiRegistryService<R>,
    oppaiAnalyseService: OppaiAnalyseService<R, P>
}

impl<R, P> OppaiController<R, P>
    where
        R: OppaiRepository,
        P: OppaiAnalysePresenter
{

    fn registry(&mut self, created_time: DateTime<Utc>) {
        unimplemented!();
    }

    fn analyse(&self) {
        unimplemented!();
    }

}
