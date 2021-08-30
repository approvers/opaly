use crate::model::Oppai;

pub trait OppaiRepository {
    fn add(&mut self, oppai: &Oppai);
    fn get(&self);
}
