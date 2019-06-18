#[macro_use]
extern crate seed;
use seed::prelude::*;
use std::convert::TryInto;

mod asset;
mod avatar;
mod username;
mod api;
mod viewer;
mod session;
mod page;
mod article;
mod route;

// Model

enum Model<'a> {
    None,
    Redirect(session::Session<'a>),
    NotFound(session::Session<'a>),
    Home(page::home::Model<'a>),
    Settings(page::settings::Model<'a>),
    Login(page::login::Model<'a>),
    Register(page::register::Model<'a>),
    Profile(page::profile::Model<'a>, username::Username<'a>),
    Article(page::article::Model<'a>),
    ArticleEditor(page::article_editor::Model<'a>, Option<article::slug::Slug<'a>>)
}

impl<'a> Default for Model<'a> {
    fn default() -> Self {
        Model::None
    }
}

impl<'a> Model<'a> {
    pub fn take(&mut self) -> Model<'a> {
        std::mem::replace(self, Model::None)
    }

    pub fn session(&self) -> Option<&session::Session> {
        match &self {
            Model::None => None,
            Model::Redirect(session) => Some(session),
            Model::NotFound(session) => Some(session),
            Model::Home(model) => Some(model.session()),
            Model::Settings(model) => Some(model.session()),
            Model::Login(model) => Some(model.session()),
            Model::Register(model) => Some(model.session()),
            Model::Profile(model, _) => Some(model.session()),
            Model::Article(model) => Some(model.session()),
            Model::ArticleEditor(model, _) => Some(model.session()),
        }
    }
}

impl<'a> From<Model<'a>> for session::Session<'a> {
    fn from(model: Model<'a>) -> session::Session<'a> {
        match model {
            Model::None => session::Session::default(),
            Model::Redirect(session) => session,
            Model::NotFound(session) => session,
            Model::Home(model) => model.into(),
            Model::Settings(model) => model.into(),
            Model::Login(model) => model.into(),
            Model::Register(model) => model.into(),
            Model::Profile(model, _) => model.into(),
            Model::Article(model) => model.into(),
            Model::ArticleEditor(model, _) => model.into(),
        }
    }
}

// Update

enum Msg<'a> {
    ChangedRoute(Option<route::Route<'a>>),
    GotSession(session::Session<'a>),
    GotHomeMsg(page::home::Msg),
    GotSettingsMsg(page::settings::Msg),
    GotLoginMsg(page::login::Msg),
    GotRegisterMsg(page::register::Msg),
    GotProfileMsg(page::profile::Msg),
    GotArticleMsg(page::article::Msg),
    GotArticleEditorMsg(page::article_editor::Msg),
}

fn update<'a>(msg: Msg<'a>, model: &mut Model<'a>, orders: &mut Orders<Msg<'static>>) {
    match msg {
        Msg::ChangedRoute(route) => change_route_to(route, model, orders),
        Msg::GotSession(session) => {
            *model = Model::Redirect(session);
            route::replace_url(route::Route::Home);
        },
        Msg::GotHomeMsg(sub_msg) => {
            if let Model::Home(model) = model {
                *orders = call_update(page::home::update, sub_msg, model)
                    .map_message(Msg::GotHomeMsg);
            }
        },
        Msg::GotSettingsMsg(sub_msg) => {
            if let Model::Settings(model) = model {
                *orders = call_update(page::settings::update, sub_msg, model)
                    .map_message(Msg::GotSettingsMsg);
            }
        },
        Msg::GotLoginMsg(sub_msg) => {
            if let Model::Login(model) = model {
                *orders = call_update(page::login::update, sub_msg, model)
                    .map_message(Msg::GotLoginMsg)
            }
        },
        Msg::GotRegisterMsg(sub_msg) => {
            if let Model::Register(model) = model {
                *orders = call_update(page::register::update, sub_msg, model)
                    .map_message(Msg::GotRegisterMsg)
            }
        },
        Msg::GotProfileMsg(sub_msg) => {
            if let Model::Profile(model, _) = model {
                *orders = call_update(page::profile::update, sub_msg, model)
                    .map_message(Msg::GotProfileMsg)
            }
        },
        Msg::GotArticleMsg(sub_msg) => {
            if let Model::Article(model) = model {
                *orders = call_update(page::article::update, sub_msg, model)
                    .map_message(Msg::GotArticleMsg)
            }
        },
        Msg::GotArticleEditorMsg(sub_msg) => {
            if let Model::ArticleEditor(model, _) = model {
                *orders = call_update(page::article_editor::update, sub_msg, model)
                    .map_message(Msg::GotArticleEditorMsg)
            }
        },
    }
}

fn change_route_to<'a>(route: Option<route::Route<'a>>, model: &mut Model<'a>, orders:&mut Orders<Msg<'static>>) {
    let mut session = || session::Session::from(model.take());
    match route {
        None => { *model = Model::NotFound(session()) },
        Some(route) => match route {
            route::Route::Root => {
                route::replace_url(route::Route::Home)
            },
            route::Route::Logout => {
                api::logout()
            },
            route::Route::NewArticle => {
                let init_page = page::article_editor::init_new(session());
                *model = Model::ArticleEditor(init_page.model, None);
                *orders = init_page.orders.map_message(Msg::GotArticleEditorMsg);
            },
            route::Route::EditArticle(slug) => {
                let init_page = page::article_editor::init_edit(session(), &slug);
                *model = Model::ArticleEditor(init_page.model, Some(slug));
                *orders = init_page.orders.map_message(Msg::GotArticleEditorMsg);
            },
            route::Route::Settings => {
                let init_page = page::settings::init(session());
                *model = Model::Settings(init_page.model);
                *orders = init_page.orders.map_message(Msg::GotSettingsMsg);
            },
            route::Route::Home => {
                let init_page = page::home::init(session());
                *model = Model::Home(init_page.model);
                *orders = init_page.orders.map_message(Msg::GotHomeMsg);
            },
            route::Route::Login => {
                let init_page = page::login::init(session());
                *model = Model::Login(init_page.model);
                *orders = init_page.orders.map_message(Msg::GotLoginMsg);
            },
            route::Route::Register => {
                let init_page = page::register::init(session());
                *model = Model::Register(init_page.model);
                *orders = init_page.orders.map_message(Msg::GotRegisterMsg);
            },
            route::Route::Profile(username) => {
                let init_page = page::profile::init(session(), &username);
                *model = Model::Profile(init_page.model, username);
                *orders = init_page.orders.map_message(Msg::GotProfileMsg);
            },
            route::Route::Article(slug) => {
                let init_page = page::article::init(session(), slug);
                *model = Model::Article(init_page.model);
                *orders = init_page.orders.map_message(Msg::GotArticleMsg);
            },
        }
    };
}

// View

fn view<'a>(model: &Model) -> impl ElContainer<Msg<'static>> {
    let viewer = || model.session().and_then(session::Session::viewer);
    match model {
        Model::None => vec![],
        Model::Redirect(_) => {
            page::view(
                page::Page::Other,
                page::blank::view(),
                viewer(),
            )
        },
        Model::NotFound(_) => {
            page::view(
                page::Page::Other,
                page::not_found::view(),
                viewer(),
            )
        },
        Model::Settings(model) => {
            page::view(
                page::Page::Settings,
                page::settings::view(model),
                viewer(),
            ).map_message(Msg::GotSettingsMsg)
        },
        Model::Home(model) => {
            page::view(
                page::Page::Home,
                page::home::view(model),
                viewer(),
            ).map_message(Msg::GotHomeMsg)
        },
        Model::Login(model) => {
            page::view(
                page::Page::Login,
                page::login::view(model),
                viewer(),
            ).map_message(Msg::GotLoginMsg)
        },
        Model::Register(model) => {
            page::view(
                page::Page::Register,
                page::register::view(model),
                viewer(),
            ).map_message(Msg::GotRegisterMsg)
        },
        Model::Profile(model, username) => {
            page::view(
                page::Page::Profile(username),
                page::profile::view(model),
                viewer(),
            ).map_message(Msg::GotProfileMsg)
        },
        Model::Article(model) => {
            page::view(
                page::Page::Other,
                page::article::view(model),
                viewer(),
            ).map_message(Msg::GotArticleMsg)
        },
        Model::ArticleEditor(model, None) => {
            page::view(
                page::Page::NewArticle,
                page::article_editor::view(model),
                viewer(),
            ).map_message(Msg::GotArticleEditorMsg)
        },
        Model::ArticleEditor(model, Some(_)) => {
            page::view(
                page::Page::Other,
                page::article_editor::view(model),
                viewer(),
            ).map_message(Msg::GotArticleEditorMsg)
        },
    }
}

#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::default(), update, view)
        .routes(|url| Msg::ChangedRoute(url.try_into().ok()))
        .finish()
        .run();
}