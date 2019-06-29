use seed::prelude::*;
use crate::{viewer, username, route, GMsg};
use std::borrow::{Borrow, Cow};

pub mod article;
pub mod article_editor;
pub mod blank;
pub mod home;
pub mod login;
pub mod not_found;
pub mod profile;
pub mod register;
pub mod settings;

pub struct ViewPage<'a, Ms: 'static> {
    title_prefix: Cow<'a, str>,
    content: El<Ms>
}

impl<'a, Ms> ViewPage<'a, Ms> {
    pub fn new(title_prefix: impl Into<Cow<'a, str>>, content: El<Ms>) -> Self {
        Self {
            title_prefix: title_prefix.into(),
            content
        }
    }
    pub fn title(&self) -> String {
        format!("{} - Conduit", self.title_prefix)
    }
    pub fn into_content(self) -> El<Ms> {
        self.content
    }
}

pub fn view<'a, Ms>(page: Page<'a>, view_page: ViewPage<'a, Ms>, viewer: Option<&viewer::Viewer>) -> Vec<El<Ms>> {
    seed::document().set_title(&view_page.title());

    vec![
        page.view_header(viewer),
        view_page.into_content(),
        page.view_footer(),
    ]
}

pub enum Page<'a> {
    Other,
    Home,
    Login,
    Register,
    Settings,
    Profile(&'a username::Username<'a>),
    NewArticle
}

impl<'a> Page<'a> {
    fn is_active(&self, route: &route::Route) -> bool {
        match (self, route) {
            (Page::Home, route::Route::Home) => true,
            (Page::Login, route::Route::Login) => true,
            (Page::Register, route::Route::Register) => true,
            (Page::Settings, route::Route::Settings) => true,
            (Page::Profile(username), route::Route::Profile(route_username)) => {
                *username == route_username.borrow()
            },
            (Page::NewArticle, route::Route::NewArticle) => true,
            _ => false,
        }
    }

    fn view_navbar_link<Ms>(&self, route: &route::Route, link_content: impl UpdateEl<El<Ms>>) -> El<Ms> {
        li![
            class!["nav-item"],
            a![
                class![
                    "nav-link",
                    "active" => self.is_active(route),
                ],
                attrs!{At::Href => route.to_string()},
                link_content
            ]
        ]
    }

    fn view_menu<Ms>(&self, viewer: Option<&viewer::Viewer>) -> Vec<El<Ms>> {
        match viewer {
            None => {
                vec![
                    self.view_navbar_link(&route::Route::Login, "Sign in"),
                    self.view_navbar_link(&route::Route::Register, "Sign up"),
                ]
            },
            Some(viewer) => {
                vec![
                    self.view_navbar_link(
                        &route::Route::NewArticle,
                        vec![
                            i![
                                class!["ion-compose"]
                            ],
                            plain!("\u{00A0}New Post")
                        ]
                    ),
                    self.view_navbar_link(
                        &route::Route::Settings,
                        vec![
                            i![
                                class!["ion-gear-a"]
                            ],
                            plain!("\u{00A0}Settings")
                        ]
                    ),
                    self.view_navbar_link(
                        &route::Route::Profile(std::borrow::Cow::Borrowed(viewer.username())),
                        vec![
                            img![
                                class!["user-pic"],
                                attrs!{At::Src => viewer.avatar().src()}
                            ],
                            plain!(viewer.username().as_str())
                        ]
                    ),
                    self.view_navbar_link(&route::Route::Logout, "Sign out"),
                ]
            }
        }
    }

    fn view_header<Ms>(&self, viewer: Option<&viewer::Viewer>) -> El<Ms> {
        nav![
            class!["navbar", "navbar-light"],
            div![
                class!["container"],
                a![
                    class!["navbar-brand"],
                    attrs!{At::Href => route::Route::Home.to_string()},
                    "conduit"
                ],
                ul![
                    class!["nav navbar-nav pull-xs-right"],
                    self.view_navbar_link(&route::Route::Home, "Home"),
                    self.view_menu(viewer),
                ],
            ]
        ]
    }

    fn view_footer<Ms>(&self) -> El<Ms> {
        footer![
            div![
                class!["container"],
                a![
                    class!["logo-font"],
                    attrs!{At::Href => route::Route::Home.to_string()},
                    "conduit"
                ],
                span![
                    class!["attribution"],
                    "An interactive learning project from ",
                    a![
                        attrs!{At::Href => "https://thinkster.io"},
                        "Thinkster"
                    ],
                    ". Code & design licensed under MIT."
                ]
            ]
        ]
    }
}
