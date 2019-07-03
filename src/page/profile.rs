use seed::prelude::*;
use super::ViewPage;
use crate::{session, username, GMsg, route, article, author, api, loading, request, paginated_list};
use core::borrow::BorrowMut;

// Model

#[derive(Default)]
pub struct Model<'a> {
    session: session::Session,
    time_zone: String,
    errors: Vec<String>,
    feed_tab: FeedTab,
    feed_page: PageNumber,
    author: Status<'a, author::Author<'a>>,
    feed: Status<'a, article::feed::Model>
}

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

pub struct PageNumber(usize);

impl PageNumber {
    pub fn to_usize(&self) -> usize {
        self.0
    }
}

impl Default for PageNumber {
    fn default() -> Self {
        PageNumber(1)
    }
}

pub fn init<'a>(session: session::Session, username: &username::Username<'a>, orders: &mut impl Orders<Msg, GMsg>
) -> Model<'a> {
    let static_username: username::Username<'static> = username.as_str().to_owned().into();
    orders
        .perform_cmd(loading::slow_threshold(Msg::SlowLoadThresholdPassed, Msg::NoOp))
        // @TODO TimeZoneLoaded?
        .perform_cmd(request::author_load::load_author(session.clone(), static_username.clone(), Msg::AuthorLoadCompleted))
        .perform_cmd(request::feed_load::load_feed(
            session.clone(),
            static_username.clone(),
            FeedTab::default(),
            PageNumber::default(),
            Msg::FeedLoadCompleted,
        ));

    Model {
        session,
        author: Status::Loading(username.clone()),
        feed: Status::Loading(username.clone()),
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
    DismissErrorsClicked,
    FollowClicked(api::Credentials, author::UnfollowedAuthor<'static>),
    UnfollowClicked(api::Credentials, author::FollowedAuthor<'static>),
    TabClicked(FeedTab),
    FeedPageClicked(PageNumber),
    FollowChangeCompleted(Result<author::Author<'static>, Vec<String>>),
    AuthorLoadCompleted(Result<author::Author<'static>, (username::Username<'static>, Vec<String>)>),
    FeedLoadCompleted(
        Result<paginated_list::PaginatedList<article::Article>,
        (username::Username<'static>, Vec<String>)>
    ),
    TimeZoneLoaded(String),
    SlowLoadThresholdPassed,
    NoOp,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
}

// View

pub fn view<'a>(model: &Model) -> ViewPage<'a, Msg> {
    ViewPage::new("@TODO", view_content())
}

fn view_content() -> El<Msg> {
    div![
        class!["profile-page"],

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
                            attrs!{At::Src => "http://i.imgur.com/Qr71crq.jpg"}
                        ],
                        p![
                            "Cofounder @GoThinkster, lived in Aol's HQ for a few months, kinda looks like Peeta from the Hunger Games"
                        ],
                        button![
                            class!["btn", "btn-sm", "btn-outline-secondary", "action-btn"],
                            i![
                                class!["ion-plus-round"]
                            ],
                            raw!("&nbsp;"),
                            "Follow Eric Simons"
                        ]
                    ]

                ]
            ]
        ],

        div![
            class!["container"],
            div![
                class!["row"],
                div![
                    class!["col-xs-12", "col-md-10", "offset-md-1"],
                    div![
                        class!["articles-toggle"],
                        ul![
                            class!["nav", "nav-pills", "outline-active"],
                            li![
                                class!["nav-item"],
                                a![
                                    class!["nav-link", "active"],
                                    attrs!{At::Href => ""},
                                    "My Articles"
                                ]
                            ],
                            li![
                                class!["nav-item"],
                                a![
                                    class!["nav-link"],
                                    attrs!{At::Href => ""},
                                    "Favorited Articles"
                                ]
                            ]
                        ]
                    ],

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
                    ],

                    div![
                        class!["article-preview"],
                        div![
                            class!["article-meta"],
                            a![
                                attrs!{At::Href => "/profile"},
                                img![
                                    attrs!{At::Src => "http://i.imgur.com/N4VcUeJ.jpg"}
                                ]
                            ],
                            div![
                                class!["info"],
                                a![
                                    class!["author"],
                                    attrs!{At::Href => ""},
                                    "Albert Pai"
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
                                    " 32"
                                ]
                            ]
                        ],
                        a![
                            class!["preview-link"],
                            attrs!{At::Href => ""},
                            h1![
                                "The song you won't ever stop singing. No matter how hard you try."
                            ],
                            p![
                                "This is the description for the post."
                            ],
                            span![
                                "Read more..."
                            ]
                        ]
                    ]

                ]
            ]
        ]

    ]
}