use seed::{prelude::*, fetch};
use super::ViewPage;
use crate::{
    article,
    session,
    route,
    viewer,
    api,
    avatar,
    username,
    GMsg,
    form::article_editor as form,
};
use serde::{Deserialize, Serialize};
use serde_json;
use std::rc::Rc;

// Model

#[derive(Default)]
pub struct Model {
    session: session::Session,
    problems: Vec<form::Problem>,
    form: form::Form,
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

pub fn init_new(session: session::Session, _: &mut impl Orders<Msg, GMsg>) -> Model {
    Model {
        session,
        ..Model::default()
    }
}

pub fn init_edit(
    session: session::Session,
    slug: &article::slug::Slug,
    _: &mut impl Orders<Msg, GMsg>,
) -> Model {
    Model {
        session,
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
    SubmittedForm,
    FieldChanged(form::Field),
    CompletedLogin(Result<viewer::Viewer, Vec<form::Problem>>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::SubmittedForm => {
//            match model.form.trim_fields().validate() {
//                Ok(valid_form) => {
//                    model.problems.clear();
//                    orders.perform_cmd(login_fetch::login(&valid_form, Msg::CompletedLogin));
//                },
//                Err(problems) => {
//                    model.problems = problems;
//                }
//            }
        },
        Msg::FieldChanged(field) => {
//            model.form.upsert_field(field);
        }
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
    ViewPage::new("@Todo", view_content(model))
}

fn view_fieldset(field: &form::Field) -> El<Msg> {
    match field {
        form::Field::Title(value) => {
            fieldset![
                class!["form-group"],
                input![
                    class!["form-control", "form-control-lg"],
                    attrs!{
                        At::Type => "text",
                        At::Placeholder => "Article Title",
                        At::Value => value
                    },
                    input_ev(Ev::Input, |new_value| Msg::FieldChanged(
                        form::Field::Title(new_value)
                    )),
                ]
            ]
        }
        form::Field::Description(value) => {
            fieldset![
                class!["form-group"],
                input![
                    class!["form-control"],
                    attrs!{
                        At::Type => "text",
                        At::Placeholder => "What's this article about?",
                        At::Value => value
                    },
                    input_ev(Ev::Input, |new_value| Msg::FieldChanged(
                        form::Field::Description(new_value)
                    )),
                ]
            ]
        }
        form::Field::Body(value) => {
            fieldset![
                class!["form-group"],
                textarea![
                    class!["form-control"],
                    attrs!{
                        At::Rows => 8,
                        At::Placeholder => "Write your article (in markdown)",
                    },
                    value,
                    input_ev(Ev::Input, |new_value| Msg::FieldChanged(
                        form::Field::Body(new_value)
                    )),
                ]
            ]
        }
        form::Field::Tags(value) => {
            fieldset![
                class!["form-group"],
                input![
                    class!["form-control"],
                    attrs!{
                        At::Type => "text",
                        At::Placeholder => "Enter tags",
                        At::Value => value
                    },
                    input_ev(Ev::Input, |new_value| Msg::FieldChanged(
                        form::Field::Tags(new_value)
                    )),
                ],
                div![
                    class!["tag-list"]
                ]
            ]
        }
    }
}

fn view_form(form: &form::Form) -> El<Msg> {
    form![
        raw_ev(Ev::Submit, |event| {
            event.prevent_default();
            Msg::SubmittedForm
        }),
        form.iter().map(view_fieldset),
        button![
            class!["btn", "btn-lg", "btn-primary", "pull-xs-right"],
            attrs!{At::Type => "button"},
            "Publish Article"
        ]
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

                    ul![
                        class!["error-messages"],
                        model.problems.iter().map(|problem| li![
                            problem.message()
                        ])
                    ],

                    view_form(&model.form)
                ]

            ]
        ]
    ]
}