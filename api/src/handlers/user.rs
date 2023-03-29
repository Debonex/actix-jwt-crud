use crate::{
    constants::{ARGON2_SALT, TOKEN_VALID_TIME},
    error::{ServerError, ServerResult},
    middlewares::auth::AuthMiddlewareFactory,
    models::{token_claims::TokenClaims, user::UserInfo},
    services::auth::{del_token, refresh_token},
};
use actix_web::{
    post,
    web::{self, ServiceConfig},
    Error, HttpResponse,
};
use chrono::Utc;
use redis::Client;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct RegisterParams {
    name: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginParams {
    name: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
    info: UserInfo,
}

#[post("/register")]
async fn register(
    db: web::Data<DatabaseConnection>,
    form: web::Json<RegisterParams>,
) -> Result<HttpResponse, Error> {
    let argon2_config = argon2::Config::default();
    let hashed_password =
        argon2::hash_encoded(form.password.as_bytes(), ARGON2_SALT, &argon2_config).unwrap();

    let user = entity::user::ActiveModel {
        name: Set(form.name.to_owned()),
        password: Set(hashed_password),
        ..Default::default()
    };

    let res = user.insert(db.as_ref()).await;

    match res {
        Ok(model) => Ok(HttpResponse::Ok().body(model.id.to_string())),
        Err(DbErr::Exec(_)) => ServerError::UserNameUsed.into(),
        _ => ServerError::DbError.into(),
    }
}

#[post("/login")]
async fn login(
    db: web::Data<DatabaseConnection>,
    form: web::Json<LoginParams>,
    redis: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    let user = entity::user::Entity::find()
        .filter(entity::user::Column::Name.eq(&form.name))
        .one(db.as_ref())
        .await
        .map_err(|_| ServerError::DbError)?
        .ok_or(ServerError::UserNotFound)?;

    if let Ok(true) = argon2::verify_encoded(&user.password, form.password.as_bytes()) {
        let now = Utc::now().timestamp();
        let token = refresh_token(
            &redis,
            &TokenClaims {
                iat: now,
                exp: now + TOKEN_VALID_TIME as i64,
                user_id: user.id,
            },
        )
        .await?;
        return Ok(HttpResponse::Ok().json(LoginResponse {
            token,
            info: user.into(),
        }));
    }

    ServerError::UserPasswordError.into()
}

async fn info(
    db: web::Data<DatabaseConnection>,
    claims: TokenClaims,
) -> Result<HttpResponse, Error> {
    let user_info: UserInfo = find_user_by_id(&db, claims.user_id).await?.into();
    Ok(HttpResponse::Ok().json(user_info))
}

async fn logout(redis: web::Data<Client>, claims: TokenClaims) -> Result<HttpResponse, Error> {
    del_token(&redis, claims.user_id)
        .await
        .map_err(|_| ServerError::UserLogoutFailed)?;

    Ok(HttpResponse::NoContent().finish())
}

async fn add_count(
    db: web::Data<DatabaseConnection>,
    claims: TokenClaims,
) -> Result<HttpResponse, Error> {
    let user = find_user_by_id(&db, claims.user_id).await?;
    let count = user.count + 1;
    let mut new_user: entity::user::ActiveModel = user.into();
    new_user.count = Set(count);
    let user_info: UserInfo = new_user
        .update(db.as_ref())
        .await
        .map_err(|_| ServerError::DbError)?
        .into();
    Ok(HttpResponse::Ok().json(user_info))
}

async fn find_user_by_id(
    db: &DatabaseConnection,
    user_id: i32,
) -> ServerResult<entity::user::Model> {
    entity::user::Entity::find_by_id(user_id)
        .one(db)
        .await
        .map_err(|_| ServerError::DbError)?
        .ok_or(ServerError::UserNotFound)
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(register)
        .service(login)
        .route("", web::get().to(info).wrap(AuthMiddlewareFactory))
        .route(
            "/logout",
            web::post().to(logout).wrap(AuthMiddlewareFactory),
        )
        .route(
            "/add",
            web::post().to(add_count).wrap(AuthMiddlewareFactory),
        );
}
