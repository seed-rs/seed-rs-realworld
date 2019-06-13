use seed::prelude::*;
use crate::viewer;

pub mod article;
pub mod article_editor;
pub mod blank;
pub mod home;
pub mod login;
pub mod not_found;
pub mod profile;
pub mod register;
pub mod settings;

pub struct ViewPage<Ms: 'static> {
    pub title: String,
    pub content: El<Ms>
}

pub enum Page {
    Other
}

impl Page {
    pub fn view<Ms>(&self, viewer: Option<viewer::Viewer>, view_page: ViewPage<Ms>) -> impl ElContainer<Ms> {
        // @TODO set title  ` { title = title ++ " - Conduit"`
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
                            El::from_html("&nbsp;"),
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
                            El::from_html("&nbsp;"),
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
