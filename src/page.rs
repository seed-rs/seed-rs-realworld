use seed::prelude::*;
use crate::{viewer, username};

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
}

pub struct InitPage<Md, Ms: 'static> {
    model: Md,
    orders: Orders<Ms>
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

    pub fn into_tuple(self) -> (Md, Orders<Ms>) {
        (self.model, self.orders)
    }
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
    pub fn view<Ms>(&self, viewer: Option<&viewer::Viewer>, view_page: ViewPage<Ms>) -> Vec<El<Ms>> {
        seed::document().set_title(&view_page.title());

        vec![
            self.view_header(),
            view_page.content,
            self.view_footer()
        ]
    }

    fn view_header<Ms>(&self) -> El<Ms> {
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
                    li![
                        class!["nav-item"],
                        a![
                            // add "active" class when you're on that page"
                            class!["nav-link", "active"],
                            attrs!{At::Href => ""},
                            "Home"
                        ],
                    ],
                    li![
                        class!["nav-item"],
                        a![
                            class!["nav-link"],
                            attrs!{At::Href => ""},
                            i![
                                class!["ion-compose"]
                            ],
                            raw!("&nbsp;"),
                            "New Post"
                        ],
                    ],
                    li![
                        class!["nav-item"],
                        a![
                            class!["nav-link"],
                            attrs!{At::Href => ""},
                            i![
                                class!["ion-gear-a"]
                            ],
                            raw!("&nbsp;"),
                            "Settings"
                        ],
                    ]
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
