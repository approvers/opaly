use chrono::Duration;

use crate::model::Oppai;

pub trait OppaiRepository {
    fn add(&mut self, oppai: &Oppai);
    fn get(&self) -> Vec<Oppai>;
    fn count(&self) -> u64;
    fn get_latest(&self, duration: Duration);
}
