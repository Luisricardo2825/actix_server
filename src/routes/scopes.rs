use actix_web::web;
// use mark_route::list_methods;

use crate::{
    middlewares::CHECK_LOGIN,
    services::{
        cms::{
            core::{field::FieldRoute, table::TableRoute},
            custom::custom::CustomRoute,
        },
        posts::PostsRoute,
        users::{auth::auth::AuthService, core::users::UsersRoute},
    },
};

/**
Route for posts.

**Endpoint**: `/posts`
### Methods:
 ```
 * GET
 * POST
 * PATCH
 * DELETE
 ```
```
return actix_web::Scope
```
 */

pub struct Scopes;

impl Scopes {
    pub fn posts_scope() -> actix_web::Scope {
        actix_web::web::scope("posts")
            .route("/", web::post().to(PostsRoute::create).wrap(CHECK_LOGIN))
            .route("/{id}/", web::get().to(PostsRoute::find))
            .route("/", web::get().to(PostsRoute::find_all))
            .route(
                "/{id}/",
                web::patch().to(PostsRoute::update).wrap(CHECK_LOGIN),
            )
            .route("/", web::delete().to(PostsRoute::delete).wrap(CHECK_LOGIN))
    }

    pub fn users_scope() -> actix_web::Scope {
        actix_web::web::scope("users")
            .route("/", web::post().to(UsersRoute::create))
            .route("/{id}/", web::get().to(UsersRoute::find))
            .route("/", web::get().to(UsersRoute::find_all))
            .route("/{id}/", web::patch().to(UsersRoute::update))
            .route("/{id}/", web::delete().to(UsersRoute::delete))
    }

    pub fn tables_scope() -> actix_web::Scope {
        actix_web::web::scope("tables")
            .route("/", web::post().to(TableRoute::create))
            .route("/{id}/", web::get().to(TableRoute::find_table_by_name))
            .route("/", web::get().to(TableRoute::find_all))
            .route("/{id}/", web::patch().to(TableRoute::update))
            .route("/{id}/", web::delete().to(TableRoute::delete_table_by_name))
    }

    pub fn fields_scope() -> actix_web::Scope {
        actix_web::web::scope("/tables/{table_name}/fields")
            .route("/", web::post().to(FieldRoute::create))
            .route("/{id}/", web::get().to(FieldRoute::find))
            .route("/", web::get().to(FieldRoute::find_all))
            .route("/{id}/", web::patch().to(FieldRoute::update))
            .route(
                "/{field_name}/",
                web::delete().to(FieldRoute::delete_by_name),
            )
    }
    pub fn custom_scope() -> actix_web::Scope {
        actix_web::web::scope("/custom")
            .route("/{table_name}/", web::get().to(CustomRoute::find_all))
            .route("/{table_name}/{id}/", web::get().to(CustomRoute::find_one))
            // .route("/", web::get().to(CustomRoute::find_test))
            .route("/{table_name}/", web::post().to(CustomRoute::create))
    }

    pub fn login_scope() -> actix_web::Scope {
        actix_web::web::scope("login").route("/", web::post().to(AuthService::login))
    }
}
