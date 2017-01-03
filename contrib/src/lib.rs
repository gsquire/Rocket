//! This crate contains officially sanctioned contributor libraries that provide
//! functionality commonly used by Rocket applications.
//!
//! These libraries are always kept in-sync with the core Rocket library. They
//! provide common, but not fundamental, abstractions to be used by Rocket
//! applications. In particular, contributor libraries typically export types
//! that implement the `FromRequest` trait, `Responder` trait, or both.
//!
//! Each module in this library is held behind a feature flag, with the most
//! common modules exposed by default. The present feature list is below, with
//! an asterisk next to the features that are enabled by default:
//!
//! * [json*](struct.JSON.html)
//! * [handlebars_templates](struct.Template.html)
//! * [tera_templates](struct.Template.html)
//!
//! The recommend way to include features from this crate via Cargo in your
//! project is by adding a `[dependencies.rocket_contrib]` section to your
//! `Cargo.toml` file, setting `default-features` to false, and specifying
//! features manually. For example, to use the JSON module, you would add:
//!
//! ```toml,ignore
//! [dependencies.rocket_contrib]
//! version = "*"
//! default-features = false
//! features = ["json"]
//! ```
//!
//! This crate is expected to grow with time, bringing in outside crates to be
//! officially supported by Rocket.

#[macro_use] extern crate log;
#[macro_use] extern crate rocket;

#[cfg_attr(feature = "lazy_static_macro", macro_use)]
#[cfg(feature = "lazy_static_macro")]
extern crate lazy_static;

#[cfg_attr(feature = "json", macro_use)]
#[cfg(feature = "json")]
#[doc(hidden)]
pub mod json;

#[cfg(feature = "templates")]
mod templates;

#[cfg(feature = "json")]
pub use json::JSON;
#[cfg(feature = "json")]
pub use json::Value;

#[cfg(feature = "templates")]
pub use templates::Template;
