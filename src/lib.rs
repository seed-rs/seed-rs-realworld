#[macro_use]
extern crate seed;
use seed::prelude::*;

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
    // @TODO Browser.UrlRequest?
    ClickedLink,
    GotHomeMsg(page::home::Msg),
    GotSettingsMsg(page::settings::Msg),
    GotLoginMsg(page::login::Msg),
    GotRegisterMsg(page::register::Msg),
    GotProfileMsg(page::profile::Msg),
    GotArticleMsg(page::article::Msg),
    GotArticleEditorMsg(page::article_editor::Msg),
    GotSession(session::Session<'a>),
}

fn update<'a>(msg: Msg<'a>, model: &mut Model<'a>, orders: &mut Orders<Msg<'static>>) {
    match msg {
        Msg::ChangedRoute(route) => change_route_to(route, model),
        // @TODOs
        Msg::ClickedLink => (),
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
            if let Model::Profile(model, username) = model {
                *orders = call_update(page::profile::update, sub_msg, model).map_message(Msg::GotProfileMsg)
            }
        },
        Msg::GotArticleMsg(sub_msg) => {
            if let Model::Article(model) = model {
                *orders = call_update(page::article::update, sub_msg, model).map_message(Msg::GotArticleMsg)
            }
        },
        Msg::GotArticleEditorMsg(sub_msg) => {
            if let Model::ArticleEditor(model, slug) = model {
                *orders = call_update(page::article_editor::update, sub_msg, model).map_message(Msg::GotArticleEditorMsg)
            }
        },
        Msg::GotSession(session) => (),
    }
}

fn change_route_to<'a>(route: Option<route::Route<'a>>, model: &mut Model<'a>) {
    match route {
        None => { *model = Model::NotFound(model.take().into()) },
        Some(route) => match route {
            route::Route::Root => {
                *model = Model::Home(page::home::init(model.take().into()))
            },
            route::Route::Logout => (),
            route::Route::NewArticle => {
                *model = Model::ArticleEditor(page::article_editor::init(model.take().into()), None)
            },
            route::Route::EditArticle(slug) => {
                *model = Model::ArticleEditor(page::article_editor::init(model.take().into()), Some(slug))
            },
            route::Route::Settings => {
                *model = Model::Settings(page::settings::init(model.take().into()))
            },
            route::Route::Home => {
                *model = Model::Home(page::home::init(model.take().into()))
            },
            route::Route::Login => {
                *model = Model::Login(page::login::init(model.take().into()))
            },
            route::Route::Register => {
                *model = Model::Register(page::register::init(model.take().into()))
            },
            route::Route::Profile(username) => {
                *model = Model::Profile(page::profile::init(model.take().into()), username)
            },
            route::Route::Article(_) => {
                *model = Model::Article(page::article::init(model.take().into()))
            },
        }
    };
}

// View

fn view<'a>(model: &Model) -> impl ElContainer<Msg<'a>> {
    let viewer = model.session().and_then(session::Session::viewer);
    match model {
        Model::None => vec![],
        Model::Redirect(_) => page::Page::Other.view(viewer, page::blank::view()),
        Model::NotFound(_) => page::Page::Other.view(viewer, page::not_found::view()),
        Model::Settings(_) => page::Page::Settings.view(viewer,page::settings::view()),
        Model::Home(_) => page::Page::Settings.view(viewer,page::home::view()),
        Model::Login(_) => page::Page::Settings.view(viewer,page::login::view()),
        Model::Register(_) => page::Page::Settings.view(viewer,page::register::view()),
        Model::Profile(_, username) => page::Page::Profile(username).view(viewer,page::profile::view()),
        Model::Article(_) => page::Page::Other.view(viewer,page::article::view()),
        Model::ArticleEditor(_, None) => page::Page::NewArticle.view(viewer,page::article_editor::view()),
        Model::ArticleEditor(_, Some(_)) => page::Page::Other.view(viewer,page::article_editor::view()),
    }
}

#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::default(), update, view)
        .routes(|url| route::url_to_msg_with_route(url, Msg::ChangedRoute))
        .finish()
        .run();
}