use crate::{
    model::Oppai,
    presenter::types::OppaiAnalysePresenter,
    repository::types::OppaiRepository
};

pub struct OppaiRegistryService<R>
    where R: OppaiRepository
{
    repository: R
}

pub struct OppaiAnalyseService<R, P>
    where
        R: OppaiRepository,
        P: OppaiAnalysePresenter
{
    repository: R,
    presenter: P
}

impl<R> OppaiRegistryService<R>
    where R: OppaiRepository
{
    fn registry(&self, input: &Oppai) {
        unimplemented!();
    }
}

impl<R, P> OppaiAnalyseService<R, P>
    where
        R: OppaiRepository,
        P: OppaiAnalysePresenter
{
    fn analyse(&self) {
        unimplemented!();
    }
}
