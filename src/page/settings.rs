use seed::prelude::*;
use super::{ViewPage, InitPage};
use crate::session;

// Model

pub struct Model<'a> {
    session: session::Session<'a>
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
    InitPage::new(Model { session })
}

// Update

pub enum Msg {

}

pub fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {

}

// View

pub fn view<Ms>() -> ViewPage<'static, Ms> {
    ViewPage::new("Settings",view_content())
}

fn view_content<Ms>() -> El<Ms> {
    div![
        class!["settings-page"],
        div![
            class!["container", "page"],
            div![
                class!["row"],

                div![
                    class!["col-md-6", "offset-md-3", "col-xs12"],
                    h1![
                        class!["text-xs-center"],
                        "Your settings"
                    ],

                    form![
                        fieldset![
                            fieldset![
                                class!["form-group"],
                                input![
                                    class!["form-control"],
                                    attrs!{At::Type => "text"; At::Placeholder => "URL of profile picture"}
                                ]
                            ],
                            fieldset![
                                class!["form-group"],
                                input![
                                    class!["form-control", "form-control-lg"],
                                    attrs!{At::Type => "text"; At::Placeholder => "Your Name"}
                                ]
                            ],
                            fieldset![
                                class!["form-group"],
                                textarea![
                                    class!["form-control", "form-control-lg"],
                                    attrs!{At::Rows => 8; At::Placeholder => "Short bio about you"}
                                ]
                            ],
                            fieldset![
                                class!["form-group"],
                                input![
                                    class!["form-control", "form-control-lg"],
                                    attrs!{At::Type => "text"; At::Placeholder => "Email"}
                                ]
                            ],
                            fieldset![
                                class!["form-group"],
                                input![
                                    class!["form-control", "form-control-lg"],
                                    attrs!{At::Type => "password"; At::Placeholder => "Password"}
                                ]
                            ],
                            button![
                                class!["btn", "btn-lg", "btn-primary", "pull-xs-right"],
                                "Update Settings"
                            ]
                        ]
                    ]
                ]

            ]
        ]
    ]
}