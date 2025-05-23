#![allow(unused_imports)]

use std::env;
use std::str::FromStr;

use bson::{doc, oid::ObjectId};
use chrono::prelude::*;
use chrono::Duration;
use dioxus::prelude::*;

use crate::server::auth::model::{TokenClaims, User};
use crate::server::auth::request::EditUserSchema;
use crate::server::auth::response::{
    AuthResponse, DashboardResponse, LoginUserSchema, RegisterUserSchema, UserResponse,
};
use crate::server::book::model::Book;
use crate::server::common::response::SuccessResponse;

#[cfg(feature = "server")]
use {
    crate::db::get_client,
    argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier},
    axum_extra::extract::cookie::{Cookie, SameSite},
    jsonwebtoken::{encode, DecodingKey, EncodingKey, Header, Validation},
    rand_core::OsRng,
};

#[server]
pub async fn register_user(
    body: RegisterUserSchema,
) -> Result<SuccessResponse<UserResponse>, ServerFnError> {
    // TODO: get this from Extension(state): Extension<Arc<AppState>>,
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let user_collection = db.collection::<User>("users");

    // Check if user already exists
    if user_collection
        .find_one(doc! { "email": &body.email })
        .await?
        .is_some()
    {
        return Err(ServerFnError::new("User with that email already exists"));
    }

    // Hash password
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|_| ServerFnError::new("Error while hashing password"))?;

    // Insert new user into MongoDB
    let new_user = User {
        id: ObjectId::new(),
        name: body.name,
        email: body.email.to_lowercase(),
        password: hashed_password,
        role: "user".into(),
        photo: "".into(),
        verified: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    user_collection.insert_one(new_user.clone()).await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: UserResponse { user: new_user },
    })
}

#[server]
pub async fn login_user(
    body: LoginUserSchema,
) -> Result<SuccessResponse<AuthResponse>, ServerFnError> {
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let user_collection = db.collection::<User>("users");

    // Find the user by email
    let user = user_collection
        .find_one(doc! { "email": &body.email })
        .await?
        .ok_or(ServerFnError::new("Invalid email or password"))?;

    // Verify the password
    let parsed_hash = PasswordHash::new(&user.password)
        .map_err(|_| ServerFnError::new("Password verification error"))?;
    if !Argon2::default()
        .verify_password(body.password.as_bytes(), &parsed_hash)
        .is_ok()
    {
        return Err(ServerFnError::new("Invalid email or password"));
    }

    // Generate a JWT token
    let now = Utc::now();
    let claims = TokenClaims {
        sub: user.id.to_hex(),
        iat: now.timestamp() as usize,
        exp: (now + Duration::minutes(60)).timestamp() as usize,
    };

    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )?;

    let cookie = Cookie::build(("token", token.to_owned()))
        .path("/")
        .max_age(time::Duration::hours(1).into())
        .same_site(SameSite::Lax)
        .http_only(true);

    Ok(SuccessResponse {
        status: "success".into(),
        data: AuthResponse {
            token: cookie.to_string().parse().unwrap(),
        },
    })
}

#[server]
async fn logout() -> Result<SuccessResponse<AuthResponse>, ServerFnError> {
    let cookie = Cookie::build(("token", ""))
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true);

    Ok(SuccessResponse {
        status: "success".into(),
        data: AuthResponse {
            token: cookie.to_string().parse().unwrap(),
        },
    })
}

#[server]
pub async fn about_me(token: String) -> Result<SuccessResponse<UserResponse>, ServerFnError> {
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let user_collection = db.collection::<User>("users");

    let claims = jsonwebtoken::decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(
            env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set")
                .as_ref(),
        ),
        &Validation::default(),
    )
    .map_err(|_| ServerFnError::new("Invalid token"))?;

    let user_id = ObjectId::from_str(&claims.claims.sub)
        .map_err(|_| ServerFnError::new("Invalid user ID"))?;
    let user = user_collection
        .find_one(doc! { "_id": user_id })
        .await?
        .ok_or(ServerFnError::new("User not found"))?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: UserResponse { user },
    })
}

#[server]
pub async fn auth(token: String) -> Result<User, ServerFnError> {
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let user_collection = db.collection::<User>("users");

    let claims = jsonwebtoken::decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(
            env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set")
                .as_ref(),
        ),
        &Validation::default(),
    )
    .map_err(|_| ServerFnError::new("Invalid token"))?;

    let user_id = ObjectId::from_str(&claims.claims.sub)
        .map_err(|_| ServerFnError::new("Invalid user ID"))?;
    let user = user_collection
        .find_one(doc! { "_id": user_id })
        .await?
        .ok_or(ServerFnError::new("User not found"))?;

    Ok(user)
}

#[server]
pub async fn get_user_info(user_id: ObjectId) -> Result<SuccessResponse<User>, ServerFnError> {
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set"));
    let user_collection = db.collection::<User>("users");

    let filter = doc! { "_id": user_id };
    let user = user_collection
        .find_one(filter)
        .await
        .map_err(|_| ServerFnError::new("Error fetching user data"))?
        .ok_or(ServerFnError::new("User not found"))?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: user,
    })
}

#[server]
pub async fn dashboard_overview() -> Result<SuccessResponse<DashboardResponse>, ServerFnError> {
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let user_collection = db.collection::<User>("users");
    let book_collection = db.collection::<Book>("books");

    let users = user_collection.estimated_document_count().await?;
    let books = book_collection.estimated_document_count().await?;
    let paid_users = user_collection
        .count_documents(doc! { "role": { "$ne": "free" } })
        .await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: DashboardResponse {
            users,
            books,
            paid_users,
        },
    })
}

#[server]
pub async fn edit_profile(
    body: EditUserSchema,
) -> Result<SuccessResponse<UserResponse>, ServerFnError> {
    let client = get_client().await;
    let db = client.database(&env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let user_collection = db.collection::<User>("users");

    let claims = jsonwebtoken::decode::<TokenClaims>(
        &body.token,
        &DecodingKey::from_secret(
            env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set")
                .as_ref(),
        ),
        &Validation::default(),
    )
    .map_err(|_| ServerFnError::new("Invalid token"))?;
    let user_id = ObjectId::from_str(&claims.claims.sub)
        .map_err(|_| ServerFnError::new("Invalid user ID"))?;

    let mut user = user_collection
        .find_one(doc! { "_id": user_id })
        .await?
        .ok_or(ServerFnError::new("User not found"))?;

    if let Some(new_name) = body.name {
        if new_name.is_empty() {
            return Err(ServerFnError::new("Name cannot be empty"));
        }
        user.name = new_name;
    }

    if let Some(new_photo) = body.photo {
        user.photo = new_photo;
    }

    if let Some(new_email) = body.email {
        if !new_email.contains("@") || !new_email.contains(".") {
            return Err(ServerFnError::new("Invalid email format"));
        }

        // Check if email is already in use by another user
        if user_collection
            .find_one(doc! { "email": &new_email, "_id": { "$ne": &user_id } })
            .await?
            .is_some()
        {
            return Err(ServerFnError::new("Email already in use"));
        }
        user.email = new_email.to_lowercase();
    }

    if let Some(old_pass) = body.old_password {
        if old_pass.is_empty() {
            return Err(ServerFnError::new("Old password cannot be empty"));
        }

        let parsed_hash = PasswordHash::new(&user.password)
            .map_err(|_| ServerFnError::new("Password verification error"))?;
        if !Argon2::default()
            .verify_password(old_pass.as_bytes(), &parsed_hash)
            .is_ok()
        {
            return Err(ServerFnError::new("Old password is incorrect"));
        }

        if let Some(new_pass) = body.new_password {
            if new_pass.len() < 8 {
                return Err(ServerFnError::new(
                    "New password must be at least 8 characters",
                ));
            }

            if let Some(confirm_pass) = body.confirm_password {
                if new_pass != confirm_pass {
                    return Err(ServerFnError::new("Passwords do not match"));
                }

                let salt = SaltString::generate(&mut OsRng);
                let hashed_password = Argon2::default()
                    .hash_password(new_pass.as_bytes(), &salt)
                    .map(|hash| hash.to_string())
                    .map_err(|_| ServerFnError::new("Error while hashing new password"))?;

                user.password = hashed_password;
            } else {
                return Err(ServerFnError::new("Confirmation password is required"));
            }
        }
    } else if body.new_password.is_some() || body.confirm_password.is_some() {
        return Err(ServerFnError::new(
            "Old password is required to update password",
        ));
    }

    user.updated_at = Utc::now();

    user_collection
        .update_one(
            doc! { "_id": user_id },
            doc! { "$set": {
                "name": &user.name,
                "email": &user.email,
                "password": &user.password,
                "updated_at": &user.updated_at,
            }},
        )
        .await
        .map_err(|_| ServerFnError::new("Failed to update user data"))?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: UserResponse { user },
    })
}
