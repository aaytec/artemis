
use std::fmt::Error;
use artemis::{ApolloClient, ApolloClientBuilder, ApolloClientEvent, ApolloServerEvent};


#[tokio::test]
async fn test_worker() {

    let workers: u16 = 1000;
    let requests: u16 = 100;
    let mut futs = Vec::new();
    for wid in 0..workers {

        let fut = async move {

            let _ = worker(wid, requests).await;

        };
        futs.push(fut);
    }

    futures::future::join_all(futs).await;
}

#[tokio::test]
async fn test_worker2() -> Result<(), Error> {

    let ws_proto: &'static str = "ws";
    let host: &'static str = "localhost";
    let port: u16 = 8080;
    let path: &'static str = "/graphql-ws";
    let mut client: Box<ApolloClient> = Box::new(ApolloClientBuilder::from(ws_proto, host, port, path).connect().await?);

    let query: Box<str> = "query { bookById(id: \"book-1\") { id name pageCount author { id firstName lastName } } }".into();
    println!("{}", query);
    client.send(ApolloClientEvent::Start(artemis::ApolloStart::new("1", query)))
        .await
        .expect("failed to send start message");

    let mut data_res: Option<artemis::ApolloData> = None;
    while let Ok(next_res) = client.next().await {
        if let Some(next_msg) = next_res {
            match next_msg {
                ApolloServerEvent::ConnectionAck => {},
                ApolloServerEvent::ConnectionError => {
                    println!("got Connection Error, closing connection");
                    client.close().await?
                },
                ApolloServerEvent::Data(data) => {
                    if (*data.id).eq("1") {
                        data_res = Some(data);
                    }
                },
                ApolloServerEvent::KeepAlive => {},
                ApolloServerEvent::Error(error) => {

                    if let Some(id) = error.id {
                        if (*id).eq("1") {
                            println!("got Error, closing connection");
                            client.close().await?
                        }
                    }
                    
                },
                ApolloServerEvent::Complete(complete) => {
                    if (*complete.id).eq("1") {
                        break;
                    }
                },
            }
        }
    }
    
    println!("{:?}", data_res);

    Ok(())
}

async fn _worker(client: &mut ApolloClient) -> Result<(), Error> {
    let query: Box<str> = "query { bookById(id: \"book-1\") { id name pageCount author { id firstName lastName } } }".into();
    let query_id = "1";
    println!("{}", query);
    client.send(ApolloClientEvent::Start(artemis::ApolloStart::new(query_id, query)))
        .await
        .expect("failed to send start message");

    client.listen(
        |_data: artemis::ApolloData| {

            async move {
                // println!("Got Data for {}, payload data: {:?}", data.id, data.payload.data);
            }
        },
        |error: artemis::ApolloError| {
            async move {
                println!("Got Error for {:?}", error.id);
                let error_id_op = error.id.clone();
                if let Some(error_id) = error_id_op {

                    // if error is for matching id, exit loop
                    error_id.as_ref().eq(query_id)
                }
                else {

                    // if found error for unknown id, exit loop
                    true
                }
            }
        },
        |complete| {
            async move {
                // println!("Got Complete for {}", complete.id);

                // if complete is for matching id, exit loop
                complete.id.as_ref().eq(query_id)
            }
        }
    ).await?;

    Ok(())
}

async fn worker(wid: u16, requests: u16) -> Result<(), Error> {
    let mut client: ApolloClient = ApolloClientBuilder::from_url("ws://localhost:8080/graphql-ws").connect().await?;
        
    for _ in 0..requests {
        match _worker(&mut client).await {
            Ok(_) => {},
            Err(_) => {
                println!("failed to do worker, worker id: {}", wid)
            },
        }
    } 


    match client.close().await {
        Ok(_) => {},
        Err(_) => {
            println!("failed to close connection, worker id: {}", wid)
        },
    };

    Ok(())
}
