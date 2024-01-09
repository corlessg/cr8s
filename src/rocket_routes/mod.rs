use std::error::Error;

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use rocket::response::status::Custom;
use rocket::serde::json::{Value,json};
use rocket_db_pools::Connection;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;


use crate::models::{User, RoleCode};
use crate::repositories::{UserRepository, RoleRepository};

pub mod rustaceans;
pub mod crates;
pub mod authorization;

#[derive(rocket_db_pools::Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);

#[derive(rocket_db_pools::Database)]
#[database("redis")]
pub struct CacheConn(rocket_db_pools::deadpool_redis::Pool);


pub fn server_error(e: Box<dyn Error>) -> Custom<Value> {
    rocket::error!("{}",e);
    Custom(Status::InternalServerError,json!("Error"))
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Authorization: Bearer SESSION_ID_128_CHARACTERS_LONG
        let session_header = req.headers().get_one("Authorization")
            .map(|v| v.split_whitespace().collect::<Vec<_>>())
            .filter(|v| v.len() == 2 && v[0] == "Bearer");

        if let Some(header_value) = session_header {
            let mut cache = req.guard::<Connection<CacheConn>>().await
                .expect("Can not connection to Redis in request guard");

            let mut db = req.guard::<Connection<DbConn>>().await
            .expect("Can not connection to Postgres in request guard");

            let result = cache.get::<String,i32>(format!("sessions/{}",header_value[1])).await;
            
            if let Ok(user_id) = result {
                if let Ok(user) = UserRepository::find(&mut db, user_id).await {
                    return Outcome::Success(user);
                }
            }
        }

        Outcome::Error((Status::Unauthorized,()))
    }
}

pub struct EditorUser(User);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for EditorUser {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user  = req.guard::<User>().await
            .expect("Cannot retrieve current logged in user");

        let mut db = req.guard::<Connection<DbConn>>().await
            .expect("Can not connection to Postgres in request guard");
        
        if let Ok(roles) = RoleRepository::find_by_user(&mut db, &user).await {
            rocket::info!("Role assigned are {:?}",roles);

            let is_editor = roles.iter().any(|r| match r.code{
                RoleCode::Admin => true,
                RoleCode::Editor => true,
                _ => false
            });

            rocket::info!("Is editor is {:?}",is_editor);
            if is_editor {
                return Outcome::Success(EditorUser(user));
            };
        };

        Outcome::Error((Status::Unauthorized,()))
    }
}
