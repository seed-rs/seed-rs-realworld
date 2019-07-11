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
    loading,
    request
};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{rc::Rc, borrow::Cow, mem};

// Model

#[derive(Default)]
pub struct Model {
    session: session::Session,
    status: Status
}

type Slug = article::slug::Slug;

enum Status {
    Placeholder,
    // -- edit article --
    Loading(Slug),
    LoadingSlowly(Slug),
    LoadingFailed(Slug, Vec<form::Problem>),
    Saving(Slug, form::Form),
    Editing(Slug, Vec<form::Problem>, form::Form),
    // -- new article --
    EditingNew(Vec<form::Problem>, form::Form),
    Creating(form::Form),
}

impl Status {
    fn take(&mut self) -> Self {
        mem::replace(self, Status::Placeholder)
    }
    fn slug(&self) -> Option<&Slug> {
        match self {
            Status::Loading(slug) => Some(slug),
            Status::LoadingSlowly(slug) => Some(slug),
            Status::LoadingFailed(slug, ..) => Some(slug),
            Status::Saving(slug, ..) => Some(slug),
            Status::Editing(slug, ..) => Some(slug),
            Status::EditingNew(..) | Status::Creating(..) | Status::Placeholder => None,
        }
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::EditingNew(Vec::default(), form::Form::default())
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

pub fn init_new(session: session::Session, _: &mut impl Orders<Msg, GMsg>) -> Model {
    Model {
        session,
        ..Model::default()
    }
}

pub fn init_edit(
    session: session::Session,
    slug: &article::slug::Slug,
    orders: &mut impl Orders<Msg, GMsg>,
) -> Model {
    orders
        .perform_cmd(loading::slow_threshold(Msg::SlowLoadThresholdPassed, Msg::NoOp))
        .perform_cmd(request::article_load::load_article(&session, slug, Msg::ArticleLoadCompleted));
    Model {
        session,
        status: Status::Loading(slug.clone())
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

#[derive(Clone)]
pub enum Msg {
    FieldChanged(form::Field),
    FormSubmitted,
    CreateCompleted(Result<article::Article, Vec<form::Problem>>),
    EditCompleted(Result<article::Article, Vec<form::Problem>>),
    ArticleLoadCompleted(Result<article::Article, (article::slug::Slug, Vec<form::Problem>)>),
    SlowLoadThresholdPassed,
    NoOp,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::FieldChanged(field) => {
            match &mut model.status {
                Status::Editing(_, _, form) => {
                    form.upsert_field(field);
                },
                Status::EditingNew(_, form) => {
                    form.upsert_field(field);
                },
                _ => error!("Can't edit the form, status has to be Editing or EditingNew!")
            }
        }
        Msg::FormSubmitted => {
            match model.status.take() {
                Status::Editing(slug, _, form) => {
                    match form.trim_fields().validate() {
                        Ok(valid_form) => {
                            orders.perform_cmd(
                                request::article_update::update_article(
                                    &model.session, &valid_form, &slug, Msg::EditCompleted
                                )
                            );
                            model.status = Status::Saving(slug, form);
                        },
                        Err(problems) => {
                            model.status = Status::Editing(slug, problems, form);
                        }
                    }
                },
                Status::EditingNew(problems, form) => {
                    match form.trim_fields().validate() {
                        Ok(valid_form) => {
                            orders.perform_cmd(
                                request::article_create::create_article(
                                    &model.session, &valid_form, Msg::CreateCompleted
                                )
                            );
                            model.status = Status::Creating(form);
                        },
                        Err(problems) => {
                            model.status = Status::EditingNew(problems, form);
                        }
                    }
                },
                status@ _ => {
                    model.status = status;
                    error!("Can't save the form, status has to be Editing or EditingNew!")
                }
            }
        },
        Msg::CreateCompleted(Ok(article)) => {
            route::go_to(route::Route::Article(article.slug), orders)
        },
        Msg::CreateCompleted(Err(problems)) => {
            match model.status.take() {
                Status::Creating(form) => {
                    model.status = Status::EditingNew(problems, form)
                },
                status @ _ => model.status = status
            }
        },
        Msg::EditCompleted(Ok(article)) => {
            route::go_to(route::Route::Article(article.slug), orders)
        },
        Msg::EditCompleted(Err(problems)) => {
            match model.status.take() {
                Status::Saving(slug, form) => {
                    model.status = Status::Editing(slug, problems, form)
                },
                status @ _ => model.status = status
            }
        },
        Msg::ArticleLoadCompleted(Ok(article)) => {
            model.status = Status::Editing(article.slug.clone(), vec![], article.into_form());
        },
        Msg::ArticleLoadCompleted(Err((slug, problems))) => {
            model.status = Status::LoadingFailed(slug, problems)
        },
        Msg::SlowLoadThresholdPassed => {
            match model.status.take() {
                Status::Loading(slug) => {
                    model.status = Status::LoadingSlowly(slug);
                }
                status @ _ => model.status = status
            }
        },
        Msg::NoOp => { panic!("NoOp!") },
    }
}

// View

pub fn view<'a>(model: &Model) -> ViewPage<'a, Msg> {
    let title: Cow<str> = match model.status.slug() {
        Some(slug) => {
            format!("Edit Article - {}", slug.as_str()).into()
        }
        None => "New Article".into()
    };
    ViewPage::new(title, view_content(model))
}

fn view_fieldset(field: &form::Field) -> Node<Msg> {
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

fn view_form(form: &form::Form, save_button: Node<Msg>) -> Node<Msg> {
    form![
        raw_ev(Ev::Submit, |event| {
            event.prevent_default();
            Msg::FormSubmitted
        }),
        form.iter().map(view_fieldset),
        save_button,
    ]
}

enum SaveButton {
    CreateArticle,
    UpdateArticle,
}

fn view_save_button(type_: SaveButton, disabled: bool) -> Node<Msg> {
    button![
        class!["btn", "btn-lg", "btn-primary", "pull-xs-right"],
        simple_ev(Ev::Click, Msg::FormSubmitted),
        attrs!{At::Type => "button", At::Disabled => disabled},
        match type_ {
            SaveButton::CreateArticle => "Publish Article",
            SaveButton::UpdateArticle => "Update Article",
        }
    ]
}

fn view_content(model: &Model) -> Node<Msg> {
    div![
        class!["auth-page"],
        div![
            class!["container", "page"],
            div![
                class!["row"],

                div![
                    class!["col-md-6", "offset-md-3", "col-x32-12"],

                    if let Some(viewer) = model.session().viewer() {
                        view_authenticated(viewer, model)
                    } else {
                        vec![
                            div![
                                "Sign in to edit this article."
                            ]
                        ]
                    }
                ]

            ]
        ]
    ]
}

fn view_authenticated(viewer: &viewer::Viewer, model: &Model) -> Vec<Node<Msg>> {
    match &model.status {
        Status::Loading(_) | Status::Placeholder => {
            vec![]
        },
        Status::LoadingSlowly(_) => {
            vec![loading::icon()]
        },
        Status::LoadingFailed(slug, problems) => {
            vec![
                view_problems(problems),
                loading::error("article")
            ]
        },
        Status::Saving(_, form) => {
            vec![view_form(form, view_save_button(SaveButton::UpdateArticle, true))]
        },
        Status::Editing(_, problems, form) => {
            vec![
                view_problems(problems),
                view_form(form, view_save_button(SaveButton::UpdateArticle, false)),
            ]
        },
        Status::EditingNew(problems, form) => {
            vec![
                view_problems(problems),
                view_form(form, view_save_button(SaveButton::CreateArticle, false)),
            ]
        },
        Status::Creating(form) => {
            vec![view_form(form, view_save_button(SaveButton::CreateArticle, true))]
        },
    }
}

fn view_problems(problems: &[form::Problem]) -> Node<Msg> {
    ul![
        class!["error-messages"],
        problems.iter().map(|problem| li![
            problem.message()
        ])
    ]
}