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
    for _ in 0..10_000 {
        coll.insert_one(doc! { "x": 1 }, None).await?;
    }

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
