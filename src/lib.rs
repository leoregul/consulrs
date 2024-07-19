//! # consulrs
//!
//! <p align="center">
//!     <a href="https://crates.io/crates/consulrs">
//!         <img src="https://img.shields.io/crates/v/consulrs">
//!     </a>
//!     <a href="https://docs.rs/consulrs">
//!         <img src="https://img.shields.io/docsrs/consulrs" />
//!     </a>
//!     <a href="https://github.com/jmgilman/consulrs/actions/workflows/ci.yml">
//!         <img src="https://github.com/jmgilman/consulrs/actions/workflows/ci.yml/badge.svg"/>
//!     </a>
//! </p>
//!
//! > An asynchronous Rust client library for the [Hashicorp Consul][1] API
//!
//! The following features are currently supported:
//!
//! * [Catalog](https://www.consul.io/api-docs/catalogv)
//! * [Checks](https://www.consul.io/api-docs/agent/check)
//! * [KV Store](https://www.consul.io/api-docs/kv)
//! * [Services](https://www.consul.io/api-docs/agent/service)
//! * [Sessions](https://www.consul.io/api-docs/session)
//! * [Snapshots](https://www.consul.io/api-docs/snapshot)
//!
//! Additionally, all optional API features such as consistency modes, blocking,
//! etc. are also supported.
//!
//! ## Installation
//!
//! Add `consulrs` as a dependency to your cargo.toml:
//! ```ignore
//! [dependencies]
//! consulrs = "0.1.0"
//! ```
//!
//! ## Usage

//! ### Basic
//!
//! The client is used to configure the connection to Consul and is required to be
//! passed to all API calls for execution. Behind the scenes it uses an asynchronous
//! client from [Reqwest](https://docs.rs/reqwest/) for communicating to Consul.
//!
//! ```rust
//! use consulrs::client::{ConsulClient, ConsulClientSettingsBuilder};
//!
//! // Create a client
//! let client = ConsulClient::new(
//!     ConsulClientSettingsBuilder::default()
//!         .address("https://127.0.0.1:8200")
//!         .build()
//!         .unwrap()
//! ).unwrap();
//! ```
//!
//! The client supports all features required to interact with a production Consul
//! service including the option to specify ACL tokens as well as client and CA
//! certificates.
//!
//! ### Using KV store
//!
//! ```should_panic
//! # use consulrs::client::{ConsulClient, ConsulClientSettingsBuilder};
//! use std::convert::TryInto;
//! use consulrs::kv;
//!
//! # let client = ConsulClient::new(
//! #     ConsulClientSettingsBuilder::default()
//! #         .address("https://127.0.0.1:8200")
//! #         .build()
//! #         .unwrap()
//! # ).unwrap();
//!
//! # tokio_test::block_on(async {
//! // Set `mykey` to "myvalue"
//! kv::set(&client, "mykey", b"myvalue", None).await;
//!
//! // Read `mykey`
//! let mut res = kv::read(&client, "mykey", None).await.unwrap();
//!
//! // All responses are base64 encoded by default. The below attempts to coerce
//! // the response back into a UTF-8 encoded string.
//! let mykey: String = res.response.pop().unwrap().value.unwrap().try_into().unwrap();
//!
//! // In most cases, it's easier to just read the raw value
//! let res = kv::read_raw(&client, "mykey", None).await.unwrap().response;
//! let mykey = std::str::from_utf8(&res).unwrap();
//!
//! assert_eq!(mykey, "myvalue".to_string());
//! # })
//! ```
//!
//! ### Registering a service
//!
//! ```rust
//! # use consulrs::client::{ConsulClient, ConsulClientSettingsBuilder};
//! use consulrs::api::check::common::AgentServiceCheckBuilder;
//! use consulrs::api::service::requests::RegisterServiceRequest;
//! use consulrs::service;
//!
//! # let client = ConsulClient::new(
//! #     ConsulClientSettingsBuilder::default()
//! #         .address("https://127.0.0.1:8200")
//! #         .build()
//! #         .unwrap()
//! # ).unwrap();
//!
//! // Create a service named "my_service" with a health check that queries the
//! // service via HTTP every 10 seconds.
//! # tokio_test::block_on(async {
//! service::register(
//!     &client,
//!     "my_service",
//!     Some(
//!         RegisterServiceRequest::builder()
//!             .address("http://myservice.lab.com")
//!             .port(1234 as u64)
//!             .check(
//!                 AgentServiceCheckBuilder::default()
//!                     .name("health_check")
//!                     .interval("10s")
//!                     .http("http://myservice.lab.com/health")
//!                     .status("passing")
//!                     .build()
//!                     .unwrap(),
//!             ),
//!     ),
//! )
//! .await;
//! # })
//! ```
//!
//! See the [examples](examples) directory for additional examples.
//!
//! ## Error Handling and Tracing
//!
//! All errors generated by this crate are wrapped in the `ClientError` enum
//! provided by the crate. API errors are captured and returned as their own
//! variant including the response code and error message from the server.
//! Connection related errors from `rustify` are wrapped and returned as a single
//! variant.
//!
//! All top level API operations are instrumented with `tracing`'s `#[instrument]`
//! attribute.
//!
//! ## Testing
//!
//! See the the [tests](tests) directory for tests. Run tests with `cargo test`.
//!
//! **Note**: All tests rely on bringing up a local Consul development server using
//! Docker. In order to run tests Docker must be running locally (Docker Desktop
//! works).
//!
//! ## Contributing
//!
//! Check out the [issues][2] for items needing attention or submit your own and
//! then:
//!
//! 1. Fork the repo (https://github.com/jmgilman/consulrs/fork)
//! 2. Create your feature branch (git checkout -b feature/fooBar)
//! 3. Commit your changes (git commit -am 'Add some fooBar')
//! 4. Push to the branch (git push origin feature/fooBar)
//! 5. Create a new Pull Request
//!
//! [1]: https://www.consul.io/
//! [2]: https://github.com/jmgilman/consulrs/issues

#[macro_use]
extern crate tracing;

pub mod api;
pub mod catalog;
pub mod check;
pub mod client;
pub mod configuration;
pub mod error;
pub mod health;
pub mod kv;
pub mod service;
pub mod session;
pub mod snapshot;
