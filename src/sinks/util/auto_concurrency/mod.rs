//! Limit the max number of requests being concurrently processed.

mod controller;
mod future;
mod layer;
mod service;

pub(crate) use controller::IsBackPressure;
pub(crate) use layer::AutoConcurrencyLimitLayer;
pub(crate) use service::AutoConcurrencyLimit;
