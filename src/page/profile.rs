use seed::prelude::*;
use super::ViewPage;
use crate::{session, username, GMsg, route, article, author, api, loading, request, paginated_list, page_number};
use std::borrow::{Cow, BorrowMut};
use futures::prelude::*;

static MY_PROFILE_TITLE: &'static str = "My Profile";
static DEFAULT_PROFILE: &'static str = "Profile";

// Model

#[derive(Default)]
pub struct Model<'a> {
    session: session::Session,
    errors: Vec<String>,
    feed_tab: FeedTab,
    feed_page: page_number::PageNumber,
    author: Status<'a, author::Author<'a>>,
    feed: Status<'a, article::feed::Model>
}

impl<'a> Status<'a, author::Author<'a>> {
    pub fn username(&'a self) -> &username::Username<'a> {
        match self {
            Status::Loading(username) => username,
            Status::LoadingSlowly(username) => username,
            Status::Loaded(author) => author.username(),
            Status::Failed(username) => username,
        }
    }
}

#[derive(Copy, Clone)]
pub enum FeedTab {
    MyArticles,
    FavoritedArticles
}

impl Default for FeedTab {
    fn default() -> Self {
        FeedTab::MyArticles
    }
}

enum Status<'a, T> {
    Loading(username::Username<'a>),
    LoadingSlowly(username::Username<'a>),
    Loaded(T),
    Failed(username::Username<'a>),
}

impl<'a, T> Default for Status<'a, T> {
    fn default() -> Self {
        Status::Loading("".into())
    }
}

impl<'a, T> Status<'a, T> {
    pub fn take(&mut self) -> Status<'a, T> {
        std::mem::replace(self, Status::default())
    }
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

pub fn init<'a>(session: session::Session, username: &username::Username<'a>, orders: &mut impl Orders<Msg, GMsg>
) -> Model<'a> {
    let static_username: username::Username<'static> = username.as_str().to_owned().into();
    orders
        .perform_cmd(loading::slow_threshold(Msg::SlowLoadThresholdPassed, Msg::NoOp))
        .perform_cmd(request::author_load::load_author(session.clone(), static_username.clone(), Msg::AuthorLoadCompleted))
        .perform_cmd(fetch_feed(
            session.clone(),
            static_username.clone(),
            FeedTab::default(),
            page_number::PageNumber::default(),
        ));

    Model {
        session,
        author: Status::Loading(username.clone()),
        feed: Status::Loading(username.clone()),
        ..Model::default()
    }
}

// @TODO merge with home feed?
fn fetch_feed(
    session: session::Session,
    username: username::Username<'static>,
    feed_tab: FeedTab,
    page_number: page_number::PageNumber,
) -> impl Future<Item=Msg, Error=Msg> {
    request::feed_load::load_feed(
        session,
        username,
        feed_tab,
        page_number,
        Msg::FeedLoadCompleted,
    )
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

#[derive(Clone)]
pub enum Msg {
    DismissErrorsClicked,
    FollowClicked(api::Credentials, author::UnfollowedAuthor<'static>),
    UnfollowClicked(api::Credentials, author::FollowedAuthor<'static>),
    TabClicked(FeedTab),
    FeedPageClicked(page_number::PageNumber),
    FollowChangeCompleted(Result<author::Author<'static>, Vec<String>>),
    AuthorLoadCompleted(Result<author::Author<'static>, (username::Username<'static>, Vec<String>)>),
    FeedLoadCompleted(
        Result<paginated_list::PaginatedList<article::Article>,
        (username::Username<'static>, Vec<String>)>
    ),
    FeedMsg(article::feed::Msg),
    SlowLoadThresholdPassed,
    NoOp,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::DismissErrorsClicked => {
            model.errors.clear();
        },
        Msg::FollowClicked(credentials, unfollowed_author) => {
            let static_username: username::Username<'static> =
                model.author.username().as_str().to_owned().into();
            orders.perform_cmd(
                request::follow::follow(
                    model.session.clone(),
                    static_username,
                    Msg::FollowChangeCompleted
                )
            );
        },
        Msg::UnfollowClicked(credentials, unfollowed_author) => {
            let static_username: username::Username<'static> =
                model.author.username().as_str().to_owned().into();
            orders.perform_cmd(
                request::unfollow::unfollow(
                    model.session.clone(),
                    static_username,
                    Msg::FollowChangeCompleted
                )
            );
        },
        Msg::TabClicked(feed_tab) => {
            let static_username: username::Username<'static> =
                model.author.username().as_str().to_owned().into();

            model.feed_tab = feed_tab;
            model.feed_page = page_number::PageNumber::default();
            orders
                .perform_cmd(fetch_feed(
                    model.session.clone(),
                    static_username,
                    feed_tab,
                    model.feed_page,
                ));
        },
        Msg::FeedPageClicked(page_number) => {
            let static_username: username::Username<'static> =
                model.author.username().as_str().to_owned().into();

            orders
                .perform_cmd(fetch_feed(
                    model.session.clone(),
                    static_username,
                    model.feed_tab,
                    page_number,
                ));
        },
        Msg::FollowChangeCompleted(Ok(author)) => {
            model.author = Status::Loaded(author)
        },
        Msg::FollowChangeCompleted(Err(errors)) => {
            // @TODO Log.error??
        },
        Msg::AuthorLoadCompleted(Ok(author)) => {
            model.author = Status::Loaded(author)
        },
        Msg::AuthorLoadCompleted(Err((username, errors))) => {
            model.author = Status::Failed(username);
            // @TODO Log.error??
        },
        Msg::FeedLoadCompleted(Ok(paginated_list)) => {
            model.feed = Status::Loaded(
                article::feed::init(model.session.clone(),paginated_list)
            );
        },
        Msg::FeedLoadCompleted(Err((username, errors))) => {
            model.feed = Status::Failed(username);
            // @TODO Log.error??
        },
        Msg::FeedMsg(feed_msg) => {
            match &mut model.feed {
                Status::Loaded(feed_model) => {
                    article::feed::update(
                        feed_msg, feed_model, &mut orders.proxy(Msg::FeedMsg)
                    )
                },
                Status::Loading(_) => {
                    // @TODO Log.error??
                },
                Status::LoadingSlowly(_) => {
                    // @TODO Log.error??
                },
                Status::Failed(_) => {
                    // @TODO Log.error??
                },
            }
        }
        Msg::SlowLoadThresholdPassed => {
            match model.feed.take() {
                Status::Loading(username) => {
                    model.feed = Status::LoadingSlowly(username)
                },
                feed => model.feed = feed
            }
        },
        Msg::NoOp => { orders.skip(); },
    }
}

// View

fn title_for_other(username: &username::Username) -> String {
    format!("Profile - {}", username.as_str())
}

fn title_for_me(credentials: Option<&api::Credentials>, username: &username::Username) -> &'static str {
    if let Some(credentials) = credentials {
        if username == &credentials.username {
            return MY_PROFILE_TITLE
        }
    }
    DEFAULT_PROFILE
}

fn title<'a>(model: &Model) -> Cow<'a, str> {
    match &model.author {
        Status::Loaded(author::Author::IsViewer(..)) => {
            MY_PROFILE_TITLE.into()
        },
        Status::Loaded(author ) => {
            title_for_other(author.username()).into()
        },
        Status::Loading(username) => {
            title_for_me(model.session().viewer().map(|viewer|&viewer.credentials), username).into()
        },
        Status::LoadingSlowly(username) => {
            title_for_me(model.session().viewer().map(|viewer|&viewer.credentials), username).into()
        },
        Status::Failed(username) => {
            title_for_me(model.session().viewer().map(|viewer|&viewer.credentials), username).into()
        },
    }
}

pub fn view<'a>(model: &'a Model) -> ViewPage<'a, Msg> {
    ViewPage::new(title(model), view_content(model))
}

fn view_tabs(feed_tab: FeedTab) -> Node<Msg> {
    let my_articles = article::feed::Tab::new("My Articles", Msg::TabClicked(FeedTab::MyArticles));
    let favorited_articles = article::feed::Tab::new("Favorited Articles", Msg::TabClicked(FeedTab::FavoritedArticles));

    match feed_tab {
        FeedTab::MyArticles => {
            article::feed::view_tabs(vec![my_articles.activate(), favorited_articles])
        },
        FeedTab::FavoritedArticles => {
            article::feed::view_tabs(vec![my_articles, favorited_articles.activate()])
        }
    }
}

fn view_feed(model: &Model) -> Node<Msg> {
    match &model.feed {
        Status::Loading(_) => empty![],
        Status::LoadingSlowly(_) => loading::icon(),
        Status::Failed(_) => loading::error("feed"),
        Status::Loaded(feed_model) => {
            div![
                class!["container"],
                div![
                    class!["row"],
                    div![
                        class!["col-xs-12", "col-md-10", "offset-md-1"],
                        div![
                            class!["articles-toggle"],
                            view_tabs(model.feed_tab),
                            article::feed::view_articles(feed_model).els().map_message(Msg::FeedMsg),
                            article::feed::view_pagination(
                                feed_model, model.feed_page, Msg::FeedPageClicked
                            )
                        ],
                    ]
                ]
            ]

        },
    }
}

fn view_follow_button(author: &author::Author, model: &Model) -> Node<Msg> {
    let credentials = model.session().viewer().map(|viewer| &viewer.credentials);
    match credentials {
        None => empty![],
        Some(credentials) => {
            match author {
                author::Author::IsViewer(..) => {
                    empty![]
                },
                author::Author::Following(followed_author) => {
                    author::view_unfollow_button(
                        Msg::UnfollowClicked(credentials.clone(), followed_author.to_static()),
                        author.username()
                    )
                }
                author::Author::NotFollowing(unfollowed_author) => {
                    author::view_follow_button(
                        Msg::FollowClicked(credentials.clone(), unfollowed_author.to_static()),
                        author.username()
                    )
                }
            }
        }
    }
}

fn view_content(model: &Model) -> Node<Msg> {
    match &model.author {
        Status::Loading(_) => empty![],
        Status::LoadingSlowly(_) => loading::icon(),
        Status::Failed(_) => loading::error("profile"),
        Status::Loaded(author) => {
            div![
                class!["profile-page"],

                // @TODO show errors!

                div![
                    class!["user-info"],
                    div![
                        class!["container"],
                        div![
                            class!["row"],

                            div![
                                class!["col-xs-12", "col-md-10", "offset-md-1"],
                                img![
                                    class!["user-img"],
                                    attrs!{At::Src => author.profile().avatar.src() }
                                ],
                                h4![
                                    author.username().to_string()
                                ],
                                p![
                                    author.profile().bio.as_ref().unwrap_or(&String::new())
                                ],
                                view_follow_button(author, model)
                            ]

                        ]
                    ]
                ],

                view_feed(model)
            ]
        },
    }
}