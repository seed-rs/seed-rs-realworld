use seed::{prelude::*, fetch};
use super::ViewPage;
use crate::{session, route, viewer, api, avatar, username, GMsg, login_form, login_fetch};
use futures::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::rc::Rc;

// Model

#[derive(Default)]
pub struct Model {
    session: session::Session,
    problems: Vec<login_form::Problem>,
    form: login_form::Form,
}

impl Model {
    pub fn session(&self) -> &session::Session {
        &self.session
    }
}

impl From<Model> for session::Session {
    fn from(model: Model) -> session::Session {
        model.session
    }
}

// Init

pub fn init<'a, RMsg>(session: session::Session, _: &mut impl OrdersTrait<Msg, GMsg, RMsg>) -> Model {
    Model {
        session,
        ..Model::default()
    }
}

// Global msg handler

pub fn g_msg_handler<RMsg>(g_msg: GMsg, model: &mut Model, orders: &mut impl OrdersTrait<Msg, GMsg, RMsg>) {
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
    SubmittedForm,
    EnteredEmail(String),
    EnteredPassword(String),
    CompletedLogin(Result<viewer::Viewer, Vec<login_form::Problem>>),
}

pub fn update<RMsg>(msg: Msg, model: &mut Model, orders: &mut impl OrdersTrait<Msg, GMsg, RMsg>) {
    match msg {
        Msg::SubmittedForm => {
            match model.form.trim_fields().validate() {
                Ok(valid_form) => {
                    model.problems.clear();
                    orders.perform_cmd(login_fetch::login(&valid_form, Msg::CompletedLogin));
                },
                Err(problems) => {
                    model.problems = problems;
                }
            }
        },
        Msg::EnteredEmail(email) => {
            model.form.user.insert(login_form::Field::Email, email);
        },
        Msg::EnteredPassword(password) => {
            model.form.user.insert(login_form::Field::Password, password);
        },
        Msg::CompletedLogin(Ok(viewer)) => {
            viewer.store();
            orders.send_g_msg(GMsg::SessionChanged(Some(viewer).into()));
        },
        Msg::CompletedLogin(Err(problems)) => {
            model.problems = problems;
        },
    }
}

// View

pub fn view<'a>(model: &Model) -> ViewPage<'a, Msg> {
    ViewPage::new("Login", view_content(model))
}

//fn view_field(fi)

fn view_form(form: &login_form::Form) -> El<Msg> {
    form![
        raw_ev(Ev::Submit, |event| {
            event.prevent_default();
            Msg::SubmittedForm
        }),
        fieldset![
            class!["form-group"],
            input![
                class!["form-control", "form-control-lg"],
                attrs!{
                    At::Type => "text",
                    At::Placeholder => "Email",
                    At::Value => form.user.get(&login_form::Field::Email).unwrap()
                },
                input_ev(Ev::Input, Msg::EnteredEmail),
            ]
        ],
        fieldset![
            class!["form-group"],
            input![
                class!["form-control", "form-control-lg"],
                attrs!{
                    At::Type => "password",
                    At::Placeholder => "Password",
                    At::Value => form.user.get(&login_form::Field::Password).unwrap()
                },
                input_ev(Ev::Input, Msg::EnteredPassword),
            ]
        ],
        button![
            class!["btn", "btn-lg", "btn-primary", "pull-xs-right"],
            "Sign in"
        ]
    ]
}

fn view_problem<'a>(problem: &login_form::Problem) -> El<Msg> {
    li![
        match problem {
            login_form::Problem::InvalidEntry(_, error) => error,
            login_form::Problem::ServerError(error) => error,
        }
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