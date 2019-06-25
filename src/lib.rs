#[macro_use]
extern crate seed;
use seed::prelude::*;
use std::convert::TryInto;
use std::collections::VecDeque;

mod asset;
mod avatar;
mod username;
mod api;
mod viewer;
mod session;
mod page;
mod article;
mod route;
mod framework;

// Model

enum Model<'a> {
    Redirect(session::Session),
    NotFound(session::Session),
    Home(page::home::Model),
    Settings(page::settings::Model),
    Login(page::login::Model),
    Register(page::register::Model),
    Profile(page::profile::Model, username::Username<'a>),
    Article(page::article::Model),
    ArticleEditor(page::article_editor::Model, Option<article::slug::Slug<'a>>)
}

impl<'a> Default for Model<'a> {
    fn default() -> Self {
        Model::Redirect(session::Session::default())
    }
}

impl<'a> Model<'a> {
    pub fn take(&mut self) -> Model<'a> {
        std::mem::replace(self, Model::default())
    }

    pub fn session(&self) -> Option<&session::Session> {
        match &self {
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

impl<'a> From<Model<'a>> for session::Session {
    fn from(model: Model<'a>) -> session::Session {
        match model {
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

// Global msg handler

#[derive(Default)]
pub struct GMsgs(VecDeque<GMsg>);

impl GMsgs {
    pub fn send_g_msg(&mut self, g_msg: GMsg) {
        self.0.push_back(g_msg);
    }
}

#[derive(Clone)]
pub enum GMsg {
    RoutePushed(route::Route<'static>),
    SessionChanged(session::Session, HasSessionChangedOnInit)
}

pub type HasSessionChangedOnInit = bool;

fn g_msg_handler<'a>(g_msg: GMsg, model: &mut Model<'a>, orders: &mut Orders<Msg<'static>, GMsg>) {
    match g_msg.clone() {
        GMsg::RoutePushed(route) => {
            orders.send_msg(Msg::ChangedRoute(Some(route)));
        },
        _ => ()
    }

    match model {
        Model::NotFound(_) | Model::Redirect(_) => {
            if let GMsg::SessionChanged(session, false) = g_msg {
                orders.send_msg(Msg::GotSession(session));
            }
        },
        Model::Settings(model) => {
            page::settings::g_msg_handler(g_msg, model, &mut orders.proxy(Msg::GotSettingsMsg));
        },
        Model::Home(model) => {
            page::home::g_msg_handler(g_msg, model, &mut orders.proxy(Msg::GotHomeMsg));
        },
        Model::Login(model) => {
            page::login::g_msg_handler(g_msg, model, &mut orders.proxy(Msg::GotLoginMsg));
        },
        Model::Register(model) => {
            page::register::g_msg_handler(g_msg, model, &mut orders.proxy(Msg::GotRegisterMsg));
        },
        Model::Profile(model, _) => {
            page::profile::g_msg_handler(g_msg, model, &mut orders.proxy(Msg::GotProfileMsg));
        },
        Model::Article(model) => {
            page::article::g_msg_handler(g_msg, model, &mut orders.proxy(Msg::GotArticleMsg));
        },
        Model::ArticleEditor(model, _) => {
            page::article_editor::g_msg_handler(g_msg, model, &mut orders.proxy(Msg::GotArticleEditorMsg));
        },
    }
}

// Update

enum Msg<'a> {
    Init,
    ChangedRoute(Option<route::Route<'a>>),
    GotSession(session::Session),
    GotHomeMsg(page::home::Msg),
    GotSettingsMsg(page::settings::Msg),
    GotLoginMsg(page::login::Msg),
    GotRegisterMsg(page::register::Msg),
    GotProfileMsg(page::profile::Msg),
    GotArticleMsg(page::article::Msg),
    GotArticleEditorMsg(page::article_editor::Msg),
}

fn update<'a>(msg: Msg<'a>, model: &mut Model<'a>, orders: &mut Orders<Msg<'static>, GMsg>) {
    match msg {
        Msg::Init => {
            let session = session::Session::from(api::load_viewer());
            orders.send_g_msg(GMsg::SessionChanged(session, true));
        },
        Msg::ChangedRoute(route) => {
            change_route_to(route, model, orders);
        },
        Msg::GotSession(session) => {
            if let Model::Redirect(_) = model {
                *model = Model::Redirect(session);
                route::go_to(route::Route::Home, orders);
            }
        }
        Msg::GotHomeMsg(module_msg) => {
            if let Model::Home(module_model) = model {
                page::home::update(module_msg, module_model, &mut orders.proxy(Msg::GotHomeMsg));
            }
        },
        Msg::GotSettingsMsg(module_msg) => {
            if let Model::Settings(module_model) = model {
                page::settings::update(module_msg, module_model, &mut orders.proxy(Msg::GotSettingsMsg));
            }
        },
        Msg::GotLoginMsg(module_msg) => {
            if let Model::Login(module_model) = model {
                page::login::update(module_msg, module_model, &mut orders.proxy(Msg::GotLoginMsg));
            }
        },
        Msg::GotRegisterMsg(module_msg) => {
            if let Model::Register(module_model) = model {
                page::register::update(module_msg, module_model, &mut orders.proxy(Msg::GotRegisterMsg));
            }
        },
        Msg::GotProfileMsg(module_msg) => {
            if let Model::Profile(module_model, _) = model {
                page::profile::update(module_msg, module_model, &mut orders.proxy(Msg::GotProfileMsg));
            }
        },
        Msg::GotArticleMsg(module_msg) => {
            if let Model::Article(module_model) = model {
                page::article::update(module_msg, module_model, &mut orders.proxy(Msg::GotArticleMsg));
            }
        },
        Msg::GotArticleEditorMsg(module_msg) => {
            if let Model::ArticleEditor(module_model, _) = model {
                page::article_editor::update(module_msg, module_model, &mut orders.proxy(Msg::GotArticleEditorMsg));
            }
        },
    }
}

fn change_route_to<'a>(
    route: Option<route::Route<'a>>,
    model: &mut Model<'a>,
    orders:&mut Orders<Msg<'static>, GMsg>,
) {
    let mut session = || session::Session::from(model.take());
    match route {
        None => { *model = Model::NotFound(session()) },
        Some(route) => match route {
            route::Route::Root => {
                route::go_to(route::Route::Home, orders)
            },
            route::Route::Logout => {
                api::logout();
                orders.send_g_msg(GMsg::SessionChanged(None.into(), false));
                route::go_to(route::Route::Home, orders)
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
                *model = Model::Profile(init_page.model, username.into_owned());
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
    let app = seed::App::build(Model::default(), update, view)
        .routes(|url| {
            Msg::ChangedRoute(url.try_into().ok())
        })
        .g_msg_handler(g_msg_handler)
        .finish()
        .run();

    app.update(Msg::Init);
}