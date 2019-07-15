use seed::prelude::*;
use super::ViewPage;
use crate::{session, article, GMsg, route, api, comment_id, author, logger, request, helper::take, markdown, loading};
use std::collections::VecDeque;

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
    comments: Status<(CommentText, VecDeque<article::comment::Comment<'a>>)>,
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

pub fn init<'a>(session: session::Session, slug: article::slug::Slug, orders: &mut impl Orders<Msg, GMsg>
) -> Model<'a> {
    orders
        .perform_cmd(loading::slow_threshold(Msg::SlowLoadThresholdPassed, Msg::Unreachable))
        .perform_cmd(request::article_article_load::load_article(&session, &slug,Msg::LoadArticleCompleted))
        .perform_cmd(request::comments_load::load_comments(session.clone(), &slug,Msg::LoadCommentsCompleted));

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
    DeleteArticleClicked(article::slug::Slug),
    DeleteCommentClicked(article::slug::Slug, comment_id::CommentId),
    DismissErrorsClicked,
    FavoriteClicked(article::slug::Slug),
    UnfavoriteClicked(article::slug::Slug),
    FollowClicked(author::Author<'static>),
    UnfollowClicked(author::Author<'static>),
    PostCommentClicked(article::slug::Slug),
    CommentTextEntered(String),
    LoadArticleCompleted(Result<article::Article, Vec<String>>),
    LoadCommentsCompleted(Result<VecDeque<article::comment::Comment<'static>>, Vec<String>>),
    DeleteArticleCompleted(Result<(), Vec<String>>),
    DeleteCommentCompleted(Result<comment_id::CommentId, Vec<String>>),
    FavoriteChangeCompleted(Result<article::Article, Vec<String>>),
    FollowChangeCompleted(Result<author::Author<'static>, Vec<String>>),
    PostCommentCompleted(Result<article::comment::Comment<'static>, Vec<String>>),
    SlowLoadThresholdPassed,
    Unreachable
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::DeleteArticleClicked(slug) => {
            orders
                .perform_cmd(request::article_delete::delete_article(
                    model.session(),
                    &slug,
                    Msg::DeleteArticleCompleted
                ))
                .skip();
        }
        Msg::DeleteCommentClicked(slug, comment_id) => {
            orders
                .perform_cmd(request::comment_delete::delete_comment(
                    model.session(),
                    &slug,
                    comment_id,
                    Msg::DeleteCommentCompleted
                ))
                .skip();
        }
        Msg::DismissErrorsClicked => {
            model.errors.clear();
        }
        Msg::FavoriteClicked(slug) => {
            // @TODO check if handlers with only orders has skip() called (especially feed.rs)
            orders
                .perform_cmd(request::favorite::favorite(
                    &model.session,
                    &slug,
                    Msg::FavoriteChangeCompleted
                ))
                .skip();
        }
        Msg::UnfavoriteClicked(slug) => {
            orders
                .perform_cmd(request::unfavorite::unfavorite(
                    &model.session,
                    &slug,
                    Msg::FavoriteChangeCompleted
                ))
                .skip();
        }
        Msg::FollowClicked(author) => {
            orders
                .perform_cmd(request::follow::follow(
                    model.session.clone(),
                    author.username().to_static(),
                    Msg::FollowChangeCompleted
                ))
                .skip();
        }
        Msg::UnfollowClicked(author) => {
            orders
                .perform_cmd(request::unfollow::unfollow(
                    model.session.clone(),
                    author.username().to_static(),
                    Msg::FollowChangeCompleted
                ))
                .skip();
        }
        Msg::PostCommentClicked(slug) => {
            let model_comments = &mut model.comments;
            match model_comments {
                Status::Loaded((CommentText::Editing(text), _)) if text.is_empty() => {
                    orders.skip();
                }
                Status::Loaded((CommentText::Editing(text), comments)) => {
                    orders
                        .perform_cmd(request::comment_create::create_comment(
                            &model.session.clone(),
                            &slug,
                            text.clone(),
                            Msg::PostCommentCompleted
                        ));
                    *model_comments = Status::Loaded((CommentText::Sending(take(text)), take(comments)));
                }
                _ => logger::error("Comment can be created only in Editing mode!")
            }
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
            model.article = Status::Loaded(article)
        }
        Msg::LoadArticleCompleted(Err(errors)) => {
            model.article = Status::Failed;
            logger::error("Load article failed");
        }

        Msg::LoadCommentsCompleted(Ok(comments)) => {
            model.comments = Status::Loaded((CommentText::Editing("".into()), comments));
        }
        Msg::LoadCommentsCompleted(Err(errors)) => {
            model.comments = Status::Failed;
            logger::error("Load comments failed");
        }

        Msg::DeleteArticleCompleted(Ok(())) => {
            route::go_to(route::Route::Home, orders);
        }
        Msg::DeleteArticleCompleted(Err(errors)) => {
            // @TODO errors (see Elm example)?
        }

        Msg::DeleteCommentCompleted(Ok(comment_id)) => {
            if let Status::Loaded((_, comments)) = &mut model.comments {
                comments.retain(|comment|comment.id != comment_id);
            }
        }
        Msg::DeleteCommentCompleted(Err(errors)) => {
            // @TODO errors (see Elm example)?
        }

        Msg::FavoriteChangeCompleted(Ok(article)) => {
            model.article = Status::Loaded(article);
        }
        Msg::FavoriteChangeCompleted(Err(errors)) => {
            // @TODO errors (see Elm example)?
        }

        Msg::FollowChangeCompleted(Ok(author)) => {
            if let Status::Loaded(article) = &mut model.article {
                article.author = author;
            }
        }
        Msg::FollowChangeCompleted(Err(errors)) => {
            // @TODO errors (see Elm example)?
        }

        Msg::PostCommentCompleted(Ok(comment)) => {
            if let Status::Loaded((text, comments)) = &mut model.comments {
                *text = CommentText::Editing("".into());
                comments.push_front(comment);
            }
        }
        Msg::PostCommentCompleted(Err(errors)) => {
            // @TODO return to editing mode?
            // @TODO errors (see Elm example)?
        }

        Msg::SlowLoadThresholdPassed => {
            if let Status::Loading = model.article {
                model.article = Status::LoadingSlowly
            }
            if let Status::Loading = model.comments {
                model.comments = Status::LoadingSlowly
            }
        }
        Msg::Unreachable => { logger::error("Unreachable!") },
    }
}

// View

pub fn view<'a>(model: &Model) -> ViewPage<'a, Msg> {
    ViewPage::new("Conduit",view_content(model))
}

fn view_banner() -> Node<Msg> {
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
    ]
}

fn view_article_content(markdown: &markdown::Markdown) -> Node<Msg> {
    div![
        class!["row", "article-content"],
        div![
            class!["col-md-12"],
            md!(markdown.as_str())
        ]
    ]
}

fn view_article_actions() -> Node<Msg> {
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
    ]
}

fn view_comment_form() -> Node<Msg> {
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
    ]
}

fn view_comment() -> Node<Msg> {
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
}

fn view_comments() -> Vec<Node<Msg>> {
    vec![view_comment()]
}

fn view_content(model: &Model) -> Node<Msg> {
    match &model.article {
        Status::Loading => empty![],
        Status::LoadingSlowly => loading::icon(),
        Status::Failed => loading::error("article"),
        Status::Loaded(article) => {
            div![
                class!["article-page"],
                view_banner(),

                div![
                    class!["container", "page"],
                    view_article_content(&article.body),
                    hr![],
                    view_article_actions(),

                    div![
                        class!["row"],
                        div![
                            class!["col-xs-12", "col-md-8", "offset-md-2"],
                            view_comment_form(),
                            view_comments(),

                        ]
                    ]

                ]

            ]
        },
    }
}