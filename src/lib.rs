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

// Subscriptions

pub struct Subs(Vec<SubMsg>);

impl Default for Subs {
    fn default() -> Self {
        Subs(Vec::new())
    }
}

impl Subs {
    pub fn add(&mut self, sub_msg: SubMsg) {
        self.0.push(sub_msg);
    }
}

#[derive(Clone)]
pub enum SubMsg {
    RoutePushed(route::Route<'static>),
    SessionChanged(session::Session, HasSessionChangedOnInit)
}

pub type HasSessionChangedOnInit = bool;

fn subscriptions<'a>(sub_msg: SubMsg, model: &mut Model<'a>, orders: &mut Orders<Msg<'static>>) {
    match sub_msg.clone() {
        SubMsg::RoutePushed(route) => {
            orders.send_msg(Msg::ChangedRoute(Some(route)));
        },
        _ => ()
    }

    match model {
        Model::NotFound(_) => {},
        Model::Redirect(_) => {
            if let SubMsg::SessionChanged(session, false) = sub_msg {
                orders.send_msg(Msg::GotSession(session));
            }
        },
        Model::Settings(model) => {
            page::settings::subscriptions(sub_msg, &model)
                .map(|module_msg| orders.send_msg(Msg::GotSettingsMsg(module_msg)));
        },
        Model::Home(model) => {
            page::home::subscriptions(sub_msg, &model)
                .map(|module_msg| orders.send_msg(Msg::GotHomeMsg(module_msg)));
        },
        Model::Login(model) => {
            page::login::subscriptions(sub_msg, &model)
                .map(|module_msg| orders.send_msg(Msg::GotLoginMsg(module_msg)));
        },
        Model::Register(model) => {
            page::register::subscriptions(sub_msg, &model)
                .map(|module_msg| orders.send_msg(Msg::GotRegisterMsg(module_msg)));
        },
        Model::Profile(model, _) => {
            page::profile::subscriptions(sub_msg, &model)
                .map(|module_msg| orders.send_msg(Msg::GotProfileMsg(module_msg)));
        },
        Model::Article(model) => {
            page::article::subscriptions(sub_msg, &model)
            .map(|module_msg| orders.send_msg(Msg::GotArticleMsg(module_msg)));
        },
        Model::ArticleEditor(model, _) => {
            page::article_editor::subscriptions(sub_msg, &model)
            .map(|module_msg| orders.send_msg(Msg::GotArticleEditorMsg(module_msg)));
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

fn update<'a>(msg: Msg<'a>, model: &mut Model<'a>, orders: &mut Orders<Msg<'static>>) {
    match msg {
        Msg::Init => {
            let session = session::Session::from(api::load_viewer());
            subscriptions(SubMsg::SessionChanged(session, true), model, orders);
        },
        Msg::ChangedRoute(route) => {
            let mut subs = Subs::default();
            change_route_to(route, model, orders, &mut subs);
            for sub_msg in subs.0 {
                subscriptions(sub_msg, model, orders)
            }
        },
        Msg::GotSession(session) => {
            if let Model::Redirect(_) = model {
                *model = Model::Redirect(session);
                let mut subs = Subs::default();
                route::go_to(route::Route::Home, &mut subs);
                for sub_msg in subs.0 {
                    subscriptions(sub_msg, model, orders)
                }
            }
        }
        Msg::GotHomeMsg(module_msg) => {
            if let Model::Home(module_model) = model {
                let mut module_orders = Orders::default();
                let mut subs = Subs::default();
                page::home::update(module_msg, module_model, &mut module_orders, &mut subs);
                *orders = module_orders.map_message(Msg::GotHomeMsg);
                for sub_msg in subs.0 {
                    subscriptions(sub_msg, model, orders)
                }
            }
        },
        Msg::GotSettingsMsg(module_msg) => {
            if let Model::Settings(module_model) = model {
                let mut module_orders = Orders::default();
                let mut subs = Subs::default();
                page::settings::update(module_msg, module_model, &mut module_orders, &mut subs);
                *orders = module_orders.map_message(Msg::GotSettingsMsg);
                for sub_msg in subs.0 {
                    subscriptions(sub_msg, model, orders)
                }
            }
        },
        Msg::GotLoginMsg(module_msg) => {
            if let Model::Login(module_model) = model {
                let mut module_orders = Orders::default();
                let mut subs = Subs::default();
                page::login::update(module_msg, module_model, &mut module_orders, &mut subs);
                *orders = module_orders.map_message(Msg::GotLoginMsg);
                for sub_msg in subs.0 {
                    subscriptions(sub_msg, model, orders)
                }
            }
        },
        Msg::GotRegisterMsg(module_msg) => {
            if let Model::Register(module_model) = model {
                let mut module_orders = Orders::default();
                let mut subs = Subs::default();
                page::register::update(module_msg, module_model, &mut module_orders, &mut subs);
                *orders = module_orders.map_message(Msg::GotRegisterMsg);
                for sub_msg in subs.0 {
                    subscriptions(sub_msg, model, orders)
                }
            }
        },
        Msg::GotProfileMsg(module_msg) => {
            if let Model::Profile(module_model, _) = model {
                let mut module_orders = Orders::default();
                let mut subs = Subs::default();
                page::profile::update(module_msg, module_model, &mut module_orders, &mut subs);
                *orders = module_orders.map_message(Msg::GotProfileMsg);
                for sub_msg in subs.0 {
                    subscriptions(sub_msg, model, orders)
                }
            }
        },
        Msg::GotArticleMsg(module_msg) => {
            if let Model::Article(module_model) = model {
                let mut module_orders = Orders::default();
                let mut subs = Subs::default();
                page::article::update(module_msg, module_model, &mut module_orders, &mut subs);
                *orders = module_orders.map_message(Msg::GotArticleMsg);
                for sub_msg in subs.0 {
                    subscriptions(sub_msg, model, orders)
                }
            }
        },
        Msg::GotArticleEditorMsg(module_msg) => {
            if let Model::ArticleEditor(module_model, _) = model {
                let mut module_orders = Orders::default();
                let mut subs = Subs::default();
                page::article_editor::update(module_msg, module_model, &mut module_orders, &mut subs);
                *orders = module_orders.map_message(Msg::GotArticleEditorMsg);
                for sub_msg in subs.0 {
                    subscriptions(sub_msg, model, orders)
                }
            }
        },
    }
}

fn change_route_to<'a>(
    route: Option<route::Route<'a>>,
    model: &mut Model<'a>,
    orders:&mut Orders<Msg<'static>>,
    subs: &mut Subs,
) {
    let mut session = || session::Session::from(model.take());
    match route {
        None => { *model = Model::NotFound(session()) },
        Some(route) => match route {
            route::Route::Root => {
                route::go_to(route::Route::Home, subs)
            },
            route::Route::Logout => {
                api::logout();
                subs.add(SubMsg::SessionChanged(None.into(), false));
                route::go_to(route::Route::Home, subs)
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
        .finish()
        .run();

    app.update(Msg::Init);
}