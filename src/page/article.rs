use super::ViewPage;
use crate::entity::{
    author::{self, Author},
    timestamp, Article, Comment, CommentId, ErrorMessage, Slug,
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

// ------ ------
//     Model
// ------ ------

// ------ Model ------

#[derive(Default)]
pub struct Model {
    session: Session,
    errors: Vec<ErrorMessage>,
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

// ------ Status ------

enum Status<T> {
    Loading,
    LoadingSlowly,
    Loaded(T),
    Failed,
}

impl<T> Default for Status<T> {
    fn default() -> Self {
        Self::Loading
    }
}

// ------ CommentText ------

enum CommentText {
    Editing(String),
    Sending(String),
}

impl Default for CommentText {
    fn default() -> Self {
        Self::Editing("".into())
    }
}

// ------ ------
//     Init
// ------ ------

pub fn init(session: Session, slug: &Slug, orders: &mut impl Orders<Msg, GMsg>) -> Model {
    orders
        .perform_cmd(loading::notify_on_slow_load(
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

// ------ ------
//     Sink
// ------ ------

pub fn sink(g_msg: GMsg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match g_msg {
        GMsg::SessionChanged(session) => {
            model.session = session;
            route::go_to(Route::Home, orders);
        }
        _ => (),
    }
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone)]
pub enum Msg {
    DeleteArticleClicked(Slug),
    DeleteCommentClicked(Slug, CommentId),
    DismissErrorsClicked,
    FavoriteClicked(Slug),
    UnfavoriteClicked(Slug),
    FollowClicked(Author),
    UnfollowClicked(Author),
    PostCommentClicked(Slug),
    CommentTextEntered(String),
    LoadArticleCompleted(Result<Article, Vec<ErrorMessage>>),
    LoadCommentsCompleted(Result<VecDeque<Comment>, Vec<ErrorMessage>>),
    DeleteArticleCompleted(Result<(), Vec<ErrorMessage>>),
    DeleteCommentCompleted(Result<CommentId, Vec<ErrorMessage>>),
    FavoriteChangeCompleted(Result<Article, Vec<ErrorMessage>>),
    FollowChangeCompleted(Result<Author, Vec<ErrorMessage>>),
    PostCommentCompleted(Result<Comment, Vec<ErrorMessage>>),
    SlowLoadThresholdPassed,
    Unreachable,
}

#[allow(clippy::match_same_arms, clippy::too_many_lines)]
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
        Msg::FollowClicked(author) => {
            orders
                .perform_cmd(request::follow::follow(
                    model.session.viewer().cloned(),
                    author.username(),
                    Msg::FollowChangeCompleted,
                ))
                .skip();
        }
        Msg::UnfollowClicked(author) => {
            orders
                .perform_cmd(request::follow::unfollow(
                    model.session.viewer().cloned(),
                    author.username(),
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
            logger::errors(&errors);
            model.errors = errors
        }

        Msg::DeleteCommentCompleted(Ok(comment_id)) => {
            if let Status::Loaded((_, comments)) = &mut model.comments {
                comments.retain(|comment| comment.id != comment_id);
            }
        }
        Msg::DeleteCommentCompleted(Err(errors)) => {
            logger::errors(&errors);
            model.errors = errors
        }

        Msg::FavoriteChangeCompleted(Ok(article)) => {
            model.article = Status::Loaded(article);
        }
        Msg::FavoriteChangeCompleted(Err(errors)) => {
            logger::errors(&errors);
            model.errors = errors
        }

        Msg::FollowChangeCompleted(Ok(author)) => {
            if let Status::Loaded(article) = &mut model.article {
                article.author = author;
            }
        }
        Msg::FollowChangeCompleted(Err(errors)) => {
            logger::errors(&errors);
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
            logger::errors(&errors);
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

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> ViewPage<Msg> {
    ViewPage::new(title_prefix(&model.article), view_content(model))
}

// ====== PRIVATE ======

fn title_prefix(article: &Status<Article>) -> impl Into<Cow<str>> {
    match &article {
        Status::Loaded(article) => &article.title,
        _ => "Article",
    }
}

fn view_content(model: &Model) -> Node<Msg> {
    match &model.article {
        Status::Loading => empty![],
        Status::LoadingSlowly => loading::view_icon(),
        Status::Failed => loading::view_error("article"),
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

// ------ view form and comments

fn view_form_and_comments(slug: &Slug, model: &Model) -> Vec<Node<Msg>> {
    match &model.comments {
        Status::Loading => vec![],
        Status::LoadingSlowly => vec![loading::view_icon()],
        Status::Failed => vec![loading::view_error("comments")],
        Status::Loaded((comment_text, comments)) => {
            vec![view_comment_form(slug.clone(), comment_text, model)]
                .into_iter()
                .chain(view_comments(slug, comments))
                .collect()
        }
    }
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

fn view_comments(slug: &Slug, comments: &VecDeque<Comment>) -> Vec<Node<Msg>> {
    comments
        .iter()
        .map(|comment| view_comment(slug, comment))
        .collect()
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

// ------ view buttons ------

fn view_buttons(article: &Article, model: &Model) -> Vec<Node<Msg>> {
    match model.session.viewer() {
        None => vec![],
        Some(_) => match &article.author {
            Author::IsViewer(..) => vec![
                view_edit_button(article.slug.clone()),
                plain![" "],
                view_delete_button(article.slug.clone()),
            ],
            author @ Author::Following(_) => vec![
                author::view_unfollow_button(
                    Msg::UnfollowClicked(author.clone()),
                    author.username(),
                ),
                plain![" "],
                view_favorite_button(article),
            ],
            author @ Author::NotFollowing(_) => vec![
                author::view_follow_button(Msg::FollowClicked(author.clone()), author.username()),
                plain![" "],
                view_favorite_button(article),
            ],
        },
    }
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

fn view_edit_button(slug: Slug) -> Node<Msg> {
    a![
        class!["btn", "btn-outline-secondary", "btn-sm"],
        attrs! {At::Href => Route::EditArticle(slug).to_string()},
        i![class!["ion-edit"],],
        " Edit Article",
    ]
}

fn view_delete_button(slug: Slug) -> Node<Msg> {
    button![
        class!["btn", "btn-outline-danger", "btn-sm"],
        simple_ev(Ev::Click, Msg::DeleteArticleClicked(slug)),
        i![class!["ion-trash-a"]],
        " Delete Article",
    ]
}
