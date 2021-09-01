mod impls;

use crate::{
    model::Oppai,
    presenter::types::{OppaiAnalysePresenter, OppaiAnalysePresenterInput},
    repository::OppaiRepository
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
    pub fn registry(&mut self, input: Oppai) {
        self.repository.add(input);
    }
}

