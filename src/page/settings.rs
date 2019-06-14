use seed::prelude::*;
use super::ViewPage;
use crate::session;

// Model

pub struct Model {
    session: session::Session
}

impl From<Model> for session::Session {
    fn from(model: Model) -> session::Session {
        model.session
    }
}

// View

pub fn view<Ms>() -> ViewPage<Ms> {
    ViewPage {
        title: "Settings".into(),
        content: view_content()
    }
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
                                    attrs!{At::Type => "text"; At::PlaceHolder => "URL of profile picture"}
                                ]
                            ],
                            fieldset![
                                class!["form-group"],
                                input![
                                    class!["form-control", "form-control-lg"],
                                    attrs!{At::Type => "text"; At::PlaceHolder => "Your Name"}
                                ]
                            ],
                            fieldset![
                                class!["form-group"],
                                textarea![
                                    class!["form-control", "form-control-lg"],
                                    attrs!{At::Rows => 8; At::PlaceHolder => "Short bio about you"}
                                ]
                            ],
                            fieldset![
                                class!["form-group"],
                                input![
                                    class!["form-control", "form-control-lg"],
                                    attrs!{At::Type => "text"; At::PlaceHolder => "Email"}
                                ]
                            ],
                            fieldset![
                                class!["form-group"],
                                input![
                                    class!["form-control", "form-control-lg"],
                                    attrs!{At::Type => "password"; At::PlaceHolder => "Password"}
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