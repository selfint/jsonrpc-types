use serde::{ser::SerializeStruct, Deserialize, Serialize};

use crate::JSONRPC_V2;

/// A Notification is a Request object without an "id" member.
///
/// A Request object that is a Notification signifies the Client's lack
/// of interest in the corresponding Response object, and as such no
/// Response object needs to be returned to the client.
///
/// The Server MUST NOT reply to a Notification, including those that are
/// within a batch request.
///
/// Notifications are not confirmable by definition, since they
/// do not have a Response object to be returned. As such, the
/// Client would not be aware of any errors (like e.g. "Invalid
/// params","Internal error").
#[derive(Deserialize, PartialEq, Eq, Hash, Debug, Clone)]
pub struct Notification<P> {
    pub method: String,
    pub params: P,
}

impl<P> Notification<P> {
    pub fn new(method: impl Into<String>, params: P) -> Self {
        Self {
            method: method.into(),
            params,
        }
    }
}

impl<P: Serialize> Serialize for Notification<P> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Notification", 3)?;
        state.serialize_field("jsonrpc", JSONRPC_V2)?;
        state.serialize_field("method", &self.method)?;
        state.serialize_field("params", &self.params)?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_utils::{snapshot, Params};

    #[test]
    fn test_notification_serde() {
        snapshot!(Notification::new("method", ()));
        snapshot!(Notification::new("method", vec![0, 1]));
        snapshot!(Notification::new("method", Params { p0: 0, p1: 1 }));
    }
}
