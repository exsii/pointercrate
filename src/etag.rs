//! Module for dealing with pointercrate ETags.
//!
//! Note that the format described here is **not part of the public API**.
//!
//! A pointercrate ETag value has two parts: A part relevant for `PATCH` requests, which is a hash
//! of all fields that can be modified via a direct `PATCH` request to the object represented, and a
//! part relevant for `GET` requests, which is generally just a hash of the complete objects.
//!
//! These two parts are unsigned 64 bit integers separated by a semicolon (`;`)
//!
//! The idea is that for `GET` requests only the second part of the ETag is used to determine if a
//! 304 response should be generated, while for `PATCH` requests only the first part is used to
//! determine whether a `412` should be returned.
//!
//! The difference between `GET` and `PATCH` ETag is important for objects where specific subfields
//! are not modifiable via `PATCH` (e.g. the record list of a player), so having changes to them
//! cause a `412` is silly, yet for caching purposes, those parts are obviously important.

use actix_web::{dev::HttpResponseBuilder, HttpResponse};
use serde::Serialize;

pub use pointercrate_core::etag::Taggable;

pub trait HttpResponseBuilderEtagExt {
    fn etag<H: Taggable>(&mut self, obj: &H) -> &mut Self;
    fn json_with_etag<H: Serialize + Taggable>(&mut self, obj: &H) -> HttpResponse;
}

impl HttpResponseBuilderEtagExt for HttpResponseBuilder {
    fn etag<H: Taggable>(&mut self, obj: &H) -> &mut Self {
        self.header("ETag", obj.etag_string())
    }

    fn json_with_etag<H: Serialize + Taggable>(&mut self, obj: &H) -> HttpResponse {
        self.etag(obj).json(serde_json::json!({ "data": obj }))
    }
}
