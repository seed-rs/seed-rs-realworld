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
    div![
        class!["auth-page"],
        div![
            class!["container", "page"],
            div![
                class!["row"],

                div![
                    class!["col-md-6", "offset-md-3", "col-x32-12"],
                    h1![
                        class!["text-xs-center"],
                        "Sign up"
                    ],
                    p![
                        class!["text-xs-center"],
                        a![
                            attrs!{At::Href => ""},
                            "Have an account?"
                        ]
                    ],

                    ul![
                        class!["error-messages"],
                        li![
                            "That email is already taken"
                        ]
                    ],

                    form![
                        fieldset![
                            class!["form-group"],
                            input![
                                class!["form-control", "form-control-lg"],
                                attrs!{At::Type => "text"; At::PlaceHolder => "Your Name"}
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
                            "Sign up"
                        ]
                    ]
                ]

            ]
        ]
    ]
}

fn view_profile_page() -> El<Msg> {
    div![
        class!["profile-page"],

        div![
            class!["user-info"],
            div![
                class!["container"],
                div![
                    class!["row"],

                    div![
                        class!["col-xs-12", "col-md-10", "offset-md-1"],
                        img![
                            class!["user-img"],
                            attrs!{At::Src => "http://i.imgur.com/Qr71crq.jpg"}
                        ],
                        p![
                            "Cofounder @GoThinkster, lived in Aol's HQ for a few months, kinda looks like Peeta from the Hunger Games"
                        ],
                        button![
                            class!["btn", "btn-sm", "btn-outline-secondary", "action-btn"],
                            i![
                                class!["ion-plus-round"]
                            ],
                            El::from_html("&nbsp;"),
                            "Follow Eric Simons"
                        ]
                    ]

                ]
            ]
        ],

        div![
            class!["container"],
            div![
                class!["row"],
                div![
                    class!["col-xs-12", "col-md-10", "offset-md-1"],
                    div![
                        class!["articles-toggle"],
                        ul![
                            class!["nav", "nav-pills", "outline-active"],
                            li![
                                class!["nav-item"],
                                a![
                                    class!["nav-link", "active"],
                                    attrs!{At::Href => ""},
                                    "My Articles"
                                ]
                            ],
                            li![
                                class!["nav-item"],
                                a![
                                    class!["nav-link"],
                                    attrs!{At::Href => ""},
                                    "Favorited Articles"
                                ]
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

                ]
            ]
        ]

    ]
}

fn view_settings_page() -> El<Msg> {
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

fn view_create_edit_article_page() -> El<Msg> {
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
                                    attrs!{At::Type => "text"; At::PlaceHolder => "Article Title"}
                                ]
                            ],
                            fieldset![
                                class!["form-group"],
                                input![
                                    class!["form-control"],
                                    attrs!{At::Type => "text"; At::PlaceHolder => "What's this article about?"}
                                ]
                            ],
                            fieldset![
                                class!["form-group"],
                                textarea![
                                    class!["form-control"],
                                    attrs!{At::Rows => 8; At::PlaceHolder => "Write your article (in markdown)"}
                                ]
                            ],
                            fieldset![
                                class!["form-group"],
                                input![
                                    class!["form-control"],
                                    attrs!{At::Type => "text"; At::PlaceHolder => "Enter tags"}
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

fn view_article_page() -> El<Msg> {
    div![
        class!["article-page"],

        div![
            class!["banner"],
            div![
                class!["container"],

                h1![
                    "How to build webapps that scale"
                ],

                div![
                    class!["article-meta"],
                    a![
                        attrs!{At::Href => ""},
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
                        class!["btn", "btn-sm", "btn-outline-secondary"],
                        i![
                            class!["ion-plus-round"]
                        ],
                        El::from_html("&nbsp;"),
                        "Follow Eric Simons ",
                        span![
                            class!["counter"],
                            "(10)"
                        ]
                    ],
                    El::from_html("&nbsp;&nbsp;"),
                    button![
                        class!["btn", "btn-sm", "btn-outline-primary"],
                        i![
                            class!["ion-heart"]
                        ],
                        El::from_html("&nbsp;"),
                        "Favorite Post ",
                        span![
                            class!["counter"],
                            "(29)"
                        ]
                    ],
                ]

            ]
        ],

        div![
            class!["container", "page"],

            div![
                class!["row", "article-content"],
                div![
                    class!["col-md-12"],
                    p![
                        "Web development technologies have evolved at an incredible clip over the past few years."
                    ],
                    h2![
                        id!("introducing-ionic"),
                        "Introducing RealWorld."
                    ],
                    p![
                        "It's a great solution for learning how other frameworks work."
                    ]
                ]
            ],

            hr![],

            div![
                class!["article-actions"],
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
                        class!["btn", "btn-sm", "btn-outline-secondary"],
                        i![
                            class!["ion-plus-round"]
                        ],
                        El::from_html("&nbsp;"),
                        "Follow Eric Simons ",
                        span![
                            class!["counter"],
                            "(10)"
                        ]
                    ],
                    El::from_html("&nbsp;&nbsp;"),
                    button![
                        class!["btn", "btn-sm", "btn-outline-primary"],
                        i![
                            class!["ion-heart"]
                        ],
                        El::from_html("&nbsp;"),
                        "Favorite Post ",
                        span![
                            class!["counter"],
                            "(29)"
                        ]
                    ],
                ]
            ],

            div![
                class!["row"],

                div![
                    class!["col-xs-12", "col-md-8", "offset-md-2"],

                    form![
                        class!["card", "comment-form"],
                        div![
                            class!["card-block"],
                            textarea![
                                class!["form-control"],
                                attrs!{At::Rows => 3; At::PlaceHolder => "Write a comment..."}
                            ]
                        ],
                        div![
                            class!["card-footer"],
                            img![
                                class!["comment-author-img"],
                                attrs!{At::Src => "http://i.imgur.com/Qr71crq.jpg"}
                            ],
                            button![
                                class!["btn", "btn-sm", "btn-primary"],
                                "Post Comment"
                            ]
                        ]
                    ],

                    div![
                        class!["card"],
                        div![
                            class!["card-block"],
                            p![
                                class!["card-text"],
                                "With supporting text below as a natural lead-in to additional content."
                            ]
                        ],
                        div![
                            class!["card-footer"],
                            a![
                                class!["comment-author"],
                                attrs!{At::Href => ""},
                                img![
                                    class!["comment-author-img"],
                                    attrs!{At::Src => "http://i.imgur.com/Qr71crq.jpg"}
                                ]
                            ],
                            El::from_html("&nbsp;"),
                            a![
                                class!["comment-author"],
                                attrs!{At::Href => ""},
                                "Jacob Schmidt"
                            ],
                            span![
                                class!["date-posted"],
                                "Dec 29th"
                            ]
                        ]
                    ],

                    div![
                        class!["card"],
                        div![
                            class!["card-block"],
                            p![
                                class!["card-text"],
                                "With supporting text below as a natural lead-in to additional content."
                            ]
                        ],
                        div![
                            class!["card-footer"],
                            a![
                                class!["comment-author"],
                                attrs!{At::Href => ""},
                                img![
                                    class!["comment-author-img"],
                                    attrs!{At::Src => "http://i.imgur.com/Qr71crq.jpg"}
                                ]
                            ],
                            El::from_html("&nbsp;"),
                            a![
                                class!["comment-author"],
                                attrs!{At::Href => ""},
                                "Jacob Schmidt"
                            ],
                            span![
                                class!["date-posted"],
                                "Dec 29th"
                            ],
                            span![
                                class!["mod-options"],
                                i![
                                    class!["ion-edit"]
                                ],
                                i![
                                    class!["ion-trash-a"]
                                ]
                            ]
                        ]
                    ]

                ]

            ]

        ]
    ]
}

fn view_not_found_page() -> El<Msg> {
    h2![
        "404"
    ]
}

#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::default(), update, view)
        .routes(routes)
        .finish()
        .run();

}