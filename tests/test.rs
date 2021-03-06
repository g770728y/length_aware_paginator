#[macro_use]
extern crate diesel;

use diesel::pg::PgConnection;
use diesel::Connection;
use diesel::QueryDsl;
use length_aware_paginator::{LoadPaginated, Response};
use serde::{Deserialize, Serialize};

/// Get the database connection
/// *panics* if no DATABASE_URL is defined in the env or if the db is unreachable
fn get_connection() -> PgConnection {
    let database_url =
        dotenv::var("DATABASE_URL").expect("You have to provide DATABASE_URL to run tests");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", &database_url))
}

// schema.rs : autogenerated by diesel after running migration
table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        password -> Varchar,
    }
}

// user.rs : your model for the table represented in schema.rs
#[derive(Queryable, Deserialize, Serialize)]
pub struct User {
    id: i32,
    email: String,
    first_name: String,
    last_name: String,
    password: String,
}

/// Return paginated object with users
/// *panics* if any errors are returned by the query
fn get_paginated_users(
    connection: &PgConnection,
    page: Option<i64>,
    per_page: Option<i64>,
) -> Response<User> {
    // Use `length_aware_paginator::LoadPaginated` trait to enable
    // using the `load_paginated` method on your query.
    // Your query will return `length_aware_paginator::Response<T>` struct
    users::table
        .into_boxed()
        .load_paginated(connection, page, per_page)
        .unwrap()
}

#[test]
fn test_orm_query_pagination() {
    let connection = get_connection();

    let response: Response<User> = get_paginated_users(&connection, Some(1), Some(10));

    assert_eq!(response.page, 1);
    assert_eq!(response.per_page, 10);
    assert_eq!(response.total, 15);
    assert_eq!(response.last_page, 2);
    assert_eq!(response.data.len(), 10);
}

// TODO: Figure out a way to make this happen...
// #[test]
// fn test_sql_query_pagination() {
//     let connection = get_connection();

//     let response: Response<User> = diesel::sql_query("SELECT * FROM users")
//         .load_paginated(&connection, Some(1), Some(2))
//         .unwrap();

//     assert_eq!(response.page, 1);
// }
