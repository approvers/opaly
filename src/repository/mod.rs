pub mod mock;

use std::error::Error;

use chrono::Duration;

use crate::model::Oppai;

type PossiblyError<T> = Result<T, Box<dyn Error>>;

pub trait OppaiRepository {
    fn new() -> Self;
    fn add(&mut self, oppai: Oppai) -> PossiblyError<()>;
    fn get(&self) -> PossiblyError<&Vec<Oppai>>;
    fn count(&self) -> PossiblyError<u64>;
    fn get_latest_duration(&self, duration: Duration) -> PossiblyError<Vec<&Oppai>>;
    fn get_latest(&self) -> PossiblyError<Option<&Oppai>>;
}
