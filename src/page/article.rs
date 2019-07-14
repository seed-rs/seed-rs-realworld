use seed::prelude::*;
use super::ViewPage;
use crate::{session, article, GMsg, route, api, comment_id, author, logger};

// Model

enum Status<T> {
    Loading,
    LoadingSlowly,
    Loaded(T),
    Failed,
}

impl<T> Default for Status<T> {
    fn default() -> Self {
        Status::Loading
    }
}

enum CommentText {
    Editing(String),
    Sending(String)
}

impl Default for CommentText {
    fn default() -> Self {
        CommentText::Editing("".into())
    }
}

#[derive(Default)]
pub struct Model<'a> {
    session: session::Session,
    errors: Vec<String>,
    comments: Status<(CommentText, Vec<article::comment::Comment<'a>>)>,
    article: Status<article::Article>
}

impl<'a> Model<'a> {
    pub fn session(&self) -> &session::Session {
        &self.session
    }
}

impl<'a> From<Model<'a>> for session::Session {
    fn from(model: Model) -> session::Session {
        model.session
    }
}

pub fn init<'a>(session: session::Session, slug: article::slug::Slug, _: &mut impl Orders<Msg, GMsg>
) -> Model<'a> {
    Model {
        session,
        ..Model::default()
    }
}

// Global msg handler

pub fn g_msg_handler(g_msg: GMsg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match g_msg {
        GMsg::SessionChanged(session) => {
            model.session = session;
            route::go_to(route::Route::Home, orders);
        }
        _ => ()
    }
}

// Update

pub enum Msg {
    DeleteArticleClicked(api::Credentials, article::slug::Slug),
    DeleteCommentClicked(api::Credentials, article::slug::Slug, comment_id::CommentId),
    DismissErrorsClicked,
    FavoriteClicked(api::Credentials, article::slug::Slug, article::body::Body),
    UnfavoriteClicked(api::Credentials, article::slug::Slug, article::body::Body),
    FollowClicked(api::Credentials, author::UnfollowedAuthor<'static>),
    UnfollowClicked(api::Credentials, author::FollowedAuthor<'static>),
    PostCommentClicked(api::Credentials, article::slug::Slug),
    CommentTextEntered(String),
    LoadArticleCompleted(Result<article::Article, Vec<String>>),
    LoadCommentsCompleted(Result<Vec<article::comment::Comment<'static>>, Vec<String>>),
    DeleteArticleCompleted(Result<(), Vec<String>>),
    DeleteCommentCompleted(Result<comment_id::CommentId, Vec<String>>),
    FavoriteChangeCompleted(Result<article::Article, Vec<String>>),
    FollowChangeCompleted(Result<author::Author<'static>, Vec<String>>),
    PostCommentCompleted(Result<article::comment::Comment<'static>, Vec<String>>),
    SlowLoadThresholdPassed,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::DeleteArticleClicked(credentials, slug) => {
            unimplemented!()
        }
        Msg::DeleteCommentClicked(credentials, slug, comment_id) => {
            unimplemented!()
        }
        Msg::DismissErrorsClicked => {
            model.errors.clear();
        }
        Msg::FavoriteClicked(credentials, slug, article_body) => {
            unimplemented!()
        }
        Msg::UnfavoriteClicked(credentials, slug, article_body) => {
            unimplemented!()
        }
        Msg::FollowClicked(credentials, unfollowed_author) => {
            unimplemented!()
        }
        Msg::UnfollowClicked(credentials, followed_author) => {
            unimplemented!()
        }
        Msg::PostCommentClicked(credentials, slug) => {
            unimplemented!()
        }
        Msg::CommentTextEntered(comment_text) => {
            match &mut model.comments {
                Status::Loaded((CommentText::Editing(text), _)) => {
                    *text = comment_text;
                }
                _ => logger::error("Comment text can be changed only in Editing mode!")
            }
        }

        Msg::LoadArticleCompleted(Ok(article)) => {
            unimplemented!()
        }
        Msg::LoadArticleCompleted(Err(errors)) => {
            unimplemented!()
        }

        Msg::LoadCommentsCompleted(Ok(comments)) => {
            unimplemented!()
        }
        Msg::LoadCommentsCompleted(Err(errors)) => {
            unimplemented!()
        }

        Msg::DeleteArticleCompleted(Ok(())) => {
            unimplemented!()
        }
        Msg::DeleteArticleCompleted(Err(errors)) => {
            unimplemented!()
        }

        Msg::DeleteCommentCompleted(Ok(comment_id)) => {
            unimplemented!()
        }
        Msg::DeleteCommentCompleted(Err(errors)) => {
            unimplemented!()
        }

        Msg::FavoriteChangeCompleted(Ok(article)) => {
            unimplemented!()
        }
        Msg::FavoriteChangeCompleted(Err(errors)) => {
            unimplemented!()
        }

        Msg::FollowChangeCompleted(Ok(article)) => {
            unimplemented!()
        }
        Msg::FollowChangeCompleted(Err(errors)) => {
            unimplemented!()
        }

        Msg::PostCommentCompleted(Ok(comment)) => {
            unimplemented!()
        }
        Msg::PostCommentCompleted(Err(errors)) => {
            unimplemented!()
        }

        Msg::SlowLoadThresholdPassed => {
            unimplemented!()
        }
    }
}

// View

pub fn view<'a>(model: &Model) -> ViewPage<'a, Msg> {
    ViewPage::new("Conduit",view_content())
}

fn view_content() -> Node<Msg> {
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