#[macro_use]
extern crate seed;
use seed::prelude::*;


// Model

#[derive(Clone, Copy)]
enum Page {
    Home,
    LoginRegister,
    Profile,
    Settings,
    CreateEditArticle,
    Article,
    NotFound,
}

impl Default for Page {
    fn default() -> Self {
        Page::Home
    }
}

#[derive(Default)]
struct Model {
    page: Page
}

// Update

#[derive(Clone)]
enum Msg {
    ChangePage(Page),
}

fn update(msg: Msg, model: &mut Model, _: &mut Orders<Msg>) {
    match msg {
        Msg::ChangePage(page) => model.page = page,
    }
}

// Routes

fn routes(url: &seed::Url) -> Msg {
    match url.path[0].as_ref() {
        "home" | "" => Msg::ChangePage(Page::default()),
        "login-register" => Msg::ChangePage(Page::LoginRegister),
        "profile" => Msg::ChangePage(Page::Profile),
        "settings" => Msg::ChangePage(Page::Settings),
        "create-edit-article" => Msg::ChangePage(Page::CreateEditArticle),
        "article" => Msg::ChangePage(Page::Article),
        _ => Msg::ChangePage(Page::NotFound),
    }
}

// View

fn view(model: &Model) -> impl ElContainer<Msg> {
    use Page::*;
    let header = view_header();
    let content = match model.page {
        Home => view_home_page(),
        LoginRegister => view_login_register_page(),
        Profile => view_profile_page(),
        Settings => view_settings_page(),
        CreateEditArticle => view_create_edit_article_page(),
        Article => view_article_page(),
        NotFound => view_not_found_page(),
    };
    let footer = view_footer();
    vec![ header, content, footer]
}

fn view_header() -> El<Msg> {
    nav![
        class!["navbar", "navbar-light"],
        div![
            class!["container"],
            a![
                class!["navbar-brand"],
                attrs!{At::Href => "/"},
                "conduit"
            ],
            ul![
                class!["nav navbar-nav pull-xs-right"],
                li![
                    class!["nav-item"],
                    a![
                        // add "active" class when you're on that page"
                        class!["nav-link", "active"],
                        attrs!{At::Href => ""},
                        "Home"
                    ],
                ],
                li![
                    class!["nav-item"],
                    a![
                        class!["nav-link"],
                        attrs!{At::Href => ""},
                        i![
                            class!["ion-compose"]
                        ],
                        El::from_html("&nbsp;"),
                        "New Post"
                    ],
                ],
                li![
                    class!["nav-item"],
                    a![
                        class!["nav-link"],
                        attrs!{At::Href => ""},
                        i![
                            class!["ion-gear-a"]
                        ],
                        El::from_html("&nbsp;"),
                        "Settings"
                    ],
                ]
            ],
        ]
    ]
}

fn view_footer() -> El<Msg> {
    footer![
        div![
            class!["container"],
            a![
                class!["logo-font"],
                attrs!{At::Href => "/"},
                "conduit"
            ],
            span![
                class!["attribution"],
                "An interactive learning project from ",
                a![
                    attrs!{At::Href => "https://thinkster.io"},
                    "Thinkster"
                ],
                ". Code & design licensed under MIT."
            ]
        ]
    ]
}

fn view_home_page() -> El<Msg> {
    div![
        class!["home-page"],

        div![
            class!["banner"],
            div![
                class!["container"],
                h1![
                    class!["logo-font"],
                    "conduit"
                ],
                p![
                    "A place to share your knowledge."
                ]
            ]
        ],

        div![
            class!["container", "page"],
            div![
                class!["row"],

                div![
                    class!["col-md-9"],
                    div![
                        class!["feed-toggle"],
                        ul![
                            class!["nav", "nav-pills", "outline-active"],
                            li![
                                class!["nav-item"],
                                a![
                                    class!["nav-link", "disabled"],
                                    attrs!{At::Href => ""},
                                    "Your Feed"
                                ]
                            ],
                            li![
                                class!["nav-item"],
                                a![
                                    class!["nav-link", "active"],
                                    attrs!{At::Href => ""},
                                    "Global Feed"
                                ]
                            ],
                        ],
                    ],

                    div![
                        class!["article-preview"],
                        div![
                            class!["article-meta"],
                            a![
                                attrs!{At::Href => "/profile"},
                                img![
                                    attrs!{At::Src => "http://i.imgur.com/Qr71crq.jpg"}
                                ]
                            ],
                            div![
                                class!["info"],
                                a![
                                    class!["author"],
                                    attrs!{At::Href => ""},
                                    "Eric Simons"
                                ],
                                span![
                                    class!["date"],
                                    "January 20th"
                                ]
                            ],
                            button![
                                class!["btn","btn-outline-primary", "btn-sm", "pull-xs-right"],
                                i![
                                    class!["ion-heart"],
                                    " 29"
                                ]
                            ]
                        ],
                        a![
                            class!["preview-link"],
                            attrs!{At::Href => ""},
                            h1![
                                "How to build webapps that scale"
                            ],
                            p![
                                "This is the description for the post."
                            ],
                            span![
                                "Read more..."
                            ]
                        ]
                    ],

                    div![
                        class!["article-preview"],
                        div![
                            class!["article-meta"],
                            a![
                                attrs!{At::Href => "/profile"},
                                img![
                                    attrs!{At::Src => "http://i.imgur.com/N4VcUeJ.jpg"}
                                ]
                            ],
                            div![
                                class!["info"],
                                a![
                                    class!["author"],
                                    attrs!{At::Href => ""},
                                    "Albert Pai"
                                ],
                                span![
                                    class!["date"],
                                    "January 20th"
                                ]
                            ],
                            button![
                                class!["btn","btn-outline-primary", "btn-sm", "pull-xs-right"],
                                i![
                                    class!["ion-heart"],
                                    " 32"
                                ]
                            ]
                        ],
                        a![
                            class!["preview-link"],
                            attrs!{At::Href => ""},
                            h1![
                                "The song you won't ever stop singing. No matter how hard you try."
                            ],
                            p![
                                "This is the description for the post."
                            ],
                            span![
                                "Read more..."
                            ]
                        ]
                    ]

                ],

                div![
                    class!["col-md-3"],
                    div![
                        class!["sidebar"],
                        p![
                            "Popular Tags"
                        ],

                        div![
                            class!["tag-list"],
                            a![
                                class!["tag-pill", "tag-default"],
                                attrs!{At::Href => ""},
                                "programming"
                            ],
                            a![
                                class!["tag-pill", "tag-default"],
                                attrs!{At::Href => ""},
                                "javascript"
                            ],
                            a![
                                class!["tag-pill", "tag-default"],
                                attrs!{At::Href => ""},
                                "emberjs"
                            ],
                            a![
                                class!["tag-pill", "tag-default"],
                                attrs!{At::Href => ""},
                                "angularjs"
                            ],
                            a![
                                class!["tag-pill", "tag-default"],
                                attrs!{At::Href => ""},
                                "react"
                            ],
                            a![
                                class!["tag-pill", "tag-default"],
                                attrs!{At::Href => ""},
                                "mean"
                            ],
                            a![
                                class!["tag-pill", "tag-default"],
                                attrs!{At::Href => ""},
                                "node"
                            ],
                            a![
                                class!["tag-pill", "tag-default"],
                                attrs!{At::Href => ""},
                                "rails"
                            ]
                        ]
                    ]
                ]

            ]
        ]

    ]
}

fn view_login_register_page() -> El<Msg> {
    empty![]
}

fn view_profile_page() -> El<Msg> {
    empty![]
}

fn view_settings_page() -> El<Msg> {
    empty![]
}

fn view_create_edit_article_page() -> El<Msg> {
    empty![]
}

fn view_article_page() -> El<Msg> {
    empty![]
}

fn view_not_found_page() -> El<Msg> {
    empty![]
}

#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::default(), update, view)
        .routes(routes)
        .finish()
        .run();

}