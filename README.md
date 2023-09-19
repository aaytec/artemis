# Artemis
Graph QL WebSocket Library

## Example
```rust

/* import */
use artemis::{Client, ClientBuilder};


/* code */
let mut client: Client = ClientBuilder::from_url("ws://localhost:8080/graphql-ws").connect().await?;

let query: Box<str> = Box::new("query { bookById(id: \"book-1\") { id name pageCount author { id firstName lastName } } }");
let query_id = "1";
client.send(ClientEvent::Start(artemis::Start::new(query_id, query))).await?;

client.listen(
    |data: artemis::Data| {
        async move {
            // handle data
        }
    },
    |error: artemis::Error| {
        async move {
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
    |complete: artemis::Complete| {
        async move {
            // if complete is for matching id, exit loop
            complete.id.as_ref().eq(query_id)
        }
    }
).await?;

```