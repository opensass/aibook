#![allow(non_snake_case)]

use crate::components::navbar::HomeNavBar;
use crate::components::navbar::LoginNavBar;
use crate::pages::book::EditBook;
use crate::pages::book::ReadBook;
use crate::pages::dashboard::Dashboard;
use crate::pages::home::Home;
use crate::pages::login::Login;
use crate::pages::profile::UserProfile;
use crate::pages::signup::Register;
use crate::pages::success::SuccessPage;
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[layout(HomeNavBar)]
    #[route("/")]
    Home {},
    #[end_layout]
    #[layout(LoginNavBar)]
    #[route("/login")]
    Login {},
    #[route("/signup")]
    Register {},
    #[end_layout]
    #[route("/dashboard/book/read/:id")]
    ReadBook { id: String },
    #[route("/dashboard/book/edit/:id")]
    EditBook { id: String },
    #[route("/dashboard")]
    Dashboard {},
    #[route("/success")]
    SuccessPage {},
    #[route("/dashboard/profile/:id")]
    UserProfile { id: String },
}
