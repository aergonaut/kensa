//! The `handlers` umbrella module contains modules for each resource in the application.
//! 
//! Define new resources by creating a new module in `handlers` and naming it after the pluralized
//! name of the resource. Routes within each resource should be defined as though the resource
//! would be mounted at the root. In reality most resources will be mounted on sub-paths, but
//! defining routes this way allows some flexibility in changing the URL structure in the future.
//! 
//! By convention, the handlers for the usual REST actions should be named:
//! 
//! *   index
//! *   show
//! *   create
//! *   update
//! *   destroy
use rocket::Request;
use rocket_contrib::JSON;

pub mod features;

#[derive(Debug, Serialize)]
pub struct ClientError {
    pub id: String,
    pub message: String,
    pub url: Option<String>
}

#[error(404)]
pub fn not_found(_: &Request) -> JSON<ClientError> {
    JSON(ClientError {
        id: "not_found".to_owned(),
        message: "The requested resource could not be found".to_owned(),
        url: None
    })
}
