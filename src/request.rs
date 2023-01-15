use serde::{ser::SerializeStruct, Deserialize, Serialize};

use crate::JSONRPC_V2;

/// Represents an rpc call to a Server.
///
/// The Request object has the following members:
///
/// **jsonrpc**
///
/// A String specifying the version of the JSON-RPC protocol. MUST be exactly "2.0".
///
/// **method**
///
/// A String containing the name of the method to be invoked. Method names that begin with the word rpc followed by a period character (U+002E or ASCII 46) are reserved for rpc-internal methods and extensions and MUST NOT be used for anything else.
///
/// **params**
///
/// A Structured value that holds the parameter values to be used during the invocation of the method. This member MAY be omitted.
///
/// **id**
///
/// An identifier established by the Client that MUST contain a String, Number, or NULL value if included. If it is not included it is assumed to be a notification. The value SHOULD normally not be Null[^1] and Numbers SHOULD NOT contain fractional parts [^2]
/// The Server MUST reply with the same value in the Response object if included. This member is used to correlate the context between the two objects.
///
/// [^1]: The use of Null as a value for the id member in a Request object is discouraged, because this specification uses a value of Null for Responses with an unknown id. Also, because JSON-RPC 1.0 uses an id value of Null for Notifications this could cause confusion in handling.
///
/// [^2]: Fractional parts may be problematic, since many decimal fractions cannot be represented exactly as binary fractions.
#[derive(Deserialize, PartialEq, Eq, Hash, Debug, Clone)]
pub struct Request<P> {
    pub method: String,
    pub params: P,
    pub id: Option<u64>,
}

impl<P> Request<P> {
    pub fn new(method: impl Into<String>, params: P, id: Option<u64>) -> Self {
        Self {
            method: method.into(),
            params,
            id,
        }
    }
}

impl<P: Serialize> Serialize for Request<P> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Request", 4)?;
        state.serialize_field("jsonrpc", JSONRPC_V2)?;
        state.serialize_field("method", &self.method)?;
        state.serialize_field("params", &self.params)?;
        state.serialize_field("id", &self.id)?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::{snapshot, Params};

    #[test]
    fn test_request_serde() {
        snapshot!(Request::new("method", (), None));
        snapshot!(Request::new("method", (), Some(1)));
        snapshot!(Request::new("method", vec![0, 1], None));
        snapshot!(Request::new("method", vec![0, 1], Some(1)));
        snapshot!(Request::new("method", Params { p0: 0, p1: 1 }, None));
        snapshot!(Request::new("method", Params { p0: 0, p1: 1 }, Some(1)));
    }
}
