use super::ViewPage;
use crate::entity::{
    author::{self, Author, FollowedAuthor, UnfollowedAuthor},
    timestamp, Article, Comment, CommentId, Slug,
};
use crate::{
    helper::take,
    loading, logger, page, request,
    route::{self, Route},
    GMsg, Session,
};
use seed::prelude::*;
use std::borrow::Cow;
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
    Sending(String),
}

impl Default for CommentText {
    fn default() -> Self {
        CommentText::Editing("".into())
    }
}

#[derive(Default)]
pub struct Model {
    session: Session,
    errors: Vec<String>,
    comments: Status<(CommentText, VecDeque<Comment>)>,
    article: Status<Article>,
}

impl Model {
    pub const fn session(&self) -> &Session {
        &self.session
    }
}

impl From<Model> for Session {
    fn from(model: Model) -> Self {
        model.session
    }
}

pub fn init(session: Session, slug: &Slug, orders: &mut impl Orders<Msg, GMsg>) -> Model {
    orders
        .perform_cmd(loading::slow_threshold(
            Msg::SlowLoadThresholdPassed,
            Msg::Unreachable,
        ))
        .perform_cmd(request::article::load(
            session.viewer().cloned(),
            slug,
            Msg::LoadArticleCompleted,
        ))
        .perform_cmd(request::comment::load_list(
            session.viewer().cloned(),
            slug,
            Msg::LoadCommentsCompleted,
        ));

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
            route::go_to(Route::Home, orders);
        }
        _ => (),
    }
}

// Update

#[derive(Clone)]
pub enum Msg {
    DeleteArticleClicked(Slug),
    DeleteCommentClicked(Slug, CommentId),
    DismissErrorsClicked,
    FavoriteClicked(Slug),
    UnfavoriteClicked(Slug),
    FollowClicked(UnfollowedAuthor),
    UnfollowClicked(FollowedAuthor),
    PostCommentClicked(Slug),
    CommentTextEntered(String),
    LoadArticleCompleted(Result<Article, Vec<String>>),
    LoadCommentsCompleted(Result<VecDeque<Comment>, Vec<String>>),
    DeleteArticleCompleted(Result<(), Vec<String>>),
    DeleteCommentCompleted(Result<CommentId, Vec<String>>),
    FavoriteChangeCompleted(Result<Article, Vec<String>>),
    FollowChangeCompleted(Result<Author, Vec<String>>),
    PostCommentCompleted(Result<Comment, Vec<String>>),
    SlowLoadThresholdPassed,
    Unreachable,
}

#[allow(clippy::match_same_arms)]
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::DeleteArticleClicked(slug) => {
            orders
                .perform_cmd(request::article::delete(
                    model.session.viewer(),
                    &slug,
                    Msg::DeleteArticleCompleted,
                ))
                .skip();
        }
        Msg::DeleteCommentClicked(slug, comment_id) => {
            orders
                .perform_cmd(request::comment::delete(
                    model.session.viewer(),
                    &slug,
                    comment_id,
                    Msg::DeleteCommentCompleted,
                ))
                .skip();
        }
        Msg::DismissErrorsClicked => {
            model.errors.clear();
        }
        Msg::FavoriteClicked(slug) => {
            orders
                .perform_cmd(request::favorite::unfavorite(
                    model.session.viewer().cloned(),
                    &slug,
                    Msg::FavoriteChangeCompleted,
                ))
                .skip();
        }
        Msg::UnfavoriteClicked(slug) => {
            orders
                .perform_cmd(request::favorite::favorite(
                    model.session.viewer().cloned(),
                    &slug,
                    Msg::FavoriteChangeCompleted,
                ))
                .skip();
        }
        Msg::FollowClicked(unfollowed_author) => {
            orders
                .perform_cmd(request::follow::follow(
                    model.session.viewer().cloned(),
                    &unfollowed_author.profile.username,
                    Msg::FollowChangeCompleted,
                ))
                .skip();
        }
        Msg::UnfollowClicked(followed_author) => {
            orders
                .perform_cmd(request::follow::unfollow(
                    model.session.viewer().cloned(),
                    &followed_author.profile.username,
                    Msg::FollowChangeCompleted,
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
                    orders.perform_cmd(request::comment::create(
                        model.session.viewer().cloned(),
                        &slug,
                        text.clone(),
                        Msg::PostCommentCompleted,
                    ));
                    *model_comments =
                        Status::Loaded((CommentText::Sending(take(text)), take(comments)));
                }
                _ => logger::error("Comment can be created only in Editing mode!"),
            }
        }
        Msg::CommentTextEntered(comment_text) => match &mut model.comments {
            Status::Loaded((CommentText::Editing(text), _)) => {
                *text = comment_text;
            }
            _ => logger::error("Comment text can be changed only in Editing mode!"),
        },

        Msg::LoadArticleCompleted(Ok(article)) => model.article = Status::Loaded(article),
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
            route::go_to(Route::Home, orders);
        }
        Msg::DeleteArticleCompleted(Err(errors)) => {
            logger::errors(errors.clone());
            model.errors = errors
        }

        Msg::DeleteCommentCompleted(Ok(comment_id)) => {
            if let Status::Loaded((_, comments)) = &mut model.comments {
                comments.retain(|comment| comment.id != comment_id);
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
        Msg::Unreachable => logger::error("Unreachable!"),
    }
}

// View

fn title(article: &Status<Article>) -> impl Into<Cow<str>> {
    match &article {
        Status::Loaded(article) => &article.title,
        _ => "Article",
    }
}

pub fn view(model: &Model) -> ViewPage<Msg> {
    ViewPage::new(title(&model.article), view_content(model))
}

fn view_favorite_button(article: &Article) -> Node<Msg> {
    if article.favorited {
        button![
            class!["btn", "btn-primary", "btn-sm"],
            simple_ev(Ev::Click, Msg::FavoriteClicked(article.slug.clone())),
            i![
                class!["ion-heart"],
                format!(" Favorite Article ({})", article.favorites_count),
            ]
        ]
    } else {
        button![
            class!["btn", "btn-outline-primary", "btn-sm"],
            simple_ev(Ev::Click, Msg::UnfavoriteClicked(article.slug.clone())),
            i![
                class!["ion-heart"],
                format!(" Favorite Article ({})", article.favorites_count),
            ]
        ]
    }
}

fn view_delete_button(slug: Slug) -> Node<Msg> {
    button![
        class!["btn", "btn-outline-danger", "btn-sm"],
        simple_ev(Ev::Click, Msg::DeleteArticleClicked(slug)),
        i![class!["ion-trash-a"]],
        " Delete Article",
    ]
}

fn view_edit_button(slug: Slug) -> Node<Msg> {
    a![
        class!["btn", "btn-outline-secondary", "btn-sm"],
        attrs! {At::Href => Route::EditArticle(slug).to_string()},
        i![class!["ion-edit"],],
        " Edit Article",
    ]
}

fn view_buttons(article: &Article, model: &Model) -> Vec<Node<Msg>> {
    match model.session.viewer() {
        None => vec![],
        Some(_) => match &article.author {
            Author::IsViewer(..) => vec![
                view_edit_button(article.slug.clone()),
                plain![" "],
                view_delete_button(article.slug.clone()),
            ],
            Author::Following(followed_author) => vec![
                author::view_unfollow_button(
                    Msg::UnfollowClicked(followed_author.clone()),
                    &followed_author.profile.username,
                ),
                plain![" "],
                view_favorite_button(article),
            ],
            Author::NotFollowing(unfollowed_author) => vec![
                author::view_follow_button(
                    Msg::FollowClicked(unfollowed_author.clone()),
                    &unfollowed_author.profile.username,
                ),
                plain![" "],
                view_favorite_button(article),
            ],
        },
    }
}

fn view_article_meta(article: &Article, model: &Model) -> Node<Msg> {
    div![
        class!["article-meta"],
        a![
            attrs! {At::Href => Route::Profile(Cow::Borrowed(article.author.username())).to_string()},
            img![attrs! {At::Src => article.author.profile().avatar.src()}]
        ],
        div![
            class!["info"],
            author::view(article.author.username()),
            span![class!["date"], timestamp::view(&article.created_at)]
        ],
        view_buttons(article, model),
    ]
}

fn view_banner(article: &Article, model: &Model) -> Node<Msg> {
    div![
        class!["banner"],
        div![
            class!["container"],
            h1![article.title],
            view_article_meta(article, model),
            page::view_errors(Msg::DismissErrorsClicked, &model.errors),
        ]
    ]
}

fn view_comment_form(slug: Slug, comment_text: &CommentText, model: &Model) -> Node<Msg> {
    match model.session.viewer() {
        None => p![
            a!["Sign in", attrs! {At::Href => Route::Login.to_string()}],
            " or ",
            a!["Sign up", attrs! {At::Href => Route::Register.to_string()}],
            " to comment."
        ],
        Some(viewer) => {
            let (comment_text, post_comment_disabled) = match comment_text {
                CommentText::Editing(text) => (text, false),
                CommentText::Sending(text) => (text, true),
            };

            form![
                class!["card", "comment-form"],
                raw_ev(Ev::Submit, |event| {
                    event.prevent_default();
                    Msg::PostCommentClicked(slug)
                }),
                div![
                    class!["card-block"],
                    textarea![
                        class!["form-control"],
                        input_ev(Ev::Input, Msg::CommentTextEntered),
                        attrs! {
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
                        attrs! {At::Src => viewer.avatar().src()}
                    ],
                    button![
                        class!["btn", "btn-sm", "btn-primary"],
                        attrs! {At::Disabled => post_comment_disabled.as_at_value()},
                        "Post Comment"
                    ]
                ]
            ]
        }
    }
}

fn view_delete_comment_button(slug: &Slug, comment: &Comment) -> Node<Msg> {
    match comment.author {
        Author::IsViewer(..) => span![
            class!["mod-options"],
            i![
                class!["ion-trash-a"],
                simple_ev(
                    Ev::Click,
                    Msg::DeleteCommentClicked(slug.clone(), comment.id.clone())
                )
            ]
        ],
        _ => empty![],
    }
}

fn view_comment(slug: &Slug, comment: &Comment) -> Node<Msg> {
    div![
        class!["card"],
        div![class!["card-block"], p![class!["card-text"], comment.body]],
        div![
            class!["card-footer"],
            a![
                class!["comment-author"],
                attrs! {At::Href => Route::Profile(Cow::Borrowed(comment.author.username())).to_string()},
                img![
                    class!["comment-author-img"],
                    attrs! {At::Src => comment.author.profile().avatar.src()}
                ]
            ],
            raw!("&nbsp;"),
            a![
                class!["comment-author"],
                attrs! {At::Href => Route::Profile(Cow::Borrowed(comment.author.username())).to_string()},
                comment.author.username().to_string()
            ],
            span![class!["date-posted"], timestamp::view(&comment.created_at)],
            view_delete_comment_button(slug, comment)
        ]
    ]
}

fn view_comments(slug: &Slug, comments: &VecDeque<Comment>) -> Vec<Node<Msg>> {
    comments
        .iter()
        .map(|comment| view_comment(slug, comment))
        .collect()
}

fn view_form_and_comments(slug: &Slug, model: &Model) -> Vec<Node<Msg>> {
    match &model.comments {
        Status::Loading => vec![],
        Status::LoadingSlowly => vec![loading::icon()],
        Status::Failed => vec![loading::error("comments")],
        Status::Loaded((comment_text, comments)) => {
            vec![view_comment_form(slug.clone(), comment_text, model)]
                .into_iter()
                .chain(view_comments(slug, comments))
                .collect()
        }
    }
}

fn view_content(model: &Model) -> Node<Msg> {
    match &model.article {
        Status::Loading => empty![],
        Status::LoadingSlowly => loading::icon(),
        Status::Failed => loading::error("article"),
        Status::Loaded(article) => div![
            class!["article-page"],
            view_banner(article, model),
            div![
                class!["container", "page"],
                div![
                    class!["row", "article-content"],
                    div![class!["col-md-12"], md!(article.body.as_str())]
                ],
                hr![],
                div![class!["article-actions"], view_article_meta(article, model)],
                div![
                    class!["row"],
                    div![
                        class!["col-xs-12", "col-md-8", "offset-md-2"],
                        view_form_and_comments(&article.slug, model)
                    ]
                ],
            ]
        ],
    }
}
