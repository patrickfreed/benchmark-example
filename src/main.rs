use futures::TryStreamExt;
use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};

/// Insert 10k docs into the collection.
///
/// Use this function to insert some test data so that the `find` below has some
/// work to do. To reduce noise, don't run this while generating flamegraphs.
#[allow(dead_code)]
async fn seed_data(coll: &Collection<Document>) -> mongodb::error::Result<()> {
    let doc = doc! {
        "hello": "world",
        "anotherKey": "anotherValue",
        "number": 1234
    };
    let docs = vec![&doc; 10_000];
    coll.insert_many(docs, None).await.unwrap();

    Ok(())
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let client = Client::with_uri_str("mongodb://localhost:27017").await?;
    let coll = client.database("perftest").collection::<Document>("corpus");

    coll.find(doc! {}, None)
        .await
        .unwrap()
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    Ok(())
}
