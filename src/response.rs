use serde::{ser::SerializeStruct, Deserialize, Serialize};

use crate::JSONRPC_V2;

/// When a rpc call is made, the Server MUST reply with a Response, except for in the case of Notifications.
///
/// The Response is expressed as a single JSON Object, with the following members:
///
/// **jsonrpc**
///
/// A String specifying the version of the JSON-RPC protocol. MUST be exactly "2.0".
///
/// **result**
///
/// This member is REQUIRED on success.
/// This member MUST NOT exist if there was an error invoking the method.
/// The value of this member is determined by the method invoked on the Server.
///
/// **error**
///
/// This member is REQUIRED on error.
/// This member MUST NOT exist if there was no error triggered during invocation.
/// The value for this member MUST be an Object as defined in section 5.1.
///
/// **id**
///
/// This member is REQUIRED.
/// It MUST be the same as the value of the id member in the Request Object.
/// If there was an error in detecting the id in the Request object (e.g. Parse error/Invalid Request), it MUST be Null.
///
/// Either the result member or error member MUST be included, but both members MUST NOT be included.
#[derive(Deserialize, PartialEq, Eq, Hash, Debug, Clone)]
pub struct Response<R, E> {
    /// Contains the **result** or **error** contents.
    #[serde(flatten)]
    pub content: ResponseContent<R, E>,
    pub id: Option<u64>,
}

impl<R, E> Response<R, E> {
    pub fn new(content: ResponseContent<R, E>, id: Option<u64>) -> Self {
        Self { content, id }
    }
}

/// Contains either the **result** or **error** content of a [`Response`].
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ResponseContent<R, E> {
    Result(R),
    Error(ResponseError<E>),
}

/// Contents of an **error** response.
///
/// When a rpc call encounters an error, the Response Object MUST contain the
/// error member with a value that is a Object with the following members:
///
/// **code**
///
/// A Number that indicates the error type that occurred.
/// This MUST be an integer.
///
/// **message**
///
/// A String providing a short description of the error.
/// The message SHOULD be limited to a concise single sentence.
///
/// **data**
///
/// A Primitive or Structured value that contains additional information about
/// the error.
///
/// This may be omitted.
///
/// The value of this member is defined by the Server (e.g. detailed error
/// information, nested errors etc.).
///
/// The error codes from and including -32768 to -32000 are reserved for
/// pre-defined errors. Any code within this range, but not defined explicitly
/// below is reserved for future use.
///
/// The error codes are nearly the same as those suggested for XML-RPC at the
/// following url: <http://xmlrpc-epi.sourceforge.net/specs/rfc.fault_codes.php>.
///
/// | code | message | meaning |
/// |------|---------|---------|
/// | -32700 | Parse error | Invalid JSON was received by the server.<br />An error occurred on the server while parsing the JSON text. |
/// | -32600 | Invalid Request | The JSON sent is not a valid Request object. |
/// | -32601 | Method not found | The method does not exist / is not available. |
/// | -32602 | Invalid params | Invalid method parameter(s). |
/// | -32603 | Internal error | Internal JSON-RPC error. |
/// | -32000 to -32099 | Server error | Reserved for implementation-defined server-errors. |
///
/// The remainder of the space is available for application defined errors.
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone)]
pub struct ResponseError<D> {
    pub code: i64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<D>,
}

impl<R: Serialize, E: Serialize> Serialize for Response<R, E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Response", 3)?;
        state.serialize_field("jsonrpc", JSONRPC_V2)?;

        // flatten result
        match &self.content {
            ResponseContent::Result(r) => state.serialize_field("result", r)?,
            ResponseContent::Error(e) => state.serialize_field("error", e)?,
        }

        state.serialize_field("id", &self.id)?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::{snapshot, Params};

    #[test]
    fn test_response_serde() {
        macro_rules! snapshot_permutations {
            ($data:expr) => {
                snapshot!(Response::new(
                    ResponseContent::<_, ()>::Result($data),
                    Some(1)
                ));
                snapshot!(Response::new(ResponseContent::<_, ()>::Result($data), None));
                snapshot!(Response::new(
                    ResponseContent::<(), _>::Error(ResponseError {
                        code: -1,
                        message: "message".to_string(),
                        data: Some($data)
                    }),
                    Some(1)
                ));
                snapshot!(Response::new(
                    ResponseContent::<(), ()>::Error(ResponseError {
                        code: -1,
                        message: "message".to_string(),
                        data: None
                    }),
                    Some(1)
                ));
                snapshot!(Response::new(
                    ResponseContent::<(), _>::Error(ResponseError {
                        code: -1,
                        message: "message".to_string(),
                        data: Some($data)
                    }),
                    None
                ));
                snapshot!(Response::new(
                    ResponseContent::<(), ()>::Error(ResponseError {
                        code: -1,
                        message: "message".to_string(),
                        data: None
                    }),
                    None
                ));
            };
        }

        snapshot_permutations!(1);
        snapshot_permutations!(vec![1, -1]);
        snapshot_permutations!(Params { p0: 0, p1: 1 });
    }
}
