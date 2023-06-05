use crate::{db_connection::DbPool, mocks::api};
use crate::errors::ApiError;
use crate::mocks::api::PartSearchAPI;
use crate::ActingUserId;
use crate::db::requests::SearchRequest;
use crate::db::users::User;
use crate::search::highlighting::highlight_search_query_in_mpn;
use actix_web::{web, HttpResponse};

pub async fn search_by_mpn(
    db_pool: web::Data<DbPool>,
    part_api: web::Data<PartSearchAPI>,
    user_id: web::Data<ActingUserId>,
    search_query: web::Query<String>,
) -> Result<HttpResponse, ApiError> {
    if search_query.is_empty() {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let conn = db_pool.get()?;
    let api_key = User::find(&conn, user_id.0)?.get_api_key().to_owned();
    let response;

    if let Some(_result) = SearchRequest::check_cached_result(&conn,api_key.clone() , &search_query)? {
        response = vec![];
    }
    else {
        response = part_api.search(search_query.as_str(), api_key.as_str());
        if response.is_empty() {
            return Ok(HttpResponse::NotFound().finish());
        }
        else{
            let new_request = SearchRequest {
                id: uuid::Uuid::new_v4(),
                api_key,
                search_string: search_query.to_owned(),
                successful: true,
            };
            SearchRequest::store_search_request(&conn, new_request)?;
        }
    }
    let mut highlighted = Vec::new();
    for line in &response {
        highlighted.push(highlight_search_query_in_mpn(line.get_mpn(), search_query.as_str()));
    }
    

    Ok(HttpResponse::Ok().json(highlighted))
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
