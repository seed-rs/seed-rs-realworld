use crate::{session, paginated_list, article, api, GMsg, route, author, request, timestamp, page_number};
use seed::prelude::*;
use std::borrow::Cow;
use crate::api::Credentials;

// Model

#[derive(Default)]
pub struct Model {
    session: session::Session,
    errors: Vec<String>,
    articles: paginated_list::PaginatedList<article::Article>,
    is_loading: bool,
}

// Init

pub fn init(
    session: session::Session,
    articles: paginated_list::PaginatedList<article::Article>
) -> Model {
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
    active: bool
}

impl<Ms> Tab<Ms> {
    pub fn new(title: impl Into<Cow<'static, str>>, msg: Ms) -> Self {
        Self {
            title: title.into(),
            msg,
            active: false
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
            attrs!{At::Href => ""},
            tab.title,
            simple_ev(Ev::Click, tab.msg)
        ]
    ]
}

fn view_page_link<Ms: Clone>(
    page_number: page_number::PageNumber,
    active: bool,
    msg: Ms
) -> Node<Ms> {
    li![
        class!["page-item", "active" => active],
        a![
            class!["page-link"],
            attrs!{At::Href => ""},
            simple_ev(Ev::Click, msg),
            page_number.to_string()
        ]
    ]
}

pub fn view_pagination<Ms: Clone>(
    model: &Model,
    current_page: page_number::PageNumber,
    msg_constructor: fn(page_number::PageNumber) -> Ms
) -> Node<Ms> {
    if model.articles.total_pages() > 1 {
        ul![
            class!["pagination"],
            (1..=model.articles.total_pages())
                .map(page_number::PageNumber::new)
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

fn view_favorite_button(credentials: Option<&Credentials>, article: &article::Article) -> Node<Msg> {
    match credentials {
        None => empty![],
        Some(credentials) => {
            if article.favorited {
                button![
                    class!["btn","btn-primary", "btn-sm", "pull-xs-right"],
                    simple_ev(Ev::Click, Msg::FavoriteClicked(credentials.clone(), article.slug.clone())),
                    i![
                        class!["ion-heart"],
                        format!(" {}", article.favorites_count),
                    ]
                ]
            } else {
                button![
                    class!["btn","btn-outline-primary", "btn-sm", "pull-xs-right"],
                    simple_ev(Ev::Click, Msg::UnfavoriteClicked(credentials.clone(), article.slug.clone())),
                    i![
                        class!["ion-heart"],
                        format!(" {}", article.favorites_count),
                    ]
                ]
            }
        }
    }
}

fn view_tag(tag: String) -> Node<Msg> {
    li![
        class!["tag-default", "tag-pill", "tag-outline"],
        tag
    ]
}

fn view_article_preview(credentials: Option<&api::Credentials>, article: &article::Article) -> Node<Msg> {
    div![
        class!["article-preview"],
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
                timestamp::view(&article.created_at)
            ],
            view_favorite_button(credentials, article)
        ],
        a![
            class!["preview-link"],
            attrs!{At::Href => route::Route::Article(article.slug.clone()).to_string()},
            h1![
                article.title
            ],
            p![
                article.description
            ],
            span![
                "Read more..."
            ],
            ul![
                class!["tag-list"],
                article.tag_list.clone().into_iter().map(view_tag)
            ]
        ]
    ]
}

pub fn view_articles(model: &Model) -> Vec<Node<Msg>> {
    let credentials = model.session.viewer().map(|viewer|&viewer.credentials);
    if model.articles.total == 0 {
        vec![
            div![
                class!["article-preview"],
                "No articles are here... yet."
            ]
        ]
    } else {
        model.articles.values.iter().map(|article| view_article_preview(credentials, article)).collect()
    }
    // @TODO view errors
}

// Update

#[derive(Clone)]
pub enum Msg {
    DismissErrorsClicked,
    FavoriteClicked(api::Credentials, article::slug::Slug),
    UnfavoriteClicked(api::Credentials, article::slug::Slug),
    FavoriteCompleted(Result<article::Article, Vec<String>>),
}

pub fn update(
    msg: Msg,
    model: &mut Model,
    orders: &mut impl Orders<Msg, GMsg>
){
    match msg {
        Msg::DismissErrorsClicked => {
            model.errors.clear();
        },
        Msg::FavoriteClicked(credentials, slug) => {
            orders.perform_cmd(request::unfavorite::unfavorite(
                &model.session,
                &slug,
                Msg::FavoriteCompleted
            ));
        },
        Msg::UnfavoriteClicked(credentials, slug) => {
            orders.perform_cmd(request::favorite::favorite(
                &model.session,
                &slug,
                Msg::FavoriteCompleted
            ));
        },
        Msg::FavoriteCompleted(Ok(article)) => {
            model
                .articles
                .values
                .iter_mut()
                .find(|old_article| old_article.slug == article.slug)
                .map(|old_article| *old_article = article);
        },
        Msg::FavoriteCompleted(Err(errors)) => {
            // @TODO resolve errors
        },
    }
}