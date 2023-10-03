use std::fmt::Error;
use futures_util::{SinkExt, StreamExt, Future};
use websocket_lite::{AsyncClient, AsyncNetworkStream, ClientBuilder, Message};
use crate::{event::{build_client_message, GraphqlWsClientEvent, GraphqlWsComplete, GraphqlWsNext, GraphqlWsError, GraphqlWsServerEvent}, GraphqlWsProtocol};


pub struct GraphqlWsClientBuilder {
    url: Box<str>
}

impl GraphqlWsClientBuilder {

    pub fn from_url(url: &str) -> GraphqlWsClientBuilder {

        GraphqlWsClientBuilder {
            url: Box::from(url),
        }
    }

    pub fn from(ws_proto: &str, host: &str, port: u16, path: &str) -> GraphqlWsClientBuilder {

        GraphqlWsClientBuilder {
            url: Box::from(format!("{}://{}:{}{}", ws_proto, host, port, path).as_str()),
        }
    }

    pub async fn connect(&self) -> Result<GraphqlWsClient, Error> {
        let mut builder = ClientBuilder::new(self.url.as_ref())
            .expect("failed to parse url");

        builder.add_header("Sec-WebSocket-Protocol".into(), GraphqlWsProtocol::OFFICIAL.get_ws_sec_protocol().into());

        let stream = builder
            .async_connect()
            .await
            .expect("failed to connect");

        let mut client = GraphqlWsClient {
            stream
        };

        client.send(GraphqlWsClientEvent::ConnectionInit).await?;
        client.wait_connection_ack().await?;
        Ok(client)
    }
}


pub struct GraphqlWsClient {

    stream: AsyncClient<Box<dyn AsyncNetworkStream + Sync + Send + Unpin + 'static>>,

}

impl GraphqlWsClient {

    pub fn get_protocol(&self) -> GraphqlWsProtocol {
        GraphqlWsProtocol::OFFICIAL
    }

    pub async fn send(&mut self, client_event: GraphqlWsClientEvent) -> Result<(), Error> {
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
                                    GraphqlWsServerEvent::ConnectionAck => {
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


pub async fn listen<NF: Future<Output = ()>, EF: Future<Output = bool>, CF: Future<Output = bool>>(
        &mut self,
        next_handler: impl Fn(GraphqlWsNext) -> NF,
        error_handler: impl Fn(GraphqlWsError) -> EF,
        complete_handler: impl Fn(GraphqlWsComplete) -> CF
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
                                    GraphqlWsServerEvent::ConnectionAck => {
                                        // do nothing
                                        println!("Got Connection Ack");
                                        continue;
                                    }
                                    GraphqlWsServerEvent::Ping => {
                                        // close connection
                                        println!("Got Ping, sending Pong");
                                        self.send(GraphqlWsClientEvent::Pong).await?
                                    }
                                    GraphqlWsServerEvent::Pong => {
                                        // do nothing
                                        println!("Got Pong");
                                        continue;
                                    }
                                    GraphqlWsServerEvent::Next(next_event) => {
                                        next_handler(next_event).await;
                                    }
                                    GraphqlWsServerEvent::Error(error_event) => {
                                        if error_handler(error_event).await {
                                            break;
                                        }
                                    }
                                    GraphqlWsServerEvent::Complete(complete_event) => {
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

    pub async fn next(&mut self) -> Result<Option<GraphqlWsServerEvent>, Error>
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
                                if let Ok(server_msg) = serde_json::from_str::<GraphqlWsServerEvent>(m_str) {
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
        self.stream.send(Message::close(None)).await.expect("failed to send websocket close");
        Ok(())
    }
}
