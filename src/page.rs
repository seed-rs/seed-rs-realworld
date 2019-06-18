use seed::prelude::*;
use crate::{viewer, username, route};

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
    title_prefix: &'a str,
    content: El<Ms>
}

impl<'a, Ms> ViewPage<'a, Ms> {
    pub fn new(title_prefix: &'a str, content: El<Ms>) -> Self {
        Self {
            title_prefix,
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

pub struct InitPage<Md, Ms: 'static> {
    pub model: Md,
    pub orders: Orders<Ms>
}

impl<Md, Ms> InitPage<Md, Ms> {
    pub fn new(model: Md) -> Self {
        Self {
            model,
            orders: Orders::default()
        }
    }

    pub fn orders_mut(&mut self) -> &mut Orders<Ms> {
        &mut self.orders
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
                *username == route_username
            },
            (Page::NewArticle, route::Route::NewArticle) => true,
            _ => false,
        }
    }

    fn view_navbar_link<Ms>(&self, route: &route::Route, mapper: impl Fn(&mut El<Ms>)) -> El<Ms> {
        let mut link = a![
            class![
                "nav-link",
                if self.is_active(route) { "active" } else { "" },
            ],
            attrs!{At::Href => route.to_string()},
        ];
        mapper(&mut link);

        li![
            class!["nav-item"],
            link,
        ]
    }

    fn view_menu<Ms>(&self, viewer: Option<&viewer::Viewer>) -> Vec<El<Ms>> {
        match viewer {
            None => {
                vec![
                    self.view_navbar_link(
                        &route::Route::Login,
                        |link| link.set_text("Sign in")
                    ),
                    self.view_navbar_link(
                        &route::Route::Register,
                        |link| link.set_text("Sign up")
                    ),
                ]
            },
            Some(viewer) => {
                vec![
                    self.view_navbar_link(
                        &route::Route::NewArticle,
                        |link| {
                            link.add_child(i![
                                class!["ion-compose"]
                            ]);
                            link.set_text("\u{00A0}New Post")
                        }
                    ),
                    self.view_navbar_link(
                        &route::Route::Settings,
                        |link| {
                            link.add_child(i![
                                class!["ion-gear-a"]
                            ]);
                            link.set_text("\u{00A0}Settings")
                        }
                    ),
                    self.view_navbar_link(
                        &route::Route::Profile(*viewer.username()),
                        |link| {
                            link.add_child(img![
                                class!["user-pic"],
                                attrs!{At::Src => viewer.avatar().src()}
                            ]);
                            link.set_text(viewer.username().as_str())
                        }
                    ),
                    self.view_navbar_link(
                        &route::Route::Logout,
                        |link | link.set_text("Sign out")
                    ),
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
                    attrs!{At::Href => "/"},
                    "conduit"
                ],
                ul![
                    class!["nav navbar-nav pull-xs-right"],
                    self.view_navbar_link(&route::Route::Home, |link| link.set_text("Home")),
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
                    attrs!{At::Href => "/"},
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
