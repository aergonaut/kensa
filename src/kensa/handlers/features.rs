//! The `features` resource contains handlers for manipulating `Feature`s.

use errors::*;
use models::{Feature, NewFeatureParams, create_feature};
use util;

use diesel;
use diesel::prelude::*;
use rocket_contrib::JSON;
use uuid::Uuid;

/// GET /features
#[get("/")]
pub fn index() -> Result<JSON<Vec<Feature>>> {
    use schema::features::dsl::*;

    let connection = util::establish_connection()?;
    let result = features.load::<Feature>(&connection).chain_err(|| "Could not establish database connection")?;

    Ok(JSON(result))
}

#[get("/<feature_id>")]
pub fn show(feature_id: &str) -> Result<Option<JSON<Feature>>> {
    use schema::features::dsl::*;

    let uuid = Uuid::parse_str(feature_id).chain_err(|| "ID must be valid UUID")?;

    let connection = util::establish_connection()?;
    let result = features.find(uuid).first(&connection).ok().map(|f| JSON(f));

    Ok(result)
}

#[post("/", data = "<feature>")]
pub fn create(feature: JSON<NewFeatureParams>) -> Result<JSON<Feature>> {
    let connection = util::establish_connection()?;
    create_feature(&connection, &feature).map(|f| JSON(f))
}

#[delete("/<feature_id>")]
pub fn destroy(feature_id: &str) -> Result<()> {
    use schema::features::dsl::*;

    let uuid = Uuid::parse_str(feature_id).chain_err(|| "ID must be valid UUID")?;

    let connection = util::establish_connection()?;
    let _ = diesel::delete(features.filter(id.eq(uuid))).execute(&connection).chain_err(|| "Could not delete feature")?;

    Ok(())
}
