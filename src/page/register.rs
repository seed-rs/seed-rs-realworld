use seed::prelude::*;
use super::{ViewPage, InitPage};
use crate::{session, GMsg, route, HasSessionChangedOnInit};

// Model

pub struct Model {
    session: session::Session
}

impl<'a> Model {
    pub fn session(&self) -> &session::Session {
        &self.session
    }
}

impl From<Model> for session::Session {
    fn from(model: Model) -> session::Session {
        model.session
    }
}

pub fn init(session: session::Session) -> InitPage<Model, Msg> {
    InitPage::new(Model { session })
}

// Global msg handler

pub fn g_msg_handler(g_msg: GMsg, model: &mut Model, orders: &mut Orders<Msg, GMsg>) {
    match g_msg {
        GMsg::SessionChanged(session, on_init) => {
            model.session = session;
            if !on_init {
                route::go_to(route::Route::Home, orders);
            }
        }
        _ => ()
    }
}

// Update

pub enum Msg {
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg, GMsg>) {
}

// View

pub fn view<'a>(model: &Model) -> ViewPage<'a, Msg> {
    ViewPage::new("Register",view_content())
}

fn view_content() -> El<Msg> {
    div![
        class!["auth-page"],
        div![
            class!["container", "page"],
            div![
                class!["row"],

                div![
                    class!["col-md-6", "offset-md-3", "col-x32-12"],
                    h1![
                        class!["text-xs-center"],
                        "Sign up"
                    ],
                    p![
                        class!["text-xs-center"],
                        a![
                            attrs!{At::Href => ""},
                            "Have an account?"
                        ]
                    ],

                    ul![
                        class!["error-messages"],
                        li![
                            "That email is already taken"
                        ]
                    ],

                    form![
                        fieldset![
                            class!["form-group"],
                            input![
                                class!["form-control", "form-control-lg"],
                                attrs!{At::Type => "text"; At::Placeholder => "Your Name"}
                            ]
                        ],
                        fieldset![
                            class!["form-group"],
                            input![
                                class!["form-control", "form-control-lg"],
                                attrs!{At::Type => "text"; At::Placeholder => "Email"}
                            ]
                        ],
                        fieldset![
                            class!["form-group"],
                            input![
                                class!["form-control", "form-control-lg"],
                                attrs!{At::Type => "password"; At::Placeholder => "Password"}
                            ]
                        ],
                        button![
                            class!["btn", "btn-lg", "btn-primary", "pull-xs-right"],
                            "Sign up"
                        ]
                    ]
                ]

            ]
        ]
    ]
}