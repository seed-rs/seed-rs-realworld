use seed::prelude::*;
use crate::asset;
use super::ViewPage;

// View

pub fn view<Ms>() -> ViewPage<Ms> {
    ViewPage {
        title: "Page Not Found".into(),
        content: view_content()
    }
}

fn view_content<Ms>() -> El<Ms> {
    main![
        id!("content"),
        class!["container"],
        attrs!{At::TabIndex => -1},

        h1![
            "Not Found"
        ],
        div![
            class!["row"],
            img![
                attrs!{At::Src => asset::error().url()}
            ]
        ]
    ]
}