use actix_web::Result;
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};

struct Query;

#[Object]
impl Query {
    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}
#[async_std::main]

async fn main() {
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    let res = schema.execute("{ add(a: 10, b: 20) }").await;
    let json = serde_json::to_string(&res);
    match json {
        Ok(s) => println!("Fetched results: {:#?}", s),
        Err(e) => println!("Got an error: {:?}", e),
    };
    //     println!("{}", json);
    // println!("Hello, world!");
}

// async fn index(schema: web::Data<StarWarsSchema>, req: Request) -> Response {
//     schema.execute(req.into_inner()).await.into()
// }

// async fn index_playground() -> Result<HttpResponse> {
//     Ok(HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body(playground_source(
//             GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
//         )))
// }

// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
//         .data(StarWars::new())
//         .finish();

//     println!("Playground: http://localhost:8000");

//     HttpServer::new(move || {
//         App::new()
//             .data(schema.clone())
//             .service(web::resource("/").guard(guard::Post()).to(index))
//             .service(web::resource("/").guard(guard::Get()).to(index_playground))
//     })
//     .bind("127.0.0.1:8000")?
//     .run()
//     .await
// }
