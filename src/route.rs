use seed;
use crate::{username, article};

pub enum Route {
    Home,
    Root,
    Login,
    Logout,
    Register,
    Settings,
    Article(article::slug::Slug),
    Profile(username::Username),
    NewArticle,
    EditArticle(article::slug::Slug)
}

pub fn url_to_msg_with_route<Ms>(url: &seed::Url, msg_constructor: fn(Option<Route>) -> Ms) -> Ms  {
    // @TODO: we need into_iter? (is iter not enough?)
    let mut path = url.path.clone().into_iter();
    let route = match path.next().unwrap().as_str() {
        "" => Some(Route::Home),
        "login" => Some(Route::Login),
        "logout" => Some(Route::Logout),
        "settings" => Some(Route::Settings),
        "profile" => {
            path
                .next()
                .map(username::Username::from)
                .map(Route::Profile)
        },
        "register" => Some(Route::Register),
        "article" => {
            path
                .next()
                .map(article::slug::Slug::from)
                .map(Route::Article)
        },
        "editor" => {
            path.next()
                .map(article::slug::Slug::from)
                .map(Route::EditArticle)
                .or_else(|| Some(Route::NewArticle))
        },
        _ => None,
    };
    msg_constructor(route)
}
