use seed::{prelude::*, fetch};
use super::{ViewPage, InitPage};
use crate::{session, route, viewer, api};
use indexmap::IndexMap;
use futures::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
use serde::de::Unexpected::Str;

// Model

#[derive(Hash, Eq, PartialEq, Copy, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
enum Field {
    Email,
    Password
}

impl Field {
    fn validate(&self, value: &str) -> Option<Problem> {
        match self {
            Field::Email => {
                if value.is_empty() {
                    Some(Problem::InvalidEntry(*self, "email can't be blank.".into()))
                } else {
                    None
                }
            },
            Field::Password => {
                if value.is_empty() {
                    Some(Problem::InvalidEntry(*self, "password can't be blank.".into()))
                } else {
                    None
                }
            }
        }
    }
}

enum Problem {
    InvalidEntry(Field, String),
    ServerError(String)
}


struct Form {
    fields: IndexMap<Field, String>
}

impl Default for Form {
    fn default() -> Self {
        Self {
            fields: vec![
                (Field::Email, "".to_string()),
                (Field::Password, "".to_string()),
            ].into_iter().collect()
        }
    }
}


impl Form {
    fn trim_fields(&self) -> TrimmedForm {
        TrimmedForm {
            fields:
                self
                    .fields
                    .iter()
                    .map(|(field, value)|(field,value.trim()))
                    .collect()
        }
    }
}

struct TrimmedForm<'a> {
    fields: IndexMap<&'a Field, &'a str>
}

impl<'a> TrimmedForm<'a> {
    fn validate(&'a self) -> Result<ValidForm, Vec<Problem>> {
        let invalid_entries =
            self
                .fields
                .iter()
                .filter_map(|(field,value)| {
                    field.validate(value)
                })
                .collect::<Vec<Problem>>();

        if invalid_entries.is_empty() {
            Ok(ValidForm {
                fields:
                self.
                    fields
                    .iter()
                    .map(|(field, value)| (**field, (*value).to_owned()))
                    .collect()
            })
        } else {
            Err(invalid_entries)
        }
    }
}

#[derive(Serialize)]
struct ValidForm {
    fields: IndexMap<Field, String>
}

#[derive(Default)]
pub struct Model {
    session: session::Session,
    problems: Vec<Problem>,
    form: Form,
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

pub fn init<'a>(session: session::Session) -> InitPage<Model, Msg> {
    InitPage::new(Model {
        session,
        ..Model::default()
    })
}

#[derive(Deserialize)]
struct ServerErrorData {
    errors: IndexMap<String, Vec<String>>
}

// Update

#[derive(Clone)]
pub enum Msg {
    SubmittedForm,
    EnteredEmail(String),
    EnteredPassword(String),
    CompletedLogin(fetch::FetchResult<String>),
    GotSession(session::Session),
}

fn login(valid_form: &ValidForm) -> impl Future<Item=Msg, Error=Msg>  {
    fetch::Request::new("https://conduit.productionready.io/api/users/login".into())
        .method(fetch::Method::Post)
        .timeout(5000)
        .send_json(valid_form)
        .fetch_string(|fetch_object| {
            Msg::CompletedLogin(fetch_object.result)
        })
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
    match msg {
        Msg::SubmittedForm => {
            match model.form.trim_fields().validate() {
                Ok(valid_form) => {
                    model.problems.clear();
                    orders.perform_cmd(login(&valid_form));
                },
                Err(problems) => {
                    model.problems = problems;
                }
            }
        },
        Msg::EnteredEmail(email) => {
            model.form.fields.insert(Field::Email, email);
        },
        Msg::EnteredPassword(password) => {
            model.form.fields.insert(Field::Password, password);
        },
        Msg::CompletedLogin(Ok(response)) => {
            match response.status.category {
                fetch::StatusCategory::Success => {
                    let viewer =
                        response
                            .data
                            .map_err(|_|())
                            .and_then(|string| {
                                serde_json::from_str::<viewer::Viewer>(string.as_str())
                                    .map_err(|_|())
                            });

                    match viewer {
                        Ok(viewer) => {
                            //
                        },
                        Err(_) => {
                            model.problems.push(Problem::ServerError("Data error".into()))
                        }
                    }
                },
                _ => {
                    let error_messages: Result<Vec<String>, ()> =
                        response
                            .data
                            .map_err(|_|())
                            .and_then(|string| {
                                serde_json::from_str::<ServerErrorData>(string.as_str())
                                    .map_err(|_|())
                            }).and_then(|server_error_data| {
                                Ok(server_error_data.errors.into_iter().map(|(field, errors)| {
                                    format!("{} {}", field, errors.join(", "))
                                }).collect())
                            });
                    match error_messages {
                        Ok(error_messages) => {
                            let mut new_problems = error_messages
                                .into_iter()
                                .map(|message| {
                                    Problem::ServerError(message)
                                }).collect();
                            model.problems.append(&mut new_problems);
                        },
                        Err(_) => {
                            model.problems.push(Problem::ServerError("Server error".into()))
                        }
                    }
                }
            }
        },
        Msg::CompletedLogin(Err(request_error)) => {
            model.problems.push(Problem::ServerError("Request error".into()));
        }
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
                    At::Value => form.fields.get(&Field::Email).unwrap()
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
                    At::Value => form.fields.get(&Field::Password).unwrap()
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

fn view_problem<'a>(problem: &Problem) -> El<Msg> {
    li![
        match problem {
            Problem::InvalidEntry(_, error) => error,
            Problem::ServerError(error) => error,
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