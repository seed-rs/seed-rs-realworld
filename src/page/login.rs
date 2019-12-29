use super::ViewPage;
use crate::{
    entity::{
        form::login::{Field, Form, Problem},
        Viewer,
    },
    request,
    route::{self, Route},
    GMsg, Session,
};
use seed::prelude::*;

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
pub struct Model {
    session: Session,
    problems: Vec<Problem>,
    form: Form,
}

impl Model {
    pub const fn session(&self) -> &Session {
        &self.session
    }
}

impl From<Model> for Session {
    fn from(model: Model) -> Self {
        model.session
    }
}

// ------ ------
//     Init
// ------ ------

pub fn init(session: Session) -> Model {
    Model {
        session,
        ..Model::default()
    }
}

// ------ ------
//     Sink
// ------ ------

pub fn sink(g_msg: GMsg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match g_msg {
        GMsg::SessionChanged(session) => {
            model.session = session;
            route::go_to(Route::Home, orders);
        }
        _ => (),
    }
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    FormSubmitted,
    FieldChanged(Field),
    LoginCompleted(Result<Viewer, Vec<Problem>>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::FormSubmitted => match model.form.trim_fields().validate() {
            Ok(valid_form) => {
                model.problems.clear();
                orders.perform_cmd(request::login::login(&valid_form, Msg::LoginCompleted));
            }
            Err(problems) => {
                model.problems = problems;
            }
        },
        Msg::FieldChanged(field) => {
            model.form.upsert_field(field);
        }
        Msg::LoginCompleted(Ok(viewer)) => {
            viewer.store();
            orders.send_g_msg(GMsg::SessionChanged(Session::LoggedIn(viewer)));
        }
        Msg::LoginCompleted(Err(problems)) => {
            model.problems = problems;
        }
    }
}

// ------ ------
//     View
// ------ ------

pub fn view<'a>(model: &Model) -> ViewPage<'a, Msg> {
    ViewPage::new("Login", view_content(model))
}

// ====== PRIVATE ======

fn view_content(model: &Model) -> Node<Msg> {
    div![
        class!["auth-page"],
        div![
            class!["container", "page"],
            div![
                class!["row"],
                div![
                    class!["col-md-6", "offset-md-3", "col-x32-12"],
                    h1![class!["text-xs-center"], "Sign in"],
                    p![
                        class!["text-xs-center"],
                        a![
                            attrs! {At::Href => Route::Register.to_string()},
                            "Need an account?"
                        ]
                    ],
                    ul![
                        class!["error-messages"],
                        model.problems.iter().map(|problem| li![problem.message()])
                    ],
                    view_form(&model.form)
                ]
            ]
        ]
    ]
}

// ------ view form ------

fn view_form(form: &Form) -> Node<Msg> {
    form![
        raw_ev(Ev::Submit, |event| {
            event.prevent_default();
            Msg::FormSubmitted
        }),
        form.iter_fields().map(view_fieldset),
        button![
            class!["btn", "btn-lg", "btn-primary", "pull-xs-right"],
            "Sign in"
        ]
    ]
}

fn view_fieldset(field: &Field) -> Node<Msg> {
    match field {
        Field::Email(value) => fieldset![
            class!["form-group"],
            input![
                class!["form-control", "form-control-lg"],
                attrs! {
                    At::Type => "text",
                    At::Placeholder => "Email",
                    At::Value => value
                },
                input_ev(Ev::Input, |new_value| Msg::FieldChanged(Field::Email(
                    new_value
                ))),
            ]
        ],
        Field::Password(value) => fieldset![
            class!["form-group"],
            input![
                class!["form-control", "form-control-lg"],
                attrs! {
                    At::Type => "password",
                    At::Placeholder => "Password",
                    At::Value => value
                },
                input_ev(Ev::Input, |new_value| Msg::FieldChanged(Field::Password(
                    new_value
                ))),
            ]
        ],
    }
}
