//! A minimal implementation of the [JSON-RPC 2.0
//! Specification](https://www.jsonrpc.org/specification) types.
//!
//! **All documentation is copied from the specification site** (with small
//! modifications).
//!
//! # 1 Overview
//! JSON-RPC is a stateless, light-weight remote procedure call (RPC) protocol.
//! Primarily this specification defines several data structures and the rules
//! around their processing. It is transport agnostic in that the concepts can
//! be used within the same process, over sockets, over http, or in many
//! various message passing environments. It uses JSON (RFC 4627) as data format.
//!
//! It is designed to be simple!
//!
//! # 2 Conventions
//! The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD",
//! "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be
//! interpreted as described in RFC 2119.
//!
//! Since JSON-RPC utilizes JSON, it has the same type system (see
//! <http://www.json.org> or RFC 4627). JSON can represent four primitive types
//! (Strings, Numbers, Booleans, and Null) and two structured types (Objects and
//! Arrays). The term "Primitive" in this specification references any of those
//! four primitive JSON types. The term "Structured" references either of the
//! structured JSON types. Whenever this document refers to any JSON type, the
//! first letter is always capitalized: Object, Array, String, Number, Boolean,
//! Null. True and False are also capitalized.
//!
//! All member names exchanged between the Client and the Server that are
//! considered for matching of any kind should be considered to be
//! case-sensitive. The terms function, method, and procedure can be assumed
//! to be interchangeable.
//!
//! The Client is defined as the origin of Request objects and the handler
//! of Response objects.
//!
//! The Server is defined as the origin of Response objects and the handler
//! of Request objects.
//!
//! One implementation of this specification could easily fill both of those
//! roles, even at the same time, to other different clients or the same client.
//! This specification does not address that layer of complexity.
//!
//! # 3 Compatibility
//! JSON-RPC 2.0 Request objects and Response objects may not work with
//! existing JSON-RPC 1.0 clients or servers. However, it is easy to
//! distinguish between the two versions as 2.0 always has a member named "jsonrpc"
//! with a String value of "2.0" whereas 1.0 does not. Most 2.0 implementations
//! should consider trying to handle 1.0 objects, even if not the peer-to-peer
//! and class hinting aspects of 1.0.

mod notification;
mod request;
mod response;

pub use notification::Notification;
pub use request::Request;
pub use response::{Response, ResponseContent, ResponseError};

pub(crate) const JSONRPC_V2: &str = "2.0";

/// Helpers for serialization/deserialization tests
#[cfg(test)]
pub(crate) mod test_utils {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
    pub(crate) struct Params {
        pub p0: u32,
        pub p1: u32,
    }

    #[cfg(test)]
    macro_rules! snapshot {
        ($e:expr) => {
            insta::assert_json_snapshot!($e);

            let serialized = serde_json::to_value($e);
            let deserialized = serde_json::from_value(serialized.unwrap()).unwrap();

            similar_asserts::assert_eq!($e, deserialized);
        };
    }

    #[cfg(test)]
    pub(crate) use snapshot;
}
