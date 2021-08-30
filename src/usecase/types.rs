use crate::model::Oppai;

pub trait OppaiRegistryUsecase {
    fn registry(&self, input: &Oppai);
}

pub trait OppaiAnalyseUsecase {
    fn analyse(&self);
}
