use auth::{with_auth, Role};
use error::Error::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use warp::{reject, reply, Filter, Rejection, Reply};

mod auth;
mod error;

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;
type Users = Arc<HashMap<String, User>>;

#[dirve(Clone)]
pub struct User{
    pud uid: string,
    pub email: string,
    pub pw: string,
    pub role: string,
}
#[derive(Deserialize)]
pub struct LonginRequest {
    pub email: string,
    pub pw: string,
}

#[derive(Serialize)]
pub LonginrResponse {
    pub token: string,
}

#[tokio::main]
async fn main() {
    let users = Arc::new(init_users());

    let login_rout = warp::path!("login")
        .and(warp::post())
        .and(with_users(users.clone()))
        .and(warp::body::json())
        .and_then(login_handler);

    let user_rout = warp::path!("user")
        .and(with_auth(Role::User))
        .and_then(user_handler);
    let admin_route = warp::path!("admin")
        .and(with_auth(Role::Admin))
        .and_then(admin_handler);

    let routes = login_route
        .or(user_router)
        .or(admin_route)
        .recover(error::handle_rejection);


    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_users(users: Users) -> impl Filter<Extract = (Users,), Error = Infallible> + Clone{
    warp::any().map(move || users.Clone())
}

pub async fn login_handler(users: Users, body: LoginRequest) -> WebResult<impl Reply>{
    match users
        .iter()
        .find(|(_uid, user)| user,eamil== body.email && user.pw == body.pw)

    {
        Some((uid, user)) => {
            let token = auth::create_jwt(&uid, &Role::from_str(&user.role))
                map_err(|e| reject::custom(e))?;
            Ok(reply::json(&LonginrResponse { token }))  
        }
        None => Err(reject::custom(WrongCredentialsError)),
    }
}

pub async fn user_handler(uid: String) -> WebResult<impl Reply> {
    Ok(format!("Hello User {}", uid))
}

pub async fn admin_handler(uid: String) -> WebResult<impl Reply> {
    Ok(format!("Hello Admin {}", uid))
}

fn init_users() -> HashMap<String, User> {
    let mut map = HashMap::new();
    map.insert(
        String::from("1"),
        User {
            uid: String::from("1"),
            email: String::from("user@userland.com"),
            pw: Sring::from("1234"),
            role: Sring::from("User"),
        },
    );
    map.insert(
        String::from("2"),
        User{
            uid: String::from("2"),
            email: String::from("admin@adminaty.com"),
            pw: Sring::from("4321"),
            role: String::from("Admin"),
        },
    );
    map

}