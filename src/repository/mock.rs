use std::convert::TryInto;

use chrono::Utc;

use crate::model::Oppai;

use super::OppaiRepository;

pub struct InMemoryOppaiRepository {
    db: Vec<Oppai>
}

impl OppaiRepository for InMemoryOppaiRepository {
    fn new() -> Self {
        Self { db: vec![] }
    }

    fn add(&mut self, oppai: Oppai) -> super::PossiblyError<()> {
        self.db.push(oppai);
        self.db.sort_by_key(|x| x.created);
        Ok(())
    }

    fn get(&self) -> super::PossiblyError<&Vec<Oppai>> {
        Ok(&self.db)
    }

    fn count(&self) -> super::PossiblyError<u64> {
        Ok(self.db.len().try_into().unwrap())
    }

    fn get_latest_duration(&self, duration: chrono::Duration) -> super::PossiblyError<Vec<&Oppai>> {
        Ok(
            self.db
                .iter()
                .filter(|x| (Utc::now() - x.created) <= duration)
                .collect()
        )
    }

    fn get_latest(&self) -> super::PossiblyError<Option<&Oppai>> {
        Ok(
            if self.db.len() > 0 {
                Some(
                    &self.db[self.db.len() - 1]
                )
            } else {
                None
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use chrono::{DateTime, Duration, Utc};

    use crate::{model::Oppai, repository::OppaiRepository};
    use super::InMemoryOppaiRepository;

    #[test]
    fn in_memory_repo_test() {
        let current_time = Utc::now();
        let mut repo = InMemoryOppaiRepository::new();
        for i in (0..10).rev() {
            repo.add(Oppai { created: current_time - Duration::days(i) }).expect("add() should success");
        }
        perform_test(current_time, repo);
    }

    #[test]
    fn in_memory_repo_reversed_test() {
        let current_time = Utc::now();
        let mut repo = InMemoryOppaiRepository::new();
        for i in 0..10 {
            repo.add(Oppai { created: current_time - Duration::days(i) }).expect("add() should success");
        }
        perform_test(current_time, repo);
    }

    fn perform_test(current_time: DateTime<Utc>, repo: InMemoryOppaiRepository) {
        let got = repo.get().expect("get() should success");
        for i in 0..10 {
            assert_eq!(got[i].created, current_time - Duration::days((9 - i).try_into().unwrap()));
        }

        let got_latests = repo.get_latest_duration(Duration::days(5)).expect("get_latest_duration() should success");
        for i in 0..5 {
            assert_eq!(got_latests[i].created, current_time - Duration::days((4 - i).try_into().unwrap()));
        }


        assert_eq!(
            repo
                .get_latest()
                .expect("get_latest() should success")
                .expect("should return value")
                .created,
            current_time
        );
        assert_eq!(repo.count().expect("count() should success"), 10)
    }
}
