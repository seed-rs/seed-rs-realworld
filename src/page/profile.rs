use super::ViewPage;
use crate::entity::{
    article::{self, Article},
    author::{self, Author},
    PageNumber, PaginatedList, Username, Viewer,
};
use crate::{
    helper::take,
    loading, logger, page, request,
    route::{self, Route},
    GMsg, Session,
};
use futures::prelude::*;
use seed::prelude::*;
use std::borrow::Cow;

static MY_PROFILE_TITLE: &'static str = "My Profile";
static DEFAULT_PROFILE: &'static str = "Profile";

// Model

#[derive(Default)]
pub struct Model<'a> {
    session: Session,
    errors: Vec<String>,
    selected_feed: SelectedFeed,
    feed_page: PageNumber,
    author: Status<'a, Author>,
    feed: Status<'a, article::feed::Model>,
}

impl<'a> Status<'a, Author> {
    pub fn username(&'a self) -> &Username<'a> {
        match self {
            Status::Loading(username)
            | Status::LoadingSlowly(username)
            | Status::Failed(username) => username,
            Status::Loaded(author) => author.username(),
        }
    }
}

#[derive(Copy, Clone)]
pub enum SelectedFeed {
    MyArticles,
    FavoritedArticles,
}

impl Default for SelectedFeed {
    fn default() -> Self {
        SelectedFeed::MyArticles
    }
}

enum Status<'a, T> {
    Loading(Username<'a>),
    LoadingSlowly(Username<'a>),
    Loaded(T),
    Failed(Username<'a>),
}

impl<'a, T> Default for Status<'a, T> {
    fn default() -> Self {
        Status::Loading("".into())
    }
}

impl<'a> Model<'a> {
    pub const fn session(&self) -> &Session {
        &self.session
    }
}

impl<'a> From<Model<'a>> for Session {
    fn from(model: Model) -> Self {
        model.session
    }
}

pub fn init<'a>(
    session: Session,
    username: Username<'static>,
    orders: &mut impl Orders<Msg, GMsg>,
) -> Model<'a> {
    orders
        .perform_cmd(loading::slow_threshold(
            Msg::SlowLoadThresholdPassed,
            Msg::Unreachable,
        ))
        .perform_cmd(request::author::load(
            session.viewer().cloned(),
            username.clone(),
            Msg::AuthorLoadCompleted,
        ))
        .perform_cmd(fetch_feed(
            session.viewer().cloned(),
            username.clone(),
            SelectedFeed::default(),
            PageNumber::default(),
        ));

    Model {
        session,
        author: Status::Loading(username.clone()),
        feed: Status::Loading(username),
        ..Model::default()
    }
}

fn fetch_feed(
    viewer: Option<Viewer>,
    username: Username<'static>,
    selected_feed: SelectedFeed,
    page_number: PageNumber,
) -> impl Future<Item = Msg, Error = Msg> {
    request::feed::load_for_profile(
        viewer,
        username,
        selected_feed,
        page_number,
        Msg::FeedLoadCompleted,
    )
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
#[allow(clippy::pub_enum_variant_names)]
pub enum Msg {
    DismissErrorsClicked,
    FollowClicked,
    UnfollowClicked,
    TabClicked(SelectedFeed),
    FeedPageClicked(PageNumber),
    FollowChangeCompleted(Result<Author, Vec<String>>),
    AuthorLoadCompleted(Result<Author, (Username<'static>, Vec<String>)>),
    FeedLoadCompleted(Result<PaginatedList<Article>, (Username<'static>, Vec<String>)>),
    FeedMsg(article::feed::Msg),
    SlowLoadThresholdPassed,
    Unreachable,
}

#[allow(clippy::match_same_arms)]
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::DismissErrorsClicked => {
            model.errors.clear();
        }
        Msg::FollowClicked => {
            orders
                .perform_cmd(request::follow::follow(
                    model.session.viewer().cloned(),
                    model.author.username(),
                    Msg::FollowChangeCompleted,
                ))
                .skip();
        }
        Msg::UnfollowClicked => {
            orders
                .perform_cmd(request::follow::unfollow(
                    model.session.viewer().cloned(),
                    model.author.username(),
                    Msg::FollowChangeCompleted,
                ))
                .skip();
        }
        Msg::TabClicked(selected_feed) => {
            model.selected_feed = selected_feed;
            model.feed_page = PageNumber::default();
            orders.perform_cmd(fetch_feed(
                model.session.viewer().cloned(),
                model.author.username().to_static(),
                selected_feed,
                model.feed_page,
            ));
        }
        Msg::FeedPageClicked(page_number) => {
            model.feed_page = page_number;
            orders.perform_cmd(fetch_feed(
                model.session.viewer().cloned(),
                model.author.username().to_static(),
                model.selected_feed,
                model.feed_page,
            ));
            page::scroll_to_top();
        }
        Msg::FollowChangeCompleted(Ok(author)) => model.author = Status::Loaded(author),
        Msg::FollowChangeCompleted(Err(errors)) => {
            logger::errors(errors.clone());
            model.errors = errors;
        }
        Msg::AuthorLoadCompleted(Ok(author)) => model.author = Status::Loaded(author),
        Msg::AuthorLoadCompleted(Err((username, errors))) => {
            model.author = Status::Failed(username);
            logger::errors(errors.clone());
            model.errors = errors;
        }
        Msg::FeedLoadCompleted(Ok(paginated_list)) => {
            model.feed = Status::Loaded(article::feed::init(model.session.clone(), paginated_list));
        }
        Msg::FeedLoadCompleted(Err((username, errors))) => {
            model.feed = Status::Failed(username);
            logger::errors(errors.clone());
            model.errors = errors;
        }
        Msg::FeedMsg(feed_msg) => match &mut model.feed {
            Status::Loaded(feed_model) => {
                article::feed::update(feed_msg, feed_model, &mut orders.proxy(Msg::FeedMsg))
            }
            _ => logger::error("FeedMsg can be handled only if Status is Loaded"),
        },
        Msg::SlowLoadThresholdPassed => {
            if let Status::Loading(username) = &mut model.feed {
                model.feed = Status::LoadingSlowly(take(username))
            }
        }
        Msg::Unreachable => logger::error("Unreachable!"),
    }
}

// View

fn title_for_other(username: &Username) -> String {
    format!("Profile - {}", username.as_str())
}

fn title_for_me(viewer: Option<&Viewer>, username: &Username) -> &'static str {
    if let Some(viewer) = viewer {
        if username == viewer.username() {
            return MY_PROFILE_TITLE;
        }
    }
    DEFAULT_PROFILE
}

fn title<'a>(model: &Model) -> Cow<'a, str> {
    match &model.author {
        Status::Loading(username) | Status::LoadingSlowly(username) | Status::Failed(username) => {
            title_for_me(model.session.viewer(), username).into()
        }
        Status::Loaded(Author::IsViewer(..)) => MY_PROFILE_TITLE.into(),
        Status::Loaded(author) => title_for_other(author.username()).into(),
    }
}

pub fn view<'a>(model: &'a Model) -> ViewPage<'a, Msg> {
    ViewPage::new(title(model), view_content(model))
}

fn view_tabs(selected_feed: SelectedFeed) -> Node<Msg> {
    let my_articles =
        article::feed::Tab::new("My Articles", Msg::TabClicked(SelectedFeed::MyArticles));
    let favorited_articles = article::feed::Tab::new(
        "Favorited Articles",
        Msg::TabClicked(SelectedFeed::FavoritedArticles),
    );

    match selected_feed {
        SelectedFeed::MyArticles => {
            article::feed::view_tabs(vec![my_articles.activate(), favorited_articles])
        }
        SelectedFeed::FavoritedArticles => {
            article::feed::view_tabs(vec![my_articles, favorited_articles.activate()])
        }
    }
}

fn view_feed(model: &Model) -> Node<Msg> {
    match &model.feed {
        Status::Loading(_) => empty![],
        Status::LoadingSlowly(_) => loading::icon(),
        Status::Failed(_) => loading::error("feed"),
        Status::Loaded(feed_model) => div![
            class!["container"],
            div![
                class!["row"],
                div![
                    class!["col-xs-12", "col-md-10", "offset-md-1"],
                    div![
                        class!["articles-toggle"],
                        view_tabs(model.selected_feed),
                        article::feed::view_articles(feed_model)
                            .els()
                            .map_message(Msg::FeedMsg),
                        article::feed::view_pagination(
                            feed_model,
                            model.feed_page,
                            Msg::FeedPageClicked
                        )
                    ],
                ]
            ]
        ],
    }
}

fn view_follow_button(author: &Author, model: &Model) -> Node<Msg> {
    match model.session.viewer() {
        None => empty![],
        Some(_) => match author {
            Author::IsViewer(..) => empty![],
            Author::Following(_) => {
                author::view_unfollow_button(Msg::UnfollowClicked, author.username())
            }
            Author::NotFollowing(_) => {
                author::view_follow_button(Msg::FollowClicked, author.username())
            }
        },
    }
}

fn view_content(model: &Model) -> Node<Msg> {
    match &model.author {
        Status::Loading(_) => empty![],
        Status::LoadingSlowly(_) => loading::icon(),
        Status::Failed(_) => loading::error("profile"),
        Status::Loaded(author) => div![
            class!["profile-page"],
            page::view_errors(Msg::DismissErrorsClicked, &model.errors),
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
                                attrs! {At::Src => author.profile().avatar.src() }
                            ],
                            h4![author.username().to_string()],
                            p![author.profile().bio.as_ref().unwrap_or(&String::new())],
                            view_follow_button(author, model)
                        ]
                    ]
                ]
            ],
            view_feed(model)
        ],
    }
}
