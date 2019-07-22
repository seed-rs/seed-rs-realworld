use crate::entity::{author, timestamp, Article, PageNumber, PaginatedList, Slug, Tag, Viewer};
use crate::{logger, page, request, GMsg, Route, Session};
use seed::prelude::*;
use std::borrow::Cow;

// Model

#[derive(Default)]
pub struct Model {
    session: Session,
    errors: Vec<String>,
    articles: PaginatedList<Article>,
}

// Init

pub fn init(session: Session, articles: PaginatedList<Article>) -> Model {
    Model {
        session,
        articles,
        ..Model::default()
    }
}

// View

pub struct Tab<Ms> {
    title: Cow<'static, str>,
    msg: Ms,
    active: bool,
}

impl<Ms> Tab<Ms> {
    pub fn new(title: impl Into<Cow<'static, str>>, msg: Ms) -> Self {
        Self {
            title: title.into(),
            msg,
            active: false,
        }
    }
    pub fn activate(mut self) -> Self {
        self.active = true;
        self
    }
}

pub fn view_tabs<Ms: Clone>(tabs: Vec<Tab<Ms>>) -> Node<Ms> {
    ul![
        class!["nav", "nav-pills", "outline-active"],
        tabs.into_iter().map(view_tab)
    ]
}

fn view_tab<Ms: Clone>(tab: Tab<Ms>) -> Node<Ms> {
    li![
        class!["nav-item"],
        a![
            class!["nav-link", "active" => tab.active],
            attrs! {At::Href => ""},
            tab.title,
            simple_ev(Ev::Click, tab.msg)
        ]
    ]
}

fn view_page_link<Ms: Clone>(page_number: PageNumber, active: bool, msg: Ms) -> Node<Ms> {
    li![
        class!["page-item", "active" => active],
        a![
            class!["page-link"],
            attrs! {At::Href => ""},
            simple_ev(Ev::Click, msg),
            page_number.to_string()
        ]
    ]
}

pub fn view_pagination<Ms: Clone>(
    model: &Model,
    current_page: PageNumber,
    msg_constructor: fn(PageNumber) -> Ms,
) -> Node<Ms> {
    if model.articles.total_pages() > 1 {
        ul![
            class!["pagination"],
            (1..=model.articles.total_pages())
                .map(PageNumber::new)
                .map(|page_number| view_page_link(
                    page_number,
                    page_number == current_page,
                    msg_constructor(page_number)
                ))
        ]
    } else {
        empty![]
    }
}

fn view_favorite_button(viewer: Option<&Viewer>, article: &Article) -> Node<Msg> {
    match viewer {
        None => empty![],
        Some(_) => {
            if article.favorited {
                button![
                    class!["btn", "btn-primary", "btn-sm", "pull-xs-right"],
                    simple_ev(Ev::Click, Msg::FavoriteClicked(article.slug.clone())),
                    i![class!["ion-heart"], format!(" {}", article.favorites_count),]
                ]
            } else {
                button![
                    class!["btn", "btn-outline-primary", "btn-sm", "pull-xs-right"],
                    simple_ev(Ev::Click, Msg::UnfavoriteClicked(article.slug.clone())),
                    i![class!["ion-heart"], format!(" {}", article.favorites_count),]
                ]
            }
        }
    }
}

fn view_tag(tag: Tag) -> Node<Msg> {
    li![
        class!["tag-default", "tag-pill", "tag-outline"],
        tag.to_string()
    ]
}

fn view_article_preview(viewer: Option<&Viewer>, article: &Article) -> Node<Msg> {
    div![
        class!["article-preview"],
        div![
            class!["article-meta"],
            a![
                attrs! {At::Href => Route::Profile(Cow::Borrowed(article.author.username())).to_string()},
                img![attrs! {At::Src => article.author.profile().avatar.src()}]
            ],
            div![
                class!["info"],
                author::view(article.author.username()),
                timestamp::view(&article.created_at)
            ],
            view_favorite_button(viewer, article)
        ],
        a![
            class!["preview-link"],
            attrs! {At::Href => Route::Article(article.slug.clone()).to_string()},
            h1![article.title],
            p![article.description],
            span!["Read more..."],
            ul![
                class!["tag-list"],
                article.tag_list.clone().into_iter().map(view_tag)
            ]
        ]
    ]
}

pub fn view_articles(model: &Model) -> Vec<Node<Msg>> {
    vec![page::view_errors(
        Msg::DismissErrorsClicked,
        model.errors.clone(),
    )]
    .into_iter()
    .chain(if model.articles.total == 0 {
        vec![div![
            class!["article-preview"],
            "No articles are here... yet."
        ]]
    } else {
        model
            .articles
            .items
            .iter()
            .map(|article| view_article_preview(model.session.viewer(), article))
            .collect()
    })
    .collect()
}

// Update

#[derive(Clone)]
pub enum Msg {
    DismissErrorsClicked,
    FavoriteClicked(Slug),
    UnfavoriteClicked(Slug),
    FavoriteCompleted(Result<Article, Vec<String>>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::DismissErrorsClicked => {
            model.errors.clear();
        }
        Msg::FavoriteClicked(slug) => {
            orders
                .perform_cmd(request::favorite::unfavorite(
                    model.session.viewer().cloned(),
                    &slug,
                    Msg::FavoriteCompleted,
                ))
                .skip();
        }
        Msg::UnfavoriteClicked(slug) => {
            orders
                .perform_cmd(request::favorite::favorite(
                    model.session.viewer().cloned(),
                    &slug,
                    Msg::FavoriteCompleted,
                ))
                .skip();
        }
        Msg::FavoriteCompleted(Ok(article)) => {
            model
                .articles
                .items
                .iter_mut()
                .find(|old_article| old_article.slug == article.slug)
                .map(|old_article| *old_article = article);
        }
        Msg::FavoriteCompleted(Err(errors)) => {
            logger::errors(errors.clone());
            model.errors = errors;
        }
    }
}
