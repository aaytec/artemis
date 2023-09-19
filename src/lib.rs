mod event;
pub mod client;

pub type ClientBuilder = client::GraphqlWsClientBuilder;
pub type Client = client::GraphqlWsClient;
pub type ServerEvent = event::GraphqlWsServerEvent;
pub type ClientEvent = event::GraphqlWsClientEvent;
pub type Data = event::GraphqlWsData;
pub type Start = event::GraphqlWsStart;
pub type Error = event::GraphqlWsError;
pub type Complete = event::GraphqlWsComplete;
pub type Stop = event::GraphqlWsStop;