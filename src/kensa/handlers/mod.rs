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

pub mod features;
