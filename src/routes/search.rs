use crate::db_connection::DbPool;
use crate::errors::ApiError;
use crate::mocks::api::PartSearchAPI;
use crate::ActingUserId;
use actix_web::{web, HttpResponse};

pub async fn search_by_mpn(
    // Pool from which you can get connection
    _db_pool: web::Data<DbPool>,
    // Part api you can use for mocking response
    _part_api: web::Data<PartSearchAPI>,
    // Id of the user sending the request, you can use it to implement request caching here
    _user_id: web::Data<ActingUserId>,
    // TODO: You will need add search query parameter and return JSON response with parts with
    // highlighted MPNs
) -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().body("This doesn't seem right."))
}

#[cfg(test)]
mod tests {
    use super::*;

    mod integration {
        use super::*;
        use crate::db_connection::testing::create_testing_pool;
        use crate::{create_mock_user, mocks};
        use actix_web::middleware::Logger;
        use actix_web::{test, App};

        #[actix_rt::test]
        async fn test_request() {
            // Here we just add all relevant data to provide with the same objects we would have
            // when running the app when running the app
            let testing_pool = create_testing_pool();
            let mock_user = create_mock_user(&testing_pool).expect("Failed to create fake user");
            let mut app = test::init_service(
                App::new()
                    .wrap(Logger::default())
                    .data(testing_pool.clone())
                    .data(ActingUserId(mock_user.id))
                    .data(mocks::api::PartSearchAPI {}),
            )
            .await;

            let req = test::TestRequest::get().uri("/").to_request();
            let resp = test::call_service(&mut app, req).await;

            // There are not endpoints configured in this app so we expect the req to fail!
            assert_eq!(resp.status(), 404)
        }
    }
}
