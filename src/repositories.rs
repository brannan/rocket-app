use diesel::prelude::*;

use crate::{
    models::{NewRustacean, Rustacean},
    schema::rustaceans,
};

pub struct RustaceanRepository;

impl RustaceanRepository {
    pub fn find(c: &mut SqliteConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).first::<Rustacean>(c)
    }

    pub fn all(c: &mut SqliteConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table
            .order(rustaceans::id.desc())
            .limit(limit)
            .load::<Rustacean>(c)
    }

    pub fn create(c: &mut SqliteConnection, new_rustacean: NewRustacean) -> QueryResult<Rustacean> {
        let _result = diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .execute(c);

        rustaceans::table
            .order(rustaceans::id.desc())
            .first::<Rustacean>(c)
    }

    pub fn update(c: &mut SqliteConnection, id: i32, rustacean: Rustacean) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::name.eq(&rustacean.name),
                rustaceans::email.eq(&rustacean.email),
            ))
            .execute(c)?;

        Self::find(c, id)
    }

    pub fn delete(c: &mut SqliteConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id)).execute(c)
    }
}
