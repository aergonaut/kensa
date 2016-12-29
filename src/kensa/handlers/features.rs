//! The `features` resource contains handlers for manipulating `Feature`s.

use errors::*;
use models::Feature;
use util;

use diesel::prelude::*;
use rocket_contrib::JSON;

/// GET /features
#[get("/")]
pub fn index() -> Result<JSON<Vec<Feature>>> {
    use schema::features::dsl::*;

    let connection = util::establish_connection()?;
    let result = features.load::<Feature>(&connection).chain_err(|| "Could not establish database connection")?;

    Ok(JSON(result))
}
