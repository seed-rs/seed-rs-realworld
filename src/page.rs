use crate::entity::{Username, Viewer};
use crate::Route;
use seed::prelude::*;
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

#[allow(clippy::module_name_repetitions)]
pub struct ViewPage<'a, Ms: 'static> {
    title_prefix: Cow<'a, str>,
    content: Node<Ms>,
}

impl<'a, Ms> ViewPage<'a, Ms> {
    pub fn new(title_prefix: impl Into<Cow<'a, str>>, content: Node<Ms>) -> Self {
        Self {
            title_prefix: title_prefix.into(),
            content,
        }
    }
    pub fn title(&self) -> String {
        format!("{} - Conduit", self.title_prefix)
    }
    pub fn into_content(self) -> Node<Ms> {
        self.content
    }
}

pub fn view<'a, Ms>(
    page: Page<'a>,
    view_page: ViewPage<'a, Ms>,
    viewer: Option<&Viewer>,
) -> Vec<Node<Ms>> {
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
    Profile(&'a Username<'a>),
    NewArticle,
}

impl<'a> Page<'a> {
    fn is_active(&self, route: &Route) -> bool {
        match (self, route) {
            (Page::Home, Route::Home) => true,
            (Page::Login, Route::Login) => true,
            (Page::Register, Route::Register) => true,
            (Page::Settings, Route::Settings) => true,
            (Page::Profile(username), Route::Profile(route_username)) => {
                *username == route_username.borrow()
            }
            (Page::NewArticle, Route::NewArticle) => true,
            _ => false,
        }
    }

    fn view_navbar_link<Ms>(&self, route: &Route, link_content: impl UpdateEl<El<Ms>>) -> Node<Ms> {
        li![
            class!["nav-item"],
            a![
                class![
                    "nav-link",
                    "active" => self.is_active(route),
                ],
                attrs! {At::Href => route.to_string()},
                link_content
            ]
        ]
    }

    fn view_menu<Ms>(&self, viewer: Option<&Viewer>) -> Vec<Node<Ms>> {
        match viewer {
            None => vec![
                self.view_navbar_link(&Route::Login, "Sign in"),
                self.view_navbar_link(&Route::Register, "Sign up"),
            ],
            Some(viewer) => vec![
                self.view_navbar_link(
                    &Route::NewArticle,
                    vec![i![class!["ion-compose"]], plain!("\u{00A0}New Post")],
                ),
                self.view_navbar_link(
                    &Route::Settings,
                    vec![i![class!["ion-gear-a"]], plain!("\u{00A0}Settings")],
                ),
                self.view_navbar_link(
                    &Route::Profile(Cow::Borrowed(viewer.username())),
                    vec![
                        img![
                            class!["user-pic"],
                            attrs! {At::Src => viewer.avatar().src()}
                        ],
                        plain!(viewer.username().to_string()),
                    ],
                ),
                self.view_navbar_link(&Route::Logout, "Sign out"),
            ],
        }
    }

    fn view_header<Ms>(&self, viewer: Option<&Viewer>) -> Node<Ms> {
        nav![
            class!["navbar", "navbar-light"],
            div![
                class!["container"],
                a![
                    class!["navbar-brand"],
                    attrs! {At::Href => Route::Home.to_string()},
                    "conduit"
                ],
                ul![
                    class!["nav navbar-nav pull-xs-right"],
                    self.view_navbar_link(&Route::Home, "Home"),
                    self.view_menu(viewer),
                ],
            ]
        ]
    }

    fn view_footer<Ms>(&self) -> Node<Ms> {
        footer![div![
            class!["container"],
            a![
                class!["logo-font"],
                attrs! {At::Href => Route::Home.to_string()},
                "conduit"
            ],
            span![
                class!["attribution"],
                "An interactive learning project from ",
                a![attrs! {At::Href => "https://thinkster.io"}, "Thinkster"],
                ". Code & design licensed under MIT."
            ]
        ]]
    }
}

pub fn view_errors<Ms: Clone>(dismiss_errors: Ms, errors: Vec<String>) -> Node<Ms> {
    if errors.is_empty() {
        empty![]
    } else {
        div![
            class!["error-messages"],
            style! {
                "position" => "fixed",
                "top" => 0,
                "background" => "rgb(250, 250, 250)",
                "padding" => "20px",
                "border" => "1px solid",
                "z-index" => 9999,
            },
            errors.into_iter().map(|error| p![error]),
            button![simple_ev(Ev::Click, dismiss_errors), "Ok"]
        ]
    }
}

pub fn scroll_to_top() {
    seed::window().scroll_to_with_scroll_to_options(
        web_sys::ScrollToOptions::new()
            .top(0.)
            .left(0.)
            .behavior(web_sys::ScrollBehavior::Smooth),
    )
}
