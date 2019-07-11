use seed::{prelude::*, fetch};
use super::ViewPage;
use crate::{session, route, viewer, api, avatar, username, GMsg, form::settings as form, request, loading};
use serde::{Deserialize, Serialize};
use serde_json;
use std::rc::Rc;

// Model

#[derive(Default)]
pub struct Model {
    session: session::Session,
    problems: Vec<form::Problem>,
    status: Status,
}

enum Status {
    Loading,
    LoadingSlowly,
    Loaded(form::Form),
    Failed
}

impl Default for Status {
    fn default() -> Self {
        Status::Loading
    }
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

pub fn init<'a>(session: session::Session, orders: &mut impl Orders<Msg, GMsg>) -> Model {
    orders
        .perform_cmd(loading::slow_threshold(Msg::SlowLoadThresholdPassed, Msg::NoOp))
        .perform_cmd(request::settings_load::load_settings(&session, Msg::FormLoadCompleted));
    Model {
        session,
        ..Model::default()
    }
}

// Global msg handler

pub fn g_msg_handler<'a>(g_msg: GMsg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
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
    FormSubmitted,
    FieldChanged(form::Field),
    FormLoadCompleted(Result<form::Form, Vec<form::Problem>>),
    SaveCompleted(Result<viewer::Viewer, Vec<form::Problem>>),
    SlowLoadThresholdPassed,
    NoOp,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::FormSubmitted => {
            if let Status::Loaded(form) = &model.status {
                match form.trim_fields().validate() {
                    Ok(valid_form) => {
                        model.problems.clear();
                        orders
                            .perform_cmd(
                                request::settings_update::update_settings(
                                    &model.session,
                                    &valid_form,
                                    Msg::SaveCompleted
                                )
                            );
                    },
                    Err(problems) => {
                        model.problems = problems;
                    }
                }
            }
        }
        Msg::FieldChanged(field) => {
            if let Status::Loaded(form) = &mut model.status {
                form.upsert_field(field);
            }
        }
        Msg::FormLoadCompleted(Ok(form)) => {
            model.status = Status::Loaded(form);
        }
        Msg::FormLoadCompleted(Err(problems)) => {
            model.problems = problems;
            model.status = Status::Failed;
        }
        Msg::SaveCompleted(Ok(viewer)) => {
            viewer.store();
            orders.send_g_msg(GMsg::SessionChanged(Some(viewer).into()));
        },
        Msg::SaveCompleted(Err(problems)) => {
            model.problems = problems;
        },
        Msg::SlowLoadThresholdPassed => {
            if let Status::Loading = model.status {
                model.status = Status::LoadingSlowly
            }
        }
        Msg::NoOp => { orders.skip(); },
    }
}

// View

pub fn view<'a>(model: &Model) -> ViewPage<'a, Msg> {
    ViewPage::new("Settings", view_content(model))
}

fn view_fieldset(field: &form::Field) -> Node<Msg> {
    match field {
        form::Field::Avatar(value) => {
            fieldset![
                class!["form-group"],
                input![
                    class!["form-control"],
                    attrs!{
                        At::Type => "text",
                        At::Placeholder => "URL of profile picture",
                        At::Value => value
                    },
                    input_ev(Ev::Input, |new_value| Msg::FieldChanged(
                        form::Field::Avatar(new_value)
                    )),
                ]
            ]
        }
        form::Field::Username(value) => {
            fieldset![
                class!["form-group"],
                input![
                    class!["form-control", "form-control-lg"],
                    attrs!{
                        At::Type => "text",
                        At::Placeholder => "Your Name",
                        At::Value => value
                    },
                    input_ev(Ev::Input, |new_value| Msg::FieldChanged(
                        form::Field::Username(new_value)
                    )),
                ]
            ]
        }
        form::Field::Bio(value) => {
            fieldset![
                class!["form-group"],
                textarea![
                    class!["form-control", "form-control-lg"],
                    attrs!{
                        At::Rows => 8,
                        At::Placeholder => "Short bio about you",
                    },
                    value,
                    input_ev(Ev::Input, |new_value| Msg::FieldChanged(
                        form::Field::Bio(new_value)
                    )),
                ]
            ]
        }
        form::Field::Email(value) => {
            fieldset![
                class!["form-group"],
                input![
                    class!["form-control", "form-control-lg"],
                    attrs!{
                        At::Type => "text",
                        At::Placeholder => "Email",
                        At::Value => value
                    },
                    input_ev(Ev::Input, |new_value| Msg::FieldChanged(
                        form::Field::Email(new_value)
                    )),
                ]
            ]
        }
        form::Field::Password(value) => {
            fieldset![
                class!["form-group"],
                input![
                    class!["form-control", "form-control-lg"],
                    attrs!{
                        At::Type => "password",
                        At::Placeholder => "Password",
                        At::Value => value
                    },
                    input_ev(Ev::Input, |new_value| Msg::FieldChanged(
                        form::Field::Password(new_value)
                    )),
                ]
            ]
        }
    }
}

fn view_form<'a>(model: &Model, credentials: &api::Credentials) -> Node<Msg> {
    match &model.status {
        Status::Loading => empty![],
        Status::LoadingSlowly => loading::icon(),
        Status::Loaded(form) => {
            form![
                raw_ev(Ev::Submit, |event| {
                    event.prevent_default();
                    Msg::FormSubmitted
                }),
                form.iter().map(view_fieldset),
                button![
                    class!["btn", "btn-lg", "btn-primary", "pull-xs-right"],
                    "Update Settings"
                ]
            ]
        },
        Status::Failed => loading::error("page")
    }
}

fn view_content<'a>(model: &Model) -> Node<Msg> {
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
                        "Your Settings"
                    ],

                    if let Some(viewer) = model.session().viewer() {
                        vec![
                            ul![
                                class!["error-messages"],
                                model.problems.iter().map(|problem| li![
                                    problem.message()
                                ])
                            ],
                            view_form(model, &viewer.credentials),
                        ]
                    } else {
                        vec![
                            div![
                                "Sign in to view your settings."
                            ]
                        ]
                    }
                ]

            ]
        ]
    ]
}