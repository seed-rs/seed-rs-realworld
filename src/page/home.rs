use seed::prelude::*;
use super::{ViewPage, InitPage};
use crate::{session, GMsg, route, HasSessionChangedOnInit};

// Model

pub struct Model {
    session: session::Session
}

impl Model {
    pub fn session(&self) -> &session::Session {
        &self.session
    }
}

impl From<Model> for session::Session {
    fn from(model: Model) -> session::Session{
        model.session
    }
}

pub fn init(session: session::Session) -> InitPage<Model, Msg> {
    InitPage::new(Model { session })
}

// Global msg handler

pub fn g_msg_handler<PMsg>(g_msg: GMsg, model: &mut Model, _: &mut OrdersProxy<Msg, PMsg, GMsg>) {
    match g_msg {
        GMsg::SessionChanged(session, on_init) => {
            model.session = session;
        }
        _ => ()
    }
}

// Update

pub enum Msg {
}

pub fn update<PMsg>(msg: Msg, model: &mut Model, orders: &mut OrdersProxy<Msg, PMsg, GMsg>) {
}

// View

pub fn view<'a>(model: &Model) -> ViewPage<'a, Msg> {
    ViewPage::new("Conduit", view_content())
}

fn view_content() -> El<Msg> {
    div![
        class!["home-page"],

        div![
            class!["banner"],
            div![
                class!["container"],
                h1![
                    class!["logo-font"],
                    "conduit"
                ],
                p![
                    "A place to share your knowledge."
                ]
            ]
        ],

        div![
            class!["container", "page"],
            div![
                class!["row"],

                div![
                    class!["col-md-9"],
                    div![
                        class!["feed-toggle"],
                        ul![
                            class!["nav", "nav-pills", "outline-active"],
                            li![
                                class!["nav-item"],
                                a![
                                    class!["nav-link", "disabled"],
                                    attrs!{At::Href => ""},
                                    "Your Feed"
                                ]
                            ],
                            li![
                                class!["nav-item"],
                                a![
                                    class!["nav-link", "active"],
                                    attrs!{At::Href => ""},
                                    "Global Feed"
                                ]
                            ],
                        ],
                    ],

                    div![
                        class!["article-preview"],
                        div![
                            class!["article-meta"],
                            a![
                                attrs!{At::Href => "/profile"},
                                img![
                                    attrs!{At::Src => "http://i.imgur.com/Qr71crq.jpg"}
                                ]
                            ],
                            div![
                                class!["info"],
                                a![
                                    class!["author"],
                                    attrs!{At::Href => ""},
                                    "Eric Simons"
                                ],
                                span![
                                    class!["date"],
                                    "January 20th"
                                ]
                            ],
                            button![
                                class!["btn","btn-outline-primary", "btn-sm", "pull-xs-right"],
                                i![
                                    class!["ion-heart"],
                                    " 29"
                                ]
                            ]
                        ],
                        a![
                            class!["preview-link"],
                            attrs!{At::Href => ""},
                            h1![
                                "How to build webapps that scale"
                            ],
                            p![
                                "This is the description for the post."
                            ],
                            span![
                                "Read more..."
                            ]
                        ]
                    ],

                    div![
                        class!["article-preview"],
                        div![
                            class!["article-meta"],
                            a![
                                attrs!{At::Href => "/profile"},
                                img![
                                    attrs!{At::Src => "http://i.imgur.com/N4VcUeJ.jpg"}
                                ]
                            ],
                            div![
                                class!["info"],
                                a![
                                    class!["author"],
                                    attrs!{At::Href => ""},
                                    "Albert Pai"
                                ],
                                span![
                                    class!["date"],
                                    "January 20th"
                                ]
                            ],
                            button![
                                class!["btn","btn-outline-primary", "btn-sm", "pull-xs-right"],
                                i![
                                    class!["ion-heart"],
                                    " 32"
                                ]
                            ]
                        ],
                        a![
                            class!["preview-link"],
                            attrs!{At::Href => ""},
                            h1![
                                "The song you won't ever stop singing. No matter how hard you try."
                            ],
                            p![
                                "This is the description for the post."
                            ],
                            span![
                                "Read more..."
                            ]
                        ]
                    ]

                ],

                div![
                    class!["col-md-3"],
                    div![
                        class!["sidebar"],
                        p![
                            "Popular Tags"
                        ],

                        div![
                            class!["tag-list"],
                            a![
                                class!["tag-pill", "tag-default"],
                                attrs!{At::Href => ""},
                                "programming"
                            ],
                            a![
                                class!["tag-pill", "tag-default"],
                                attrs!{At::Href => ""},
                                "javascript"
                            ],
                            a![
                                class!["tag-pill", "tag-default"],
                                attrs!{At::Href => ""},
                                "emberjs"
                            ],
                            a![
                                class!["tag-pill", "tag-default"],
                                attrs!{At::Href => ""},
                                "angularjs"
                            ],
                            a![
                                class!["tag-pill", "tag-default"],
                                attrs!{At::Href => ""},
                                "react"
                            ],
                            a![
                                class!["tag-pill", "tag-default"],
                                attrs!{At::Href => ""},
                                "mean"
                            ],
                            a![
                                class!["tag-pill", "tag-default"],
                                attrs!{At::Href => ""},
                                "node"
                            ],
                            a![
                                class!["tag-pill", "tag-default"],
                                attrs!{At::Href => ""},
                                "rails"
                            ]
                        ]
                    ]
                ]

            ]
        ]

    ]
}