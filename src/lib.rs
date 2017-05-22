#![cfg_attr(test, feature(custom_attribute, custom_derive, plugin))]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_xdr;

extern crate serde;
extern crate tokio_core;

pub mod xdr_rpc;
pub mod xdr_codec;
