use std::net::TcpListener;

//use actix_web::Result;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema, SimpleObject};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn start_server() -> Result<actix_web::dev::Server, Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Failed to bind port");

    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run_schema().await;
    let server = start_server().await;

    match server {
        Ok(_s) => println!("server running"),
        Err(error) => panic!("server could not start error: {:?}", error),
    };

    Ok(())
}
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
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
