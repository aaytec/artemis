use std::convert::Into;
use std::fmt::Error;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApolloGraphqlWsStart {
    id: Box<str>,

    payload: Box<ApolloGraphqlWsStartPayload>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApolloGraphqlWsStartPayload {

    #[serde(rename = "operationName")]
    pub operation_name: Box<str>,
    pub variables: serde_json::Value,
    pub query: Box<str>

}

impl ApolloGraphqlWsStart {

    pub fn new(id: &str, query: Box<str>) -> ApolloGraphqlWsStart {
        ApolloGraphqlWsStart {
            id: id.into(),
            payload: Box::new(
                ApolloGraphqlWsStartPayload {
                    operation_name: "".into(),
                    variables: serde_json::Value::Object(Default::default()),
                    query,
                }
            )
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApolloGraphqlWsStop {
    pub id: Box<str>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApolloGraphqlWsData {
    pub id: Box<str>,
    pub payload: Box<ApolloGraphqlWsDataPayload>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApolloGraphqlWsDataPayload {
    pub data: serde_json::Value
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApolloGraphqlWsError {
    pub id: Option<Box<str>>,
    pub payload: Option<Box<ApolloGraphqlWsErrorPayload>>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApolloGraphqlWsErrorPayload {
    pub errors: serde_json::Value
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApolloGraphqlWsComplete {
    pub id: Box<str>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ApolloGraphqlWsClientEvent {

    #[serde(rename = "connection_init")]
    ConnectionInit,

    #[serde(rename = "connection_terminate")]
    ConnectionTerminate,

    #[serde(rename = "stop")]
    Stop(ApolloGraphqlWsStop),

    #[serde(rename = "start")]
    Start(ApolloGraphqlWsStart)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ApolloGraphqlWsServerEvent {


    #[serde(rename = "connection_ack")]
    ConnectionAck,

    #[serde(rename = "connection_error")]
    ConnectionError,

    #[serde(rename = "data")]
    Data(ApolloGraphqlWsData),

    #[serde(rename = "ka")]
    KeepAlive,

    #[serde(rename = "error")]
    Error(ApolloGraphqlWsError),

    #[serde(rename = "complete")]
    Complete(ApolloGraphqlWsComplete)
}

pub fn build_client_message(client_event: ApolloGraphqlWsClientEvent) -> Result<Box<[u8]>, Error> {
    return match serde_json::to_string(&client_event) {
        Ok(x) => Ok(x.as_bytes().into()),
        Err(e) => {
            println!("failed to convert {:?} to string, error: {:?}", client_event, e);
            Err(Error::default())
        }
    }
}