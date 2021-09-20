#[cfg(test)]
mod tests {
    use crate::create_actix_app;

    use super::*;
    use actix_web::{test, web, App};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
