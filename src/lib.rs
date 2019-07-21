#[macro_use]
extern crate seed;
use seed::prelude::*;
use std::convert::TryInto;
use helper::take;
use entity::{article, username};

mod coder;
mod entity;
mod helper;
mod loading;
mod logger;
mod page;
mod request;
mod route;
mod session;
mod storage;

// Model

enum Model<'a> {
    Redirect(session::Session),
    NotFound(session::Session),
    Home(page::home::Model),
    Settings(page::settings::Model),
    Login(page::login::Model),
    Register(page::register::Model),
    Profile(page::profile::Model<'a>, username::Username<'a>),
    Article(page::article::Model<'a>),
    ArticleEditor(page::article_editor::Model, Option<article::slug::Slug>)
}

impl<'a> Default for Model<'a> {
    fn default() -> Self {
        Model::Redirect(session::Session::default())
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

// Sink

pub enum GMsg {
    RoutePushed(route::Route<'static>),
    SessionChanged(session::Session)
}

fn sink<'a>(g_msg: GMsg, model: &mut Model<'a>, orders: &mut impl Orders<Msg<'static>, GMsg>) {
    if let GMsg::RoutePushed(ref route) = g_msg {
        orders.send_msg(Msg::RouteChanged(Some(route.clone())));
    }

    match model {
        Model::NotFound(_) | Model::Redirect(_) => {
            if let GMsg::SessionChanged(session) = g_msg {
                *model = Model::Redirect(session);
                route::go_to(route::Route::Home, orders);
            }
        },
        Model::Settings(model) => {
            page::settings::sink(g_msg, model, &mut orders.proxy(Msg::SettingsMsg));
        },
        Model::Home(model) => {
            page::home::sink(g_msg, model);
        },
        Model::Login(model) => {
            page::login::sink(g_msg, model, &mut orders.proxy(Msg::LoginMsg));
        },
        Model::Register(model) => {
            page::register::sink(g_msg, model, &mut orders.proxy(Msg::RegisterMsg));
        },
        Model::Profile(model, _) => {
            page::profile::sink(g_msg, model, &mut orders.proxy(Msg::ProfileMsg));
        },
        Model::Article(model) => {
            page::article::sink(g_msg, model, &mut orders.proxy(Msg::ArticleMsg));
        },
        Model::ArticleEditor(model, _) => {
            page::article_editor::sink(g_msg, model, &mut orders.proxy(Msg::ArticleEditorMsg));
        },
    }
}

// Update

enum Msg<'a> {
    RouteChanged(Option<route::Route<'a>>),
    HomeMsg(page::home::Msg),
    SettingsMsg(page::settings::Msg),
    LoginMsg(page::login::Msg),
    RegisterMsg(page::register::Msg),
    ProfileMsg(page::profile::Msg),
    ArticleMsg(page::article::Msg),
    ArticleEditorMsg(page::article_editor::Msg),
}

fn update<'a>(msg: Msg<'a>, model: &mut Model<'a>, orders: &mut impl Orders<Msg<'static>, GMsg>) {
    match msg {
        Msg::RouteChanged(route) => {
            change_model_by_route(route, model, orders);
        },
        Msg::HomeMsg(module_msg) => {
            if let Model::Home(module_model) = model {
                page::home::update(module_msg, module_model, &mut orders.proxy(Msg::HomeMsg));
            }
        },
        Msg::SettingsMsg(module_msg) => {
            if let Model::Settings(module_model) = model {
                page::settings::update(module_msg, module_model, &mut orders.proxy(Msg::SettingsMsg));
            }
        },
        Msg::LoginMsg(module_msg) => {
            if let Model::Login(module_model) = model {
                page::login::update(module_msg, module_model, &mut orders.proxy(Msg::LoginMsg));
            }
        },
        Msg::RegisterMsg(module_msg) => {
            if let Model::Register(module_model) = model {
                page::register::update(module_msg, module_model, &mut orders.proxy(Msg::RegisterMsg));
            }
        },
        Msg::ProfileMsg(module_msg) => {
            if let Model::Profile(module_model, _) = model {
                page::profile::update(module_msg, module_model, &mut orders.proxy(Msg::ProfileMsg));
            }
        },
        Msg::ArticleMsg(module_msg) => {
            if let Model::Article(module_model) = model {
                page::article::update(module_msg, module_model, &mut orders.proxy(Msg::ArticleMsg));
            }
        },
        Msg::ArticleEditorMsg(module_msg) => {
            if let Model::ArticleEditor(module_model, _) = model {
                page::article_editor::update(module_msg, module_model, &mut orders.proxy(Msg::ArticleEditorMsg));
            }
        },
    }
}

fn change_model_by_route<'a>(
    route: Option<route::Route<'a>>,
    model: &mut Model<'a>,
    orders:&mut impl Orders<Msg<'static>, GMsg>,
) {
    let mut session = || session::Session::from(take(model));
    match route {
        None => { *model = Model::NotFound(session()) },
        Some(route) => match route {
            route::Route::Root => {
                route::go_to(route::Route::Home, orders)
            },
            route::Route::Logout => {
                storage::delete_app_data();
                orders.send_g_msg(GMsg::SessionChanged(None.into()));
                route::go_to(route::Route::Home, orders)
            },
            route::Route::NewArticle => {
                *model = Model::ArticleEditor(
                    page::article_editor::init_new(session()),
                    None
                );
            },
            route::Route::EditArticle(slug) => {
                *model = Model::ArticleEditor(
                    page::article_editor::init_edit(
                        session(), slug.clone(), &mut orders.proxy(Msg::ArticleEditorMsg)
                    ),
                    Some(slug)
                );
            },
            route::Route::Settings => {
                *model = Model::Settings(page::settings::init(
                    session(), &mut orders.proxy(Msg::SettingsMsg)
                ));
            },
            route::Route::Home => {
                *model = Model::Home(
                    page::home::init(session(), &mut orders.proxy(Msg::HomeMsg))
                );
            },
            route::Route::Login => {
                *model = Model::Login(
                    page::login::init(session())
                );
            },
            route::Route::Register => {
                *model = Model::Register(
                    page::register::init(session())
                );
            },
            route::Route::Profile(username) => {
                *model = Model::Profile(
                    page::profile::init(
                        session(), username.to_static(), &mut orders.proxy(Msg::ProfileMsg)
                    ),
                    username.into_owned()
                );
            },
            route::Route::Article(slug) => {
                *model = Model::Article(
                    page::article::init(session(), &slug, &mut orders.proxy(Msg::ArticleMsg))
                );
            },
        }
    };
}

// View

fn view<'a>(model: &Model) -> impl View<Msg<'static>> {
    match model {
        Model::Redirect(session) => {
            page::view(
                page::Page::Other,
                page::blank::view(),
                session.viewer(),
            )
        },
        Model::NotFound(session) => {
            page::view(
                page::Page::Other,
                page::not_found::view(),
                session.viewer(),
            )
        },
        Model::Settings(model) => {
            page::view(
                page::Page::Settings,
                page::settings::view(model),
                model.session().viewer(),
            ).map_message(Msg::SettingsMsg)
        },
        Model::Home(model) => {
            page::view(
                page::Page::Home,
                page::home::view(model),
                model.session().viewer(),
            ).map_message(Msg::HomeMsg)
        },
        Model::Login(model) => {
            page::view(
                page::Page::Login,
                page::login::view(model),
                model.session().viewer(),
            ).map_message(Msg::LoginMsg)
        },
        Model::Register(model) => {
            page::view(
                page::Page::Register,
                page::register::view(model),
                model.session().viewer(),
            ).map_message(Msg::RegisterMsg)
        },
        Model::Profile(model, username) => {
            page::view(
                page::Page::Profile(username),
                page::profile::view(model),
                model.session().viewer(),
            ).map_message(Msg::ProfileMsg)
        },
        Model::Article(model) => {
            page::view(
                page::Page::Other,
                page::article::view(model),
                model.session().viewer(),
            ).map_message(Msg::ArticleMsg)
        },
        Model::ArticleEditor(model, None) => {
            page::view(
                page::Page::NewArticle,
                page::article_editor::view(model),
                model.session().viewer(),
            ).map_message(Msg::ArticleEditorMsg)
        },
        Model::ArticleEditor(model, Some(_)) => {
            page::view(
                page::Page::Other,
                page::article_editor::view(model),
                model.session().viewer(),
            ).map_message(Msg::ArticleEditorMsg)
        },
    }
}

// Init

fn init(url: Url, orders: &mut impl Orders<Msg<'static>, GMsg>) -> Model<'static> {
    orders.send_msg(Msg::RouteChanged(url.try_into().ok()));
    Model::Redirect(storage::load_viewer().into())
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(init, update, view)
        .routes(|url| {
            Msg::RouteChanged(url.try_into().ok())
        })
        .sink(sink)
        .finish()
        .run();
}