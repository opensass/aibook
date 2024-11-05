#![allow(non_snake_case)]

use crate::components::navbar::NavBar;
use crate::pages::blog::Blog;
use crate::pages::home::Home;
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Home {},
    #[end_layout]
    #[route("/blog/:id")]
    Blog { id: i32 },
}
