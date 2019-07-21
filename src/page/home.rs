use seed::prelude::*;
use super::ViewPage;
use crate::{session, GMsg, api, article, paginated_list, loading, request, page_number, logger, page};
use futures::prelude::*;

// Model

#[derive(Clone)]
pub enum FeedTab {
    YourFeed(api::Credentials),
    GlobalFeed,
    TagFeed(article::tag::Tag)
}

// @TODO remove unnecessary defaults?
impl Default for FeedTab {
    fn default() -> Self {
        FeedTab::GlobalFeed
    }
}

enum Status<T> {
    Loading,
    LoadingSlowly,
    Loaded(T),
    Failed
}

impl<T> Default for Status<T> {
    fn default() -> Self {
        Status::Loading
    }
}

#[derive(Default)]
pub struct Model {
    session: session::Session,
    feed_tab: FeedTab,
    feed_page: page_number::PageNumber,
    tags: Status<Vec<article::tag::Tag>>,
    feed: Status<article::feed::Model>,
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

pub fn init(session: session::Session, orders: &mut impl Orders<Msg, GMsg>) -> Model {
    let credentials = session.viewer().map(|viewer|&viewer.credentials);
    let feed_tab = credentials
        .map(|credentials| FeedTab::YourFeed(credentials.clone()))
        .unwrap_or_else(|| FeedTab::GlobalFeed);

    orders
        .perform_cmd(loading::slow_threshold(Msg::SlowLoadThresholdPassed, Msg::Unreachable))
        .perform_cmd(request::tag::load_list(Msg::TagsLoadCompleted))
        .perform_cmd(fetch_feed(
            session.credentials().cloned(),
            &feed_tab,
            page_number::PageNumber::default(),
        ));

    Model {
        session,
        feed_tab,
        ..Model::default()
    }
}

fn fetch_feed(
    credentials: Option<api::Credentials>,
    feed_tab: &FeedTab,
    page_number: page_number::PageNumber,
) -> impl Future<Item=Msg, Error=Msg> {
    request::feed::load_for_home(
        credentials,
        feed_tab,
        page_number,
        Msg::FeedLoadCompleted,
    )
}

// Sink

pub fn sink(g_msg: GMsg, model: &mut Model, _: &mut impl Orders<Msg, GMsg>) {
    match g_msg {
        GMsg::SessionChanged(session) => {
            model.session = session;
        }
        _ => ()
    }
}

// Update

#[derive(Clone)]
pub enum Msg {
    TagClicked(article::tag::Tag),
    TabClicked(FeedTab),
    FeedPageClicked(page_number::PageNumber),
    FeedLoadCompleted(Result<paginated_list::PaginatedList<article::Article>, Vec<String>>),
    TagsLoadCompleted(Result<Vec<article::tag::Tag>, Vec<String>>),
    FeedMsg(article::feed::Msg),
    SlowLoadThresholdPassed,
    Unreachable,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::TagClicked(tag) => {
            model.feed_tab = FeedTab::TagFeed(tag);
            model.feed_page = page_number::PageNumber::default();
            orders.perform_cmd(fetch_feed(
                model.session().credentials().cloned(),
                &model.feed_tab,
                model.feed_page,
            ));
        },
        Msg::TabClicked(feed_tab) => {
            model.feed_tab = feed_tab;
            model.feed_page = page_number::PageNumber::default();
            orders.perform_cmd(fetch_feed(
                model.session().credentials().cloned(),
                &model.feed_tab,
                model.feed_page,
            ));
        },
        Msg::FeedPageClicked(page_number) => {
            model.feed_page = page_number;
            orders.perform_cmd(fetch_feed(
                model.session().credentials().cloned(),
                &model.feed_tab,
                model.feed_page,
            ));
            page::scroll_to_top()
        },
        Msg::FeedLoadCompleted(Ok(paginated_list)) => {
            model.feed = Status::Loaded(
                article::feed::init(model.session.clone(),paginated_list)
            );
        },
        Msg::FeedLoadCompleted(Err(errors)) => {
            model.feed = Status::Failed;
            logger::errors(errors.clone());
        },
        Msg::TagsLoadCompleted(Ok(tags)) => {
            model.tags = Status::Loaded(tags);
        },
        Msg::TagsLoadCompleted(Err(errors)) => {
            model.tags = Status::Failed;
            logger::errors(errors.clone());
        },
        Msg::FeedMsg(feed_msg) => {
            match &mut model.feed {
                Status::Loaded(feed_model) => {
                    article::feed::update(
                        feed_msg, feed_model, &mut orders.proxy(Msg::FeedMsg)
                    )
                },
                _ => logger::error("FeedMsg can be handled only if Status is Loaded"),
            }
        },
        Msg::SlowLoadThresholdPassed => {
            if let Status::Loading = model.feed {
                model.feed = Status::LoadingSlowly
            }
            if let Status::Loading = model.tags {
                model.tags = Status::LoadingSlowly
            }
        },
        Msg::Unreachable => { logger::error("Unreachable!") },
    }
}

// View

pub fn view<'a>(model: &Model) -> ViewPage<'a, Msg> {
    ViewPage::new("Conduit", view_content(model))
}

fn view_banner() -> Node<Msg> {
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
    ]
}

fn view_tabs(model: &Model) -> Node<Msg> {
    let credentials = model.session().viewer().map(|viewer|&viewer.credentials);

    let your_feed = |credentials: api::Credentials| {
        article::feed::Tab::new("Your Feed", Msg::TabClicked(FeedTab::YourFeed(credentials)))
    };
    let global_feed = article::feed::Tab::new("Global Feed", Msg::TabClicked(FeedTab::GlobalFeed));
    let tag_feed = |tag: article::tag::Tag| { article::feed::Tab::new(
        format!("#{}", tag), Msg::TabClicked(FeedTab::TagFeed(tag))
    )};

    match &model.feed_tab {
        FeedTab::YourFeed(credentials) => {
            article::feed::view_tabs(vec![
                your_feed(credentials.clone()).activate(),
                global_feed
            ])
        },
        FeedTab::GlobalFeed => {
            match credentials {
                Some(credentials) => {
                    article::feed::view_tabs(vec![
                        your_feed(credentials.clone()),
                        global_feed.activate()
                    ])
                }
                None => {
                    article::feed::view_tabs(vec![
                        global_feed.activate()
                    ])
                }
            }
        },
        FeedTab::TagFeed(tag) => {
            match credentials {
                Some(credentials) => {
                    article::feed::view_tabs(vec![
                        your_feed(credentials.clone()),
                        global_feed,
                        tag_feed(tag.clone()).activate()
                    ])
                }
                None => {
                    article::feed::view_tabs(vec![
                        global_feed,
                        tag_feed(tag.clone()).activate()
                    ])
                }
            }
        }
    }
}

fn view_tag(tag: article::tag::Tag) -> Node<Msg> {
    a![
        class!["tag-pill", "tag-default"],
        attrs!{At::Href => ""},
        tag.to_string(),
        simple_ev(Ev::Click, Msg::TagClicked(tag))
    ]
}

fn view_tags(model: &Model) -> Node<Msg> {
    match &model.tags {
        Status::Loading => empty![],
        Status::LoadingSlowly => loading::icon(),
        Status::Failed => loading::error("tags"),
        Status::Loaded(tags) => {
            div![
                class!["sidebar"],
                p![
                    "Popular Tags"
                ],
                div![
                    class!["tag-list"],
                    tags.clone().into_iter().map(view_tag)
                ]
            ]
        },
    }
}

fn view_feed(model: &Model) -> Node<Msg> {
    match &model.feed {
        Status::Loading => empty![],
        Status::LoadingSlowly => loading::icon(),
        Status::Failed => loading::error("feed"),
        Status::Loaded(feed_model) => {
            div![
                class!["container", "page"],
                div![
                    class!["row"],
                    div![
                        class!["col-md-9"],
                        div![
                            class!["feed-toggle"],
                            view_tabs(model),
                            article::feed::view_articles(feed_model).els().map_message(Msg::FeedMsg),
                            article::feed::view_pagination(
                                feed_model, model.feed_page, Msg::FeedPageClicked
                            )
                        ],
                    ],
                    div![
                        class!["col-md-3"],
                        view_tags(model)
                    ]
                ]
            ]
        },
    }
}

fn view_content(model: &Model) -> Node<Msg> {
    div![
        class!["home-page"],
        view_banner(),
        view_feed(model),
    ]
}