use crate::{session, paginated_list, article, api, GMsg};
use seed::prelude::*;

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
    title: &'static str,
    msg: Ms,
    active: bool
}

impl<Ms> Tab<Ms> {
    pub fn new(title: &'static str, msg: Ms) -> Self {
        Self {
            title,
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

pub fn view_pagination<Ms: Clone>() -> Node<Ms> {
    // @TODO implement with home page
    plain!("I'm pagination")
}

fn view_article_preview(credentials: Option<&api::Credentials>, article: &article::Article) -> Node<Msg> {
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
    ]
}

pub fn view_articles(model: &Model) -> Vec<Node<Msg>> {
    let credentials = model.session.viewer().map(|viewer|&viewer.credentials);
    model.articles.values.iter().map(|article| view_article_preview(credentials, article)).collect()
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
    credentials: Option<api::Credentials>,
    msg: Msg,
    model: &mut Model,
    orders: &mut impl Orders<Msg, GMsg>
){
    unimplemented!();
}