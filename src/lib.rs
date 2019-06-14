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
    Profile(username::Username<'a>, page::profile::Model<'a>),
    Article(page::article::Model<'a>),
    ArticleEditor(Option<article::slug::Slug<'a>>, page::article_editor::Model<'a>)
}

impl<'a> Model<'a> {
    pub fn take(&mut self) -> Model<'a> {
        std::mem::replace(self, Model::None)
    }
}

impl<'a> Default for Model<'a> {
    fn default() -> Self {
        Model::None
    }
}

impl<'a> From<Model<'a>> for session::Session<'a> {
    fn from(model: Model<'a>) -> session::Session<'a> {
        match model {
            Model::None => None.into(),
            Model::Redirect(session) => session,
            Model::NotFound(session) => session,
            Model::Home(model) => model.into(),
            Model::Settings(model) => model.into(),
            Model::Login(model) => model.into(),
            Model::Register(model) => model.into(),
            Model::Profile(_, model) => model.into(),
            Model::Article(model) => model.into(),
            Model::ArticleEditor(_, model) => model.into(),
        }
    }
}

// Update

enum Msg<'a> {
    ChangedRoute(Option<route::Route<'a>>),
}

fn update<'a>(msg: Msg<'a>, model: &mut Model<'a>, _: &mut Orders<Msg>) {
    match msg {
        Msg::ChangedRoute(route) => change_route_to(route, model),
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
                *model = Model::ArticleEditor(None, page::article_editor::init(model.take().into()))
            },
            route::Route::EditArticle(slug) => {
                *model = Model::ArticleEditor(Some(slug), page::article_editor::init(model.take().into()))
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
                *model = Model::Profile(username, page::profile::init(model.take().into()))
            },
            route::Route::Article(_) => {
                *model = Model::Article(page::article::init(model.take().into()))
            },
        }
    };
}

// View

fn view<'a>(model: &Model) -> impl ElContainer<Msg<'a>> {
    let viewer = None;
    match model {
        Model::None => vec![],
        Model::Redirect(_) => page::Page::Other.view(viewer, page::blank::view()),
        Model::NotFound(_) => page::Page::Other.view(viewer, page::not_found::view()),
        Model::Settings(_) => page::Page::Settings.view(viewer,page::settings::view()),
        Model::Home(_) => page::Page::Settings.view(viewer,page::home::view()),
        Model::Login(_) => page::Page::Settings.view(viewer,page::login::view()),
        Model::Register(_) => page::Page::Settings.view(viewer,page::register::view()),
        Model::Profile(username, _) => page::Page::Profile(username).view(viewer,page::profile::view()),
        Model::Article(_) => page::Page::Other.view(viewer,page::article::view()),
        Model::ArticleEditor(None, _) => page::Page::NewArticle.view(viewer,page::article_editor::view()),
        Model::ArticleEditor(Some(_), _) => page::Page::Other.view(viewer,page::article_editor::view()),
    }
}

#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::default(), update, view)
        .routes(|url| route::url_to_msg_with_route(url, Msg::ChangedRoute))
        .finish()
        .run();
}