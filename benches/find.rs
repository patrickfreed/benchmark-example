use criterion::{criterion_group, criterion_main, Criterion};
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, Document},
    Client,
};

pub fn find_bench(c: &mut Criterion) {
    // begin setup

    // create the tokio runtime to be used for the benchmarks
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    // seed the data server side, get a handle to the collection
    let collection = rt.block_on(async {
        let client = Client::with_uri_str("mongodb://localhost:27017")
            .await
            .unwrap();

        let collection = client.database("foo").collection("bar");
        collection.drop(None).await.unwrap();

        let doc = doc! {
            "hello": "world",
            "anotherKey": "anotherValue",
            "number": 1234
        };
        let docs = vec![&doc; 10_000];
        collection.insert_many(docs, None).await.unwrap();
        collection
    });
    // end setup

    c.bench_function("find", |b| {
        b.to_async(&rt).iter(|| {
            // begin measured portion of benchmark
            async {
                collection
                    .find(doc! {}, None)
                    .await
                    .unwrap()
                    .try_collect::<Vec<Document>>()
                    .await
                    .unwrap();
            }
        })
    });
}

criterion_group!(benches, find_bench);
criterion_main!(benches);
