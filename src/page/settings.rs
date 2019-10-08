use super::ViewPage;
use crate::entity::{
    form::settings::{Field, Form, Problem},
    Viewer,
};
use crate::{
    loading, logger, request,
    route::{self, Route},
    GMsg, Session,
};
use seed::prelude::*;

// ------ ------
//     Model
// ------ ------

// ------ Model ------

#[derive(Default)]
pub struct Model {
    session: Session,
    problems: Vec<Problem>,
    status: Status,
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

// ------ Status ------

enum Status {
    Loading,
    LoadingSlowly,
    Loaded(Form),
    Failed,
}

impl Default for Status {
    fn default() -> Self {
        Self::Loading
    }
}

// ------ ------
//     Init
// ------ ------

pub fn init(session: Session, orders: &mut impl Orders<Msg, GMsg>) -> Model {
    orders
        .perform_cmd(loading::notify_on_slow_load(
            Msg::SlowLoadThresholdPassed,
            Msg::Unreachable,
        ))
        .perform_cmd(request::settings::load(
            session.viewer(),
            Msg::FormLoadCompleted,
        ));
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
    FormLoadCompleted(Result<Form, Vec<Problem>>),
    SaveCompleted(Result<Viewer, Vec<Problem>>),
    SlowLoadThresholdPassed,
    Unreachable,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::FormSubmitted => {
            if let Status::Loaded(form) = &model.status {
                match form.trim_fields().validate() {
                    Ok(valid_form) => {
                        model.problems.clear();
                        orders.perform_cmd(request::settings::update(
                            model.session.viewer(),
                            &valid_form,
                            Msg::SaveCompleted,
                        ));
                    }
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
            orders.send_g_msg(GMsg::SessionChanged(Session::LoggedIn(viewer)));
        }
        Msg::SaveCompleted(Err(problems)) => {
            model.problems = problems;
        }
        Msg::SlowLoadThresholdPassed => {
            if let Status::Loading = model.status {
                model.status = Status::LoadingSlowly
            }
        }
        Msg::Unreachable => logger::error("Unreachable!"),
    }
}

// ------ ------
//     View
// ------ ------

pub fn view<'a>(model: &Model) -> ViewPage<'a, Msg> {
    ViewPage::new("Settings", view_content(model))
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
                    h1![class!["text-xs-center"], "Your Settings"],
                    if model.session.viewer().is_some() {
                        vec![
                            ul![
                                class!["error-messages"],
                                model.problems.iter().map(|problem| li![problem.message()])
                            ],
                            view_form(model),
                        ]
                    } else {
                        vec![div!["Sign in to view your settings."]]
                    }
                ]
            ]
        ]
    ]
}

// ------ view form ------

fn view_form(model: &Model) -> Node<Msg> {
    match &model.status {
        Status::Loading => empty![],
        Status::LoadingSlowly => loading::view_icon(),
        Status::Loaded(form) => form![
            raw_ev(Ev::Submit, |event| {
                event.prevent_default();
                Msg::FormSubmitted
            }),
            form.iter_fields().map(view_fieldset),
            button![
                class!["btn", "btn-lg", "btn-primary", "pull-xs-right"],
                "Update Settings"
            ]
        ],
        Status::Failed => loading::view_error("page"),
    }
}

fn view_fieldset(field: &Field) -> Node<Msg> {
    match field {
        Field::Avatar(value) => fieldset![
            class!["form-group"],
            input![
                class!["form-control"],
                attrs! {
                    At::Type => "text",
                    At::Placeholder => "URL of profile picture",
                    At::Value => value
                },
                input_ev(Ev::Input, |new_value| Msg::FieldChanged(Field::Avatar(
                    new_value
                ))),
            ]
        ],
        Field::Username(value) => fieldset![
            class!["form-group"],
            input![
                class!["form-control", "form-control-lg"],
                attrs! {
                    At::Type => "text",
                    At::Placeholder => "Your Name",
                    At::Value => value
                },
                input_ev(Ev::Input, |new_value| Msg::FieldChanged(Field::Username(
                    new_value
                ))),
            ]
        ],
        Field::Bio(value) => fieldset![
            class!["form-group"],
            textarea![
                class!["form-control", "form-control-lg"],
                attrs! {
                    At::Rows => 8,
                    At::Placeholder => "Short bio about you",
                },
                value,
                input_ev(Ev::Input, |new_value| Msg::FieldChanged(Field::Bio(
                    new_value
                ))),
            ]
        ],
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
