
struct Query;

#[derive(SimpleObject)]
struct MyObject {
    /// Value a
    a: i32,

    /// Value b
    b: i32,
    //#[graphql(skip)]
    //c: i32,
}

#[Object]
impl Query {
    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    async fn obj(&self, a: i32, b: i32) -> MyObject {
        MyObject { a: a, b: b }
    }
}

async fn run_schema() {
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    let res = schema.execute("{ obj(a: 10, b: 20) }").await;
    let json = serde_json::to_string(&res);
    match json {
        Ok(s) => println!("Fetched results: {:#?}", s),
        Err(e) => println!("Got an error: {:?}", e),
    };
    //     println!("{}", json);
    // println!("Hello, world!");
}
