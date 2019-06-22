use seed::prelude::*;
use super::{ViewPage, InitPage};
use crate::{session, article, SubMsg, Subs, route, HasSessionChangedOnInit};

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

// Subscriptions

pub fn subscriptions(sub_msg: SubMsg, _: &Model) -> Option<Msg> {
    match sub_msg {
        SubMsg::SessionChanged(session, on_init) => {
            Some(Msg::GotSession(session, on_init))
        }
        _ => None
    }
}

// Update

pub enum Msg {
    GotSession(session::Session, HasSessionChangedOnInit),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>, subs: &mut Subs) {
    match msg {
        Msg::GotSession(session, on_init) => {
            model.session = session;
            if !on_init {
                route::go_to(route::Route::Home, subs);
            }
        }
    }
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