use std::fmt::Error;
use futures_util::{SinkExt, StreamExt, Future};
use websocket_lite::{AsyncClient, AsyncNetworkStream, ClientBuilder, Message};
use crate::{apollo_event::{build_client_message, ApolloGraphqlWsClientEvent, ApolloGraphqlWsComplete, ApolloGraphqlWsData, ApolloGraphqlWsError, ApolloGraphqlWsServerEvent}, GraphqlWsProtocol};


pub struct ApolloGraphqlWsClientBuilder {
    url: Box<str>
}

impl ApolloGraphqlWsClientBuilder {

    pub fn from_url(url: &str) -> ApolloGraphqlWsClientBuilder {

        ApolloGraphqlWsClientBuilder {
            url: Box::from(url),
        }
    }

    pub fn from(ws_proto: &str, host: &str, port: u16, path: &str) -> ApolloGraphqlWsClientBuilder {

        ApolloGraphqlWsClientBuilder {
            url: Box::from(format!("{}://{}:{}{}", ws_proto, host, port, path).as_str()),
        }
    }

    pub async fn connect(&self) -> Result<ApolloGraphqlWsClient, Error> {
        let mut builder = ClientBuilder::new(self.url.as_ref())
            .expect("failed to parse url");

        builder.add_header("Sec-WebSocket-Protocol".into(), GraphqlWsProtocol::APOLLO.get_ws_sec_protocol().into());

        let stream = builder
            .async_connect()
            .await
            .expect("failed to connect");

        let mut client = ApolloGraphqlWsClient {
            stream
        };

        client.send(ApolloGraphqlWsClientEvent::ConnectionInit).await?;
        client.wait_connection_ack().await?;
        Ok(client)
    }
}


pub struct ApolloGraphqlWsClient {

    stream: AsyncClient<Box<dyn AsyncNetworkStream + Sync + Send + Unpin + 'static>>,

}

impl ApolloGraphqlWsClient {

    pub fn get_protocol(&self) -> GraphqlWsProtocol {
        GraphqlWsProtocol::APOLLO
    }

    pub async fn send(&mut self, client_event: ApolloGraphqlWsClientEvent) -> Result<(), Error> {
        self.stream.send(Message::binary(build_client_message(client_event)?)).await.expect("failed to send message");
        return Ok(())
    }

    async fn wait_connection_ack(&mut self) -> Result<(), Error> {

        while let Some(msg) = self.stream.next().await {
            match msg {
                Ok(m) => {
                    match m.as_text() {
                        None => {
                            println!("failed to get Message as text from Websocket session");
                            continue;
                        }
                        Some(m_str) => {
                            if let Ok(server_msg) = serde_json::from_str(m_str) {
                                match server_msg {
                                    ApolloGraphqlWsServerEvent::ConnectionAck => {
                                        // do nothing
                                        println!("Got Connection Ack");
                                        break;
                                    }
                                    _ => {
                                        continue
                                    }
                                }
                            } else {
                                println!("failed to parse message as Server Event");
                                continue;
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("failed to get Message from Websocket session, closing connection, error: {}", e);
                    self.close().await?;
                }
            }
        }

        Ok(())
    }


pub async fn listen<DF: Future<Output = ()>, EF: Future<Output = bool>, CF: Future<Output = bool>>(
        &mut self,
        data_handler: impl Fn(ApolloGraphqlWsData) -> DF,
        error_handler: impl Fn(ApolloGraphqlWsError) -> EF,
        complete_handler: impl Fn(ApolloGraphqlWsComplete) -> CF
    ) -> Result<(), Error>
    {
        while let Some(msg) = self.stream.next().await {
            match msg {
                Ok(m) => {
                    match m.as_text() {
                        None => {
                            println!("failed to get Message as text from Websocket session");
                            continue;
                        }
                        Some(m_str) => {
                            if let Ok(server_msg) = serde_json::from_str(m_str) {
                                match server_msg {
                                    ApolloGraphqlWsServerEvent::ConnectionAck => {
                                        // do nothing
                                        println!("Got Connection Ack");
                                        continue;
                                    }
                                    ApolloGraphqlWsServerEvent::ConnectionError => {
                                        // close connection
                                        println!("Got Connection Error, closing connection");
                                        self.close().await?
                                    }
                                    ApolloGraphqlWsServerEvent::KeepAlive => {
                                        // do nothing
                                        println!("Got Keep Alive");
                                        continue;
                                    }
                                    ApolloGraphqlWsServerEvent::Data(data_event) => {
                                        data_handler(data_event).await;
                                    }
                                    ApolloGraphqlWsServerEvent::Error(error_event) => {
                                        if error_handler(error_event).await {
                                            break;
                                        }
                                    }
                                    ApolloGraphqlWsServerEvent::Complete(complete_event) => {
                                        if complete_handler(complete_event).await {
                                            break;
                                        }
                                    }
                                }
                            }
                            else {
                                println!("failed to parse message as Server Event");
                                continue;
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("failed to get Message from Websocket session, closing connection, error: {}", e);
                    self.close().await?;
                }
            }
        }

        Ok(())
    }

    pub async fn next(&mut self) -> Result<Option<ApolloGraphqlWsServerEvent>, Error>
    {
        if let Some(msg) = self.stream.next().await {
            match msg {
                    Ok(m) => {
                        match m.as_text() {
                            None => {
                                println!("failed to get Message as text from Websocket session");
                                Err(Error::default())
                            }
                            Some(m_str) => {
                                if let Ok(server_msg) = serde_json::from_str::<ApolloGraphqlWsServerEvent>(m_str) {
                                    Ok(Some(server_msg))
                                }
                                else {
                                    println!("failed to parse message as Server Event");
                                    Ok(None)
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("failed to get Message from Websocket session, closing connection, error: {}", e);
                        Err(Error::default())
                    }
                }
        }
        else {
            println!("Got last Message from Websocket session");
            Ok(None)
        }
    }

    pub async fn close(&mut self) -> Result<(), Error> {
        self.send(ApolloGraphqlWsClientEvent::ConnectionTerminate).await.expect("failed to send graphql-ws connection_terminate");
        self.stream.send(Message::close(None)).await.expect("failed to send websocket close");
        Ok(())
    }
}
