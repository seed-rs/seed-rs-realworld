use seed::prelude::*;
use super::ViewPage;
use crate::{session, article, GMsg, route, HasSessionChangedOnInit};

// Model

pub struct Model {
    session: session::Session
}

impl<'a> Model {
    pub fn session(&self) -> &session::Session {
        &self.session
    }
}

impl<'a> From<Model> for session::Session {
    fn from(model: Model) -> session::Session {
        model.session
    }
}

pub fn init<'a>(
    session: session::Session,
    slug: article::slug::Slug,
    _: &mut impl OrdersTrait<Msg, GMsg>
) -> Model {
    Model { session }
}

// Global msg handler

pub fn g_msg_handler(g_msg: GMsg, model: &mut Model, orders: &mut impl OrdersTrait<Msg, GMsg>) {
    match g_msg {
        GMsg::SessionChanged(session, on_init) => {
            model.session = session;
            if !on_init {
                route::go_to(route::Route::Home, orders);
            }
        }
        _ => ()
    }
}

// Update

pub enum Msg {
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl OrdersTrait<Msg, GMsg>) {
}

// View

pub fn view<'a>(model: &Model) -> ViewPage<'a, Msg> {
    ViewPage::new("Conduit",view_content())
}

fn view_content() -> El<Msg> {
    div![
        class!["article-page"],

        div![
            class!["banner"],
            div![
                class!["container"],

                h1![
                    "How to build webapps that scale"
                ],

                div![
                    class!["article-meta"],
                    a![
                        attrs!{At::Href => ""},
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
                        class!["btn", "btn-sm", "btn-outline-secondary"],
                        i![
                            class!["ion-plus-round"]
                        ],
                        raw!("&nbsp;"),
                        "Follow Eric Simons ",
                        span![
                            class!["counter"],
                            "(10)"
                        ]
                    ],
                    raw!("&nbsp;&nbsp;"),
                    button![
                        class!["btn", "btn-sm", "btn-outline-primary"],
                        i![
                            class!["ion-heart"]
                        ],
                        raw!("&nbsp;"),
                        "Favorite Post ",
                        span![
                            class!["counter"],
                            "(29)"
                        ]
                    ],
                ]

            ]
        ],

        div![
            class!["container", "page"],

            div![
                class!["row", "article-content"],
                div![
                    class!["col-md-12"],
                    p![
                        "Web development technologies have evolved at an incredible clip over the past few years."
                    ],
                    h2![
                        id!("introducing-ionic"),
                        "Introducing RealWorld."
                    ],
                    p![
                        "It's a great solution for learning how other frameworks work."
                    ]
                ]
            ],

            hr![],

            div![
                class!["article-actions"],
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
                        class!["btn", "btn-sm", "btn-outline-secondary"],
                        i![
                            class!["ion-plus-round"]
                        ],
                        raw!("&nbsp;"),
                        "Follow Eric Simons ",
                        span![
                            class!["counter"],
                            "(10)"
                        ]
                    ],
                    raw!("&nbsp;&nbsp;"),
                    button![
                        class!["btn", "btn-sm", "btn-outline-primary"],
                        i![
                            class!["ion-heart"]
                        ],
                        raw!("&nbsp;"),
                        "Favorite Post ",
                        span![
                            class!["counter"],
                            "(29)"
                        ]
                    ],
                ]
            ],

            div![
                class!["row"],

                div![
                    class!["col-xs-12", "col-md-8", "offset-md-2"],

                    form![
                        class!["card", "comment-form"],
                        div![
                            class!["card-block"],
                            textarea![
                                class!["form-control"],
                                attrs!{At::Rows => 3; At::Placeholder => "Write a comment..."}
                            ]
                        ],
                        div![
                            class!["card-footer"],
                            img![
                                class!["comment-author-img"],
                                attrs!{At::Src => "http://i.imgur.com/Qr71crq.jpg"}
                            ],
                            button![
                                class!["btn", "btn-sm", "btn-primary"],
                                "Post Comment"
                            ]
                        ]
                    ],

                    div![
                        class!["card"],
                        div![
                            class!["card-block"],
                            p![
                                class!["card-text"],
                                "With supporting text below as a natural lead-in to additional content."
                            ]
                        ],
                        div![
                            class!["card-footer"],
                            a![
                                class!["comment-author"],
                                attrs!{At::Href => ""},
                                img![
                                    class!["comment-author-img"],
                                    attrs!{At::Src => "http://i.imgur.com/Qr71crq.jpg"}
                                ]
                            ],
                            raw!("&nbsp;"),
                            a![
                                class!["comment-author"],
                                attrs!{At::Href => ""},
                                "Jacob Schmidt"
                            ],
                            span![
                                class!["date-posted"],
                                "Dec 29th"
                            ]
                        ]
                    ],

                    div![
                        class!["card"],
                        div![
                            class!["card-block"],
                            p![
                                class!["card-text"],
                                "With supporting text below as a natural lead-in to additional content."
                            ]
                        ],
                        div![
                            class!["card-footer"],
                            a![
                                class!["comment-author"],
                                attrs!{At::Href => ""},
                                img![
                                    class!["comment-author-img"],
                                    attrs!{At::Src => "http://i.imgur.com/Qr71crq.jpg"}
                                ]
                            ],
                            raw!("&nbsp;"),
                            a![
                                class!["comment-author"],
                                attrs!{At::Href => ""},
                                "Jacob Schmidt"
                            ],
                            span![
                                class!["date-posted"],
                                "Dec 29th"
                            ],
                            span![
                                class!["mod-options"],
                                i![
                                    class!["ion-edit"]
                                ],
                                i![
                                    class!["ion-trash-a"]
                                ]
                            ]
                        ]
                    ]

                ]

            ]

        ]
    ]
}