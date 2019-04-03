use diesel::{self, prelude::*};

mod schema {
    table! {
        challenges (id) {
            id -> Integer,
            title -> Text,
            description -> Text,
        }
    }
}

use self::schema::challenges;

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug)]
pub struct Challenge {
    pub id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name="challenges"]
pub struct NewChallenge {
    pub title: String,
    pub description: String,
}

pub enum ChallengeError {
    NotFoundError,
    DatabaseError,
}

impl Challenge {
    pub fn list(conn: &SqliteConnection) -> Vec<Challenge> {
        use self::schema::challenges::dsl::*;
        let results = challenges
                        .order(id)
                        .load::<Challenge>(conn);

        if let Ok(challenge_list) = results {
            return challenge_list;
        }

        vec![]
    }

    pub fn get(target_id: i32, conn: &SqliteConnection) -> Option<Challenge> {
        use self::schema::challenges::dsl::*;
        let result = challenges
                            .find(target_id)
                            .first::<Challenge>(conn);
        if let Ok(challenge) = result {
            return Some(challenge)
        }

        None
    }

    pub fn add(input: Vec<NewChallenge>, conn: &SqliteConnection) -> Vec<Challenge> {
        use self::schema::challenges::dsl::*;
        let results = conn.transaction::<_, diesel::result::Error, _>(|| {
            let inserted_count = diesel::insert_into(challenges)
                .values(&input)
                .execute(conn)?;

            Ok(challenges
                .order(id.desc())
                .limit(inserted_count as i64)
                .load(conn)?
                .into_iter()
                .rev()
                .collect::<Vec<_>>())
        });

        if let Ok(challenge_list) = results {
            return challenge_list;
        }

        vec![]
    }

    pub fn remove(target_id: i32, conn: &SqliteConnection) -> Result<Challenge, ChallengeError> {
        use self::schema::challenges::dsl::*;
        let result = conn.transaction::<_, diesel::result::Error, _>(|| {
            let target = challenges
                .find(target_id)
                .first::<Challenge>(conn)?;

            diesel::delete(challenges.find(target_id))
                .execute(conn)?;

            Ok(target)
        });

        match result {
            Ok(challenge) => Ok(challenge),
            Err(diesel::result::Error::NotFound) => Err(ChallengeError::NotFoundError),
            _ => Err(ChallengeError::DatabaseError),
        }
    }

    #[cfg(test)]
    pub fn remove_all(conn: &SqliteConnection) -> Vec<Challenge> {
        use self::schema::challenges::dsl::*;
        let results = conn.transaction::<_, diesel::result::Error, _>(|| {
            let removed_challenges = challenges.load::<Challenge>(conn)?;

            diesel::delete(challenges)
                .execute(conn)?;

            Ok(removed_challenges)
        });

        if let Ok(challenge_list) = results {
            return challenge_list;
        }

        vec![]
    }
}
