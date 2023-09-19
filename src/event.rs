use std::convert::Into;
use std::fmt::Error;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GraphqlWsStart {
    id: Box<str>,

    payload: Box<GraphqlWsStartPayload>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GraphqlWsStartPayload {

    #[serde(rename = "operationName")]
    pub operation_name: Box<str>,
    pub variables: serde_json::Value,
    pub query: Box<str>

}

impl GraphqlWsStart {

    pub fn new(id: &str, query: Box<str>) -> GraphqlWsStart {
        GraphqlWsStart {
            id: id.into(),
            payload: Box::new(
                GraphqlWsStartPayload {
                    operation_name: "".into(),
                    variables: serde_json::Value::Object(Default::default()),
                    query,
                }
            )
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GraphqlWsStop {
    pub id: Box<str>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GraphqlWsData {
    pub id: Box<str>,
    pub payload: Box<GraphqlWsDataPayload>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GraphqlWsDataPayload {
    pub data: serde_json::Value
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GraphqlWsError {
    pub id: Option<Box<str>>,
    pub payload: Option<Box<GraphqlWsErrorPayload>>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GraphqlWsErrorPayload {
    pub errors: serde_json::Value
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GraphqlWsComplete {
    pub id: Box<str>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum GraphqlWsClientEvent {

    #[serde(rename = "connection_init")]
    ConnectionInit,

    #[serde(rename = "connection_terminate")]
    ConnectionTerminate,

    #[serde(rename = "stop")]
    Stop(GraphqlWsStop),

    #[serde(rename = "start")]
    Start(GraphqlWsStart)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum GraphqlWsServerEvent {


    #[serde(rename = "connection_ack")]
    ConnectionAck,

    #[serde(rename = "connection_error")]
    ConnectionError,

    #[serde(rename = "data")]
    Data(GraphqlWsData),

    #[serde(rename = "ka")]
    KeepAlive,

    #[serde(rename = "error")]
    Error(GraphqlWsError),

    #[serde(rename = "complete")]
    Complete(GraphqlWsComplete)
}

pub fn build_client_message(client_event: GraphqlWsClientEvent) -> Result<Box<[u8]>, Error> {
    return match serde_json::to_string(&client_event) {
        Ok(x) => Ok(x.as_bytes().into()),
        Err(e) => {
            println!("failed to convert {:?} to string, error: {:?}", client_event, e);
            Err(Error::default())
        }
    }
}