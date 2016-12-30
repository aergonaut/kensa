use errors::*;
use schema::features;

use chrono::{self, NaiveDateTime};
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use uuid::Uuid;

#[derive(Debug, Serialize, Queryable)]
pub struct Feature {
    pub id: Uuid,
    pub name: String,
    pub feature_type: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

/// Params object for creating a new feature
#[derive(Debug, Deserialize)]
pub struct NewFeatureParams {
    pub name: String,
    pub feature_type: String 
}

#[derive(Debug, Insertable)]
#[table_name="features"]
struct NewFeature<'a> {
    name: &'a str,
    feature_type: &'a str,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}

pub fn create_feature(connection: &PgConnection, new_feature: &NewFeatureParams) -> Result<Feature> {
    use schema::features;

    let now = chrono::UTC::now().naive_utc();

    let nfi = NewFeature {
        name: &new_feature.name,
        feature_type: &new_feature.feature_type,
        created_at: now,
        updated_at: now
    };

    diesel::insert(&nfi).into(features::table).get_result(connection).chain_err(|| "Could not insert record")
}
