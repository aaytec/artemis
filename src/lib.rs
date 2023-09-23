#[derive(Clone, Copy)]
pub enum GraphqlWsProtocol {
    APOLLO,
    OFFICIAL
}

impl GraphqlWsProtocol {
    
    fn get_ws_sec_protocol(&self) -> &'static str {
        return match self {
            GraphqlWsProtocol::APOLLO => "graphql-ws",
            GraphqlWsProtocol::OFFICIAL => "graphql-transport-ws",
        }
    }

}

mod apollo_event;
pub mod apollo_client;

pub type ApolloClientBuilder = apollo_client::ApolloGraphqlWsClientBuilder;
pub type ApolloClient = apollo_client::ApolloGraphqlWsClient;
pub type ApolloServerEvent = apollo_event::ApolloGraphqlWsServerEvent;
pub type ApolloClientEvent = apollo_event::ApolloGraphqlWsClientEvent;
pub type ApolloData = apollo_event::ApolloGraphqlWsData;
pub type ApolloStart = apollo_event::ApolloGraphqlWsStart;
pub type ApolloError = apollo_event::ApolloGraphqlWsError;
pub type ApolloComplete = apollo_event::ApolloGraphqlWsComplete;
pub type ApolloStop = apollo_event::ApolloGraphqlWsStop;

mod client;
pub mod event;

pub type ClientBuilder = client::GraphqlWsClientBuilder;
pub type Client = client::GraphqlWsClient;
pub type ServerEvent = event::GraphqlWsServerEvent;
pub type ClientEvent = event::GraphqlWsClientEvent;
pub type Next = event::GraphqlWsNext;
pub type Subscribe = event::GraphqlWsSubscribe;
pub type Error = event::GraphqlWsError;
pub type Complete = event::GraphqlWsComplete;
