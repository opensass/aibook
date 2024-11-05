#![allow(non_snake_case)]

use crate::components::navbar::HomeNavBar;
use crate::components::navbar::LoginNavBar;
use crate::pages::blog::Blog;
use crate::pages::home::Home;
use crate::pages::login::Login;
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[layout(HomeNavBar)]
    #[route("/")]
    Home {},
    #[end_layout]
    // TODO: file an issue cz of ordering layout and router macros
    #[layout(LoginNavBar)]
    #[route("/login")]
    Login {},
    #[end_layout]
    #[route("/blog/:id")]
    Blog { id: i32 },
}
