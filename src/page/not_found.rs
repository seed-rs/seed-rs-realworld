use super::ViewPage;
use crate::entity::asset;
use seed::prelude::*;

// View

pub fn view<'a, Ms>() -> ViewPage<'a, Ms> {
    ViewPage::new("Page Not Found", view_content())
}

fn view_content<Ms>() -> Node<Ms> {
    main![
        id!("content"),
        class!["container"],
        attrs! {At::TabIndex => -1},
        h1!["Not Found"],
        div![
            class!["row"],
            img![attrs! {At::Src => asset::error().url()}]
        ]
    ]
}
