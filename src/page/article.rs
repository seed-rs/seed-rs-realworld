use seed::prelude::*;
use super::ViewPage;
use crate::{session, article, GMsg, route, comment_id, author, logger, request, helper::take, loading, timestamp, page};
use std::collections::VecDeque;
use std::borrow::Cow;

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

pub fn init<'a>(session: session::Session, slug: &article::slug::Slug, orders: &mut impl Orders<Msg, GMsg>
) -> Model<'a> {
    orders
        .perform_cmd(loading::slow_threshold(Msg::SlowLoadThresholdPassed, Msg::Unreachable))
        .perform_cmd(request::article::load(
            session.credentials().cloned(),
            slug,
            Msg::LoadArticleCompleted))
        .perform_cmd(request::comment::load_list(
            session.credentials().cloned(),
            slug,
            Msg::LoadCommentsCompleted));

    Model {
        session,
        ..Model::default()
    }
}

// Sink

pub fn sink(g_msg: GMsg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match g_msg {
        GMsg::SessionChanged(session) => {
            model.session = session;
            route::go_to(route::Route::Home, orders);
        }
        _ => ()
    }
}

// Update

#[derive(Clone)]
pub enum Msg {
    DeleteArticleClicked(article::slug::Slug),
    DeleteCommentClicked(article::slug::Slug, comment_id::CommentId),
    DismissErrorsClicked,
    FavoriteClicked(article::slug::Slug),
    UnfavoriteClicked(article::slug::Slug),
    FollowClicked(author::UnfollowedAuthor<'static>),
    UnfollowClicked(author::FollowedAuthor<'static>),
    PostCommentClicked,
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
                .perform_cmd(request::article::delete(
                    model.session().credentials(),
                    &slug,
                    Msg::DeleteArticleCompleted
                ))
                .skip();
        }
        Msg::DeleteCommentClicked(slug, comment_id) => {
            orders
                .perform_cmd(request::comment::delete(
                    model.session().credentials(),
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
                .perform_cmd(request::favorite::unfavorite(
                    model.session().credentials().cloned(),
                    &slug,
                    Msg::FavoriteChangeCompleted
                ))
                .skip();
        }
        Msg::UnfavoriteClicked(slug) => {
            orders
                .perform_cmd(request::favorite::favorite(
                    model.session.credentials().cloned(),
                    &slug,
                    Msg::FavoriteChangeCompleted
                ))
                .skip();
        }
        Msg::FollowClicked(unfollowed_author) => {
            orders
                .perform_cmd(request::follow::follow(
                    model.session().credentials().cloned(),
                    &unfollowed_author.0,  // @TODO refactor
                    Msg::FollowChangeCompleted
                ))
                .skip();
        }
        Msg::UnfollowClicked(followed_author) => {
            orders
                .perform_cmd(request::follow::unfollow(
                    model.session().credentials().cloned(),
                    &followed_author.0,  // @TODO refactor
                    Msg::FollowChangeCompleted
                ))
                .skip();
        }
        Msg::PostCommentClicked => {
            // @TODO unnecessary article? (pass slug through message?)
            if let Status::Loaded(article) = &model.article {
                let model_comments = &mut model.comments;
                match model_comments {
                    Status::Loaded((CommentText::Editing(text), _)) if text.is_empty() => {
                        orders.skip();
                    }
                    Status::Loaded((CommentText::Editing(text), comments)) => {
                        orders
                            .perform_cmd(request::comment::create(
                                model.session.credentials().cloned(),
                                &article.slug,
                                text.clone(),
                                Msg::PostCommentCompleted
                            ));
                        *model_comments = Status::Loaded((CommentText::Sending(take(text)), take(comments)));
                    }
                    _ => logger::error("Comment can be created only in Editing mode!")
                }
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
            logger::errors(errors);
        }

        Msg::LoadCommentsCompleted(Ok(comments)) => {
            model.comments = Status::Loaded((CommentText::Editing("".into()), comments));
        }
        Msg::LoadCommentsCompleted(Err(errors)) => {
            model.comments = Status::Failed;
            logger::errors(errors);
        }

        Msg::DeleteArticleCompleted(Ok(())) => {
            route::go_to(route::Route::Home, orders);
        }
        Msg::DeleteArticleCompleted(Err(errors)) => {
            logger::errors(errors.clone());
            model.errors = errors
        }

        Msg::DeleteCommentCompleted(Ok(comment_id)) => {
            if let Status::Loaded((_, comments)) = &mut model.comments {
                comments.retain(|comment|comment.id != comment_id);
            }
        }
        Msg::DeleteCommentCompleted(Err(errors)) => {
            logger::errors(errors.clone());
            model.errors = errors
        }

        Msg::FavoriteChangeCompleted(Ok(article)) => {
            model.article = Status::Loaded(article);
        }
        Msg::FavoriteChangeCompleted(Err(errors)) => {
            logger::errors(errors.clone());
            model.errors = errors
        }

        Msg::FollowChangeCompleted(Ok(author)) => {
            if let Status::Loaded(article) = &mut model.article {
                article.author = author;
            }
        }
        Msg::FollowChangeCompleted(Err(errors)) => {
            logger::errors(errors.clone());
            model.errors = errors
        }

        Msg::PostCommentCompleted(Ok(comment)) => {
            if let Status::Loaded((comment_text, comments)) = &mut model.comments {
                *comment_text = CommentText::Editing("".into());
                comments.push_front(comment);
            }
        }
        Msg::PostCommentCompleted(Err(errors)) => {
            if let Status::Loaded((comment_text, _)) = &mut model.comments {
                if let CommentText::Sending(text) = comment_text {
                    *comment_text = CommentText::Editing(take(text))
                }
            }
            logger::errors(errors.clone());
            model.errors = errors
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

fn view_favorite_button(article: &article::Article) -> Node<Msg> {
    if article.favorited {
        button![
            class!["btn","btn-primary", "btn-sm"],
            simple_ev(Ev::Click, Msg::FavoriteClicked(article.slug.clone())),
            i![
                class!["ion-heart"],
                format!(" Favorite Article ({})", article.favorites_count),
            ]
        ]
    } else {
        button![
            class!["btn","btn-outline-primary", "btn-sm"],
            simple_ev(Ev::Click, Msg::UnfavoriteClicked(article.slug.clone())),
            i![
                class!["ion-heart"],
                format!(" Favorite Article ({})", article.favorites_count),
            ]
        ]
    }
}

fn view_delete_button(slug: article::slug::Slug) -> Node<Msg> {
    button![
        class!["btn", "btn-outline-danger", "btn-sm"],
        simple_ev(Ev::Click, Msg::DeleteArticleClicked(slug)),
        i![
            class!["ion-trash-a"]
        ],
        " Delete Article",
    ]
}

fn view_edit_button(slug: article::slug::Slug) -> Node<Msg> {
    a![
        class!["btn", "btn-outline-secondary", "btn-sm"],
        attrs!{At::Href => route::Route::EditArticle(slug).to_string()},
        i![
            class!["ion-edit"],
        ],
        " Edit Article",
    ]
}

fn view_buttons(article: &article::Article, model: &Model) -> Vec<Node<Msg>> {
    let credentials = model.session().viewer().map(|viewer|&viewer.credentials);
    match credentials {
        None => vec![],
        Some(_) => {
            match &article.author {
                author::Author::IsViewer(..) => {
                    vec![
                        view_edit_button(article.slug.clone()),
                        plain![" "],
                        view_delete_button(article.slug.clone())
                    ]
                }
                author::Author::Following(followed_author) => {
                    vec![
                        author::view_unfollow_button(
                            Msg::UnfollowClicked(followed_author.clone()),
                            &followed_author.0
                        ),
                        plain![" "],
                        view_favorite_button(article),
                    ]
                }
                author::Author::NotFollowing(unfollowed_author) => {
                    vec![
                        author::view_follow_button(
                            Msg::FollowClicked(unfollowed_author.clone()),
                            &unfollowed_author.0
                        ),
                        plain![" "],
                        view_favorite_button(article),
                    ]
                }
            }
        }
    }
}

fn view_article_meta(article: &article::Article, model: &Model) -> Node<Msg> {
    div![
        class!["article-meta"],
        a![
            attrs!{At::Href => route::Route::Profile(Cow::Borrowed(article.author.username())).to_string()},
            img![
                attrs!{At::Src => article.author.profile().avatar.src()}
            ]
        ],
        div![
            class!["info"],
            author::view(article.author.username()),
            span![
                class!["date"],
                timestamp::view(&article.created_at)
            ]
        ],
        view_buttons(article, model),
    ]
}

fn view_banner(article: &article::Article, model: &Model) -> Node<Msg> {
    div![
        class!["banner"],
        div![
            class!["container"],
            h1![
                article.title
            ],
            view_article_meta(article, model),
            page::view_errors(Msg::DismissErrorsClicked, model.errors.clone()),
        ]
    ]
}

fn view_comment_form(comment_text: &CommentText, model: &Model) -> Node<Msg> {
    match model.session().viewer() {
        None => {
            p![
                a![
                    "Sign in",
                    attrs!{At::Href => route::Route::Login.to_string()}
                ],
                " or ",
                a![
                    "Sign up",
                    attrs!{At::Href => route::Route::Register.to_string()}
                ],
                " to comment."
            ]
        }
        Some(viewer) => {
            let (comment_text, post_comment_disabled) = match comment_text {
                CommentText::Editing(text) => {
                    (text, false)
                }
                CommentText::Sending(text) => {
                    (text, true)
                }
            };

            form![
                class!["card", "comment-form"],
                raw_ev(Ev::Submit, |event| {
                    event.prevent_default();
                    Msg::PostCommentClicked
                }),
                div![
                    class!["card-block"],
                    textarea![
                        class!["form-control"],
                        input_ev(Ev::Input, Msg::CommentTextEntered),
                        attrs!{
                            At::Rows => 3,
                            At::Placeholder => "Write a comment...",
                            At::Value => comment_text,
                        }
                    ]
                ],
                div![
                    class!["card-footer"],
                    img![
                        class!["comment-author-img"],
                        attrs!{At::Src => viewer.avatar().src()}
                    ],
                    button![
                        class!["btn", "btn-sm", "btn-primary"],
                        attrs!{At::Disabled => post_comment_disabled.as_at_value()},
                        "Post Comment"
                    ]
                ]
            ]
        }
    }
}

fn view_delete_comment_button(slug: &article::slug::Slug, comment: &article::comment::Comment) -> Node<Msg> {
    match comment.author {
        author::Author::IsViewer(..) => {
            span![
                class!["mod-options"],
                i![
                    class!["ion-trash-a"],
                    simple_ev(Ev::Click, Msg::DeleteCommentClicked(slug.clone(), comment.id.clone()))
                ]
            ]
        }
        _ => empty![]
    }
}

fn view_comment(slug: &article::slug::Slug, comment: &article::comment::Comment) -> Node<Msg> {
    div![
        class!["card"],
        div![
            class!["card-block"],
            p![
                class!["card-text"],
                comment.body
            ]
        ],
        div![
            class!["card-footer"],
            a![
                class!["comment-author"],
                attrs!{At::Href => route::Route::Profile(Cow::Borrowed(comment.author.username())).to_string()},
                img![
                    class!["comment-author-img"],
                    attrs!{At::Src => comment.author.profile().avatar.src()}
                ]
            ],
            raw!("&nbsp;"),
            a![
                class!["comment-author"],
                attrs!{At::Href => route::Route::Profile(Cow::Borrowed(comment.author.username())).to_string()},
                comment.author.username().to_string()
            ],
            span![
                class!["date-posted"],
                timestamp::view(&comment.created_at)
            ],
            view_delete_comment_button(slug, comment)
        ]
    ]
}

fn view_comments(slug: &article::slug::Slug, comments: &VecDeque<article::comment::Comment>) -> Vec<Node<Msg>> {
    comments
        .iter()
        .map(|comment| view_comment(slug, comment))
        .collect()
}

fn view_form_and_comments(slug: &article::slug::Slug, model: &Model) -> Vec<Node<Msg>> {
    match &model.comments {
        Status::Loading => vec![],
        Status::LoadingSlowly => vec![loading::icon()],
        Status::Failed => vec![loading::error("comments")],
        Status::Loaded((comment_text, comments)) => {
            vec![view_comment_form(comment_text, model)]
                .into_iter()
                .chain(view_comments(slug, comments))
                .collect()
        },
    }
}

fn view_content(model: &Model) -> Node<Msg> {
    match &model.article {
        Status::Loading => empty![],
        Status::LoadingSlowly => loading::icon(),
        Status::Failed => loading::error("article"),
        Status::Loaded(article) => {
            div![
                class!["article-page"],
                view_banner(article, model),

                div![
                    class!["container", "page"],

                    div![
                        class!["row", "article-content"],
                        div![
                            class!["col-md-12"],
                            md!(article.body.as_str())
                        ]
                    ],

                    hr![],

                    div![
                        class!["article-actions"],
                        view_article_meta(article, model)
                    ],

                    div![
                        class!["row"],
                        div![
                            class!["col-xs-12", "col-md-8", "offset-md-2"],
                            view_form_and_comments(&article.slug, model)
                        ]
                    ],

                ]

            ]
        },
    }
}