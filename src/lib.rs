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

enum Model {
    Empty,
    Redirect(session::Session),
    NotFound(session::Session),
    Home(page::home::Model),
    Settings(page::settings::Model),
    Login(page::login::Model),
    Register(page::register::Model),
    Profile(username::Username, page::profile::Model),
    Article(page::article::Model),
    Editor(Option<article::slug::Slug>, page::article_editor::Model)
}

impl From<Model> for session::Session {
    fn from(model: Model) -> session::Session {
        match model {
            Model::Empty => None.into(),
            Model::Redirect(session) => session,
            Model::NotFound(session) => session,
            Model::Home(model) => model.into(),
            Model::Settings(model) => model.into(),
            Model::Login(model) => model.into(),
            Model::Register(model) => model.into(),
            Model::Profile(_, model) => model.into(),
            Model::Article(model) => model.into(),
            Model::Editor(_, model) => model.into(),
        }
    }
}

// Update

enum Msg {
    ChangedRoute(Option<route::Route>)
}

fn update(msg: Msg, model: &mut Model, _: &mut Orders<Msg>) {
    match msg {
        Msg::ChangedRoute(route) => change_route_to(route, model),
    }
}

fn change_route_to(route: Option<route::Route>, model: &mut Model) {
    let old_model = std::mem::replace(model, Model::Empty);
    let session = session::Session::from(old_model);

    let new_model = match route {
        None => Model::NotFound(session),
        Some(route) => match route {
            // @TODO
            _ => Model::NotFound(session)
        }
    };
    *model = new_model;
}

// View

fn view(model: &Model) -> impl ElContainer<Msg> {
    let viewer = None;
    // @TODO refactor? How to get rid of clone?
    match model {
        Model::Empty => vec![],
        Model::Redirect(_) => page::Page::Other.view(viewer, page::blank::view()),
        Model::NotFound(_) => page::Page::Other.view(viewer, page::not_found::view()),
        Model::Settings(_) => page::Page::Settings.view(viewer,page::settings::view()),
        Model::Home(_) => page::Page::Settings.view(viewer,page::home::view()),
        Model::Login(_) => page::Page::Settings.view(viewer,page::login::view()),
        Model::Register(_) => page::Page::Settings.view(viewer,page::register::view()),
        Model::Profile(username, _) => page::Page::Profile(username.clone()).view(viewer,page::profile::view()),
        Model::Article(_) => page::Page::Other.view(viewer,page::article::view()),
        Model::Editor(None, _) => page::Page::NewArticle.view(viewer,page::article_editor::view()),
        Model::Editor(Some(_), _) => page::Page::Other.view(viewer,page::article_editor::view()),
    }
}

#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::Empty, update, view)
        .routes(|url| route::url_to_msg_with_route(url, Msg::ChangedRoute))
        .finish()
        .run();
}