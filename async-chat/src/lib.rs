use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod utils;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum FromClient {
    Join {
        group_name: Arc<String>,
    },
    Post {
        group_name: Arc<String>,
        message: Arc<String>,
    },
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum FromServer {
    Message {
        group_name: Arc<String>,
        message: Arc<String>,
    },
    Error(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fromclient_message_json() {
        use std::sync::Arc;

        let from_client = FromClient::Post {
            group_name: Arc::new("Dogs".to_string()),
            message: Arc::new("Samoyeds rock!".to_string()),
        };

        let json = serde_json::to_string(&from_client).unwrap();

        assert_eq!(
            json,
            r#"{"Post":{"group_name":"Dogs","message":"Samoyeds rock!"}}"#
        );
        assert_eq!(
            serde_json::from_str::<FromClient>(&json).unwrap(),
            from_client
        );
    }

    #[test]
    fn test_fromclient_join_json() {
        use std::sync::Arc;

        let from_client = FromClient::Join {
            group_name: Arc::new("Dogs".to_string()),
        };

        let json = serde_json::to_string(&from_client).unwrap();

        assert_eq!(json, r#"{"Join":{"group_name":"Dogs"}}"#);
        assert_eq!(
            serde_json::from_str::<FromClient>(&json).unwrap(),
            from_client
        );
    }

    #[test]
    fn test_fromserver_json() {
        use std::sync::Arc;

        let from_server = FromServer::Message {
            group_name: Arc::new("Dogs".to_string()),
            message: Arc::new("Samoyeds rock!".to_string()),
        };

        let json = serde_json::to_string(&from_server).unwrap();

        assert_eq!(
            json,
            r#"{"Message":{"group_name":"Dogs","message":"Samoyeds rock!"}}"#
        );
        assert_eq!(
            serde_json::from_str::<FromServer>(&json).unwrap(),
            from_server
        );
    }

    #[test]
    fn test_fromserver_error_json() {
        let from_server = FromServer::Error("error message".to_string());

        let json = serde_json::to_string(&from_server).unwrap();

        assert_eq!(json, r#"{"Error":"error message"}"#);
        assert_eq!(
            serde_json::from_str::<FromServer>(&json).unwrap(),
            from_server
        );
    }
}
