use seed::prelude::*;
use super::{ViewPage, InitPage};
use crate::{session, article, GMsg, route, HasSessionChangedOnInit};

// Model

pub struct Model {
    session: session::Session
}

impl<'a> Model {
    pub fn session(&self) -> &session::Session {
        &self.session
    }
}

impl<'a> From<Model> for session::Session {
    fn from(model: Model) -> session::Session {
        model.session
    }
}

pub fn init_new(session: session::Session) -> InitPage<Model, Msg> {
    InitPage::new(Model { session })
}

pub fn init_edit<'a>(session: session::Session, slug: &article::slug::Slug) -> InitPage<Model, Msg> {
    InitPage::new(Model { session })
}

// Global msg handler

pub fn g_msg_handler<PMsg>(g_msg: GMsg, model: &mut Model, orders: &mut OrdersProxy<Msg, PMsg, GMsg>) {
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

pub fn update<PMsg>(msg: Msg, model: &mut Model, orders: &mut OrdersProxy<Msg, PMsg, GMsg>) {
}

// View

pub fn view<'a>(model: &Model) -> ViewPage<'a, Msg> {
    ViewPage::new("@TODO",view_content())
}

fn view_content() -> El<Msg> {
    div![
        class!["editor-page"],
        div![
            class!["container", "page"],
            div![
                class!["row"],

                div![
                    class!["col-md-10", "offset-md-1", "col-xs-12"],
                    form![
                        fieldset![
                            fieldset![
                                class!["form-group"],
                                input![
                                    class!["form-control", "form-control-lg"],
                                    attrs!{At::Type => "text"; At::Placeholder => "Article Title"}
                                ]
                            ],
                            fieldset![
                                class!["form-group"],
                                input![
                                    class!["form-control"],
                                    attrs!{At::Type => "text"; At::Placeholder => "What's this article about?"}
                                ]
                            ],
                            fieldset![
                                class!["form-group"],
                                textarea![
                                    class!["form-control"],
                                    attrs!{At::Rows => 8; At::Placeholder => "Write your article (in markdown)"}
                                ]
                            ],
                            fieldset![
                                class!["form-group"],
                                input![
                                    class!["form-control"],
                                    attrs!{At::Type => "text"; At::Placeholder => "Enter tags"}
                                ],
                                div![
                                    class!["tag-list"]
                                ]
                            ],
                            button![
                                class!["btn", "btn-lg", "pull-xs-right", "btn-primary"],
                                attrs!{At::Type => "button"},
                                "Publish Article"
                            ]
                        ]
                    ]
                ]

            ]
        ]
    ]
}