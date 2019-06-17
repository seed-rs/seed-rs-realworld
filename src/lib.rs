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
                *orders = call_update(page::home::update, sub_msg, model).map_message(Msg::GotHomeMsg);
            }
        },
        Msg::GotSettingsMsg(sub_msg) => {
            if let Model::Settings(model) = model {
                *orders = call_update(page::settings::update, sub_msg, model).map_message(Msg::GotSettingsMsg);
            }
        },
        Msg::GotLoginMsg(sub_msg) => {
            if let Model::Login(model) = model {
                *orders = call_update(page::login::update, sub_msg, model).map_message(Msg::GotLoginMsg)
            }
        },
        Msg::GotRegisterMsg(sub_msg) => {
            if let Model::Register(model) = model {
                *orders = call_update(page::register::update, sub_msg, model).map_message(Msg::GotRegisterMsg)
            }
        },
        Msg::GotProfileMsg(sub_msg) => {
            if let Model::Profile(model, _) = model {
                *orders = call_update(page::profile::update, sub_msg, model).map_message(Msg::GotProfileMsg)
            }
        },
        Msg::GotArticleMsg(sub_msg) => {
            if let Model::Article(model) = model {
                *orders = call_update(page::article::update, sub_msg, model).map_message(Msg::GotArticleMsg)
            }
        },
        Msg::GotArticleEditorMsg(sub_msg) => {
            if let Model::ArticleEditor(model, _) = model {
                *orders = call_update(page::article_editor::update, sub_msg, model).map_message(Msg::GotArticleEditorMsg)
            }
        },
    }
}

fn change_route_to<'a>(route: Option<route::Route<'a>>, model: &mut Model<'a>, orders:&mut Orders<Msg<'static>>) {
    match route {
        None => { *model = Model::NotFound(model.take().into()) },
        Some(route) => match route {
            route::Route::Root => {
                route::replace_url(route::Route::Home)
            },
            route::Route::Logout => {
                api::logout()
            },
            route::Route::NewArticle => {
                let (page_model, page_orders) = page::article_editor::init_new(
                    model.take().into()
                ).into_tuple();

                *model = Model::ArticleEditor(page_model, None);
                *orders = page_orders.map_message(Msg::GotArticleEditorMsg);
            },
            route::Route::EditArticle(slug) => {
                let (page_model, page_orders) = page::article_editor::init_edit(
                    model.take().into(), &slug
                ).into_tuple();

                *model = Model::ArticleEditor(page_model, Some(slug));
                *orders = page_orders.map_message(Msg::GotArticleEditorMsg);
            },
            route::Route::Settings => {
                let (page_model, page_orders) = page::settings::init(
                    model.take().into()
                ).into_tuple();

                *model = Model::Settings(page_model);
                *orders = page_orders.map_message(Msg::GotSettingsMsg);
            },
            route::Route::Home => {
                let (page_model, page_orders) = page::home::init(
                    model.take().into()
                ).into_tuple();

                *model = Model::Home(page_model);
                *orders = page_orders.map_message(Msg::GotHomeMsg);
            },
            route::Route::Login => {
                let (page_model, page_orders) = page::login::init(
                    model.take().into()
                ).into_tuple();

                *model = Model::Login(page_model);
                *orders = page_orders.map_message(Msg::GotLoginMsg);
            },
            route::Route::Register => {
                let (page_model, page_orders) = page::register::init(
                    model.take().into()
                ).into_tuple();

                *model = Model::Register(page_model);
                *orders = page_orders.map_message(Msg::GotRegisterMsg);
            },
            route::Route::Profile(username) => {
                let (page_model, page_orders) = page::profile::init(
                    model.take().into(), &username
                ).into_tuple();

                *model = Model::Profile(page_model, username);
                *orders = page_orders.map_message(Msg::GotProfileMsg);
            },
            route::Route::Article(slug) => {
                let (page_model, page_orders) = page::article::init(
                    model.take().into(), slug
                ).into_tuple();

                *model = Model::Article(page_model);
                *orders = page_orders.map_message(Msg::GotArticleMsg);
            },
        }
    };
}

// View

fn view<'a>(model: &Model) -> impl ElContainer<Msg<'a>> {
    let viewer = model.session().and_then(session::Session::viewer);
    match model {
        Model::None => vec![],
        Model::Redirect(_) => page::Page::Other(page::blank::view()).view(viewer),
        Model::NotFound(_) => page::Page::Other(page::not_found::view()).view(viewer),
        Model::Settings(_) => page::Page::Settings.view(viewer),
        Model::Home(_) => page::Page::Home.view(viewer),
        Model::Login(_) => page::Page::Login.view(viewer),
        Model::Register(_) => page::Page::Register.view(viewer),
        Model::Profile(_, username) => page::Page::Profile(username).view(viewer),
        Model::Article(_) => page::Page::Other(page::article::view()).view(viewer),
        Model::ArticleEditor(_, None) => page::Page::NewArticle.view(viewer),
        Model::ArticleEditor(_, Some(_)) => page::Page::Other(page::article_editor::view()).view(viewer),
    }
}

#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::default(), update, view)
        .routes(|url| Msg::ChangedRoute(url.try_into().ok()))
        .finish()
        .run();
}