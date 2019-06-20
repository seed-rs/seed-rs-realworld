use seed::prelude::*;
use super::{ViewPage, InitPage};
use crate::{session, route, viewer};

// Model

enum ValidatedField {
    Email,
    Password,
}

enum Problem {
    InvalidEntry(ValidatedField, String),
    ServerError(String)
}

#[derive(Default)]
struct Form {
    email: String,
    password: String
}

#[derive(Default)]
pub struct Model<'a> {
    session: session::Session<'a>,
    problems: Vec<Problem>,
    form: Form,
}

impl<'a> Model<'a> {
    pub fn session(&self) -> &session::Session {
        &self.session
    }
}

impl<'a> From<Model<'a>> for session::Session<'a> {
    fn from(model: Model<'a>) -> session::Session<'a> {
        model.session
    }
}

pub fn init(session: session::Session) -> InitPage<Model, Msg> {
    InitPage::new(Model {
        session,
        ..Model::default()
    })
}

// Update

pub enum Msg {
    SubmittedForm,
    EnteredEmail(String),
    EnteredPassword(String),
    CompletedLogin(seed::fetch::ResponseDataResult<viewer::Viewer<'static>>),
    GotSession(session::Session<'static>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
    match msg {
        Msg::SubmittedForm => {},
        Msg::EnteredEmail(email) => {
            model.form.email = email;
        },
        Msg::EnteredPassword(password) => {
            model.form.password = password;
        },
        Msg::CompletedLogin(Ok(viewer)) => {},
        Msg::CompletedLogin(Err(fail_reason)) => {}
        Msg::GotSession(session) => {
            model.session = session;
            route::replace_url(route::Route::Home)
        }
    }
}

// View

pub fn view<'a>(model: &Model) -> ViewPage<'a, Msg> {
    ViewPage::new("Login", view_content(model))
}

fn view_form(form: &Form) -> El<Msg> {
    form![
        fieldset![
            class!["form-group"],
            input![
                class!["form-control", "form-control-lg"],
                attrs!{At::Type => "text"; At::Placeholder => "Email"},
                input_ev(Ev::Input, Msg::EnteredEmail),
            ]
        ],
        fieldset![
            class!["form-group"],
            input![
                class!["form-control", "form-control-lg"],
                attrs!{At::Type => "password"; At::Placeholder => "Password"},
                input_ev(Ev::Input, Msg::EnteredPassword),
            ]
        ],
        button![
            class!["btn", "btn-lg", "btn-primary", "pull-xs-right"],
            "Sign in"
        ]
    ]
}

fn view_problem<'a>(problem: &Problem) -> El<Msg> {
    li![
        "That email is already taken"
    ]
}

fn view_content<'a>(model: &Model) -> El<Msg> {
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
                        "Sign in"
                    ],
                    p![
                        class!["text-xs-center"],
                        a![
                            attrs!{At::Href => route::Route::Register.to_string()},
                            "Need an account?"
                        ]
                    ],

                    ul![
                        class!["error-messages"],
                        model.problems.iter().map(view_problem)
                    ],

                    view_form(&model.form)
                ]

            ]
        ]
    ]
}