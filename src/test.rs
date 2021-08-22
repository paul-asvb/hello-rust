#[cfg(test)]
mod tests {
    use crate::create_actix_app;

    use super::*;
    use actix_web::{test, web, App};

    #[actix_rt::test]
    async fn test_index_get() {
        let mut appl = test::init_service(App::new().route("/", web::get().to(index))).await;
        let mut app = create_actix_app();
        match server {
            Ok(_s) => println!("server running"),
            Err(error) => panic!("server could not start error: {:?}", error),
        };
        let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }
}
