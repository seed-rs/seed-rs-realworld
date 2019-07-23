use super::ViewPage;
use crate::entity::Image;
use seed::prelude::*;

// ------ ------
//     View
// ------ ------

pub fn view<'a, Ms>() -> ViewPage<'a, Ms> {
    ViewPage::new("Page Not Found", view_content())
}

// ====== PRIVATE ======

fn view_content<Ms>() -> Node<Ms> {
    main![
        id!("content"),
        class!["container"],
        attrs! {At::TabIndex => -1},
        h1!["Not Found"],
        div![
            class!["row"],
            img![attrs! {At::Src => Image::error().url()}]
        ]
    ]
}
