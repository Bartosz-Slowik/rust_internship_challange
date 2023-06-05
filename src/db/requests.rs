use crate::db::users::User;
use crate::schema::search_requests;
use diesel::prelude::*;
use diesel::PgConnection;
use uuid::Uuid;

#[derive(Debug, Queryable, Insertable)]
#[table_name = "search_requests"]
pub struct SearchRequest {
    pub id: Uuid,
    pub api_key: String,
    pub search_string: String,
    pub successful: bool,
}
//BONUS QUESTION: If the request is on a database we could index api_key_ and search_string
// If not we can store cached results in a hashmap with a api+search_string key

//I assmed i don't need to return structs with actual data structure

impl SearchRequest {

    pub fn store_search_request(
        conn: &PgConnection,
        new_request: SearchRequest,
    ) -> Result<(), diesel::result::Error> {
        diesel::insert_into(search_requests::table)
            .values(&new_request)
            .execute(conn)?;

        Ok(())
    }

    pub fn check_cached_result(
        conn: &PgConnection,
        v_api_key: String,
        v_search_string: &str,
    ) -> Result<Option<SearchRequest>, diesel::result::Error> {
        use crate::schema::search_requests::dsl::*;

        let result = search_requests
            .filter(api_key.eq(v_api_key))
            .filter(search_string.eq(v_search_string))
            .first(conn)
            .optional()?;

        Ok(result)
    }
}
#[test]
fn test_store_search_request() {
    use crate::db_connection::testing::create_testing_pool;
    let testing_pool = create_testing_pool();
    let conn = testing_pool.get().unwrap();
    let new_request = SearchRequest {
        id: uuid::Uuid::new_v4(),
        api_key: "SECRET".to_string(),
        search_string: "test".to_string(),
        successful: true,
    };
    let result = SearchRequest::store_search_request(&conn, new_request);
    assert!(result.is_ok());

}
