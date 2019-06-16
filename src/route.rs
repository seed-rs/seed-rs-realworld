use seed;
use crate::{username, article};
use tool::non_empty;

pub enum Route<'a> {
    Home,
    Root,
    Login,
    Logout,
    Register,
    Settings,
    Article(article::slug::Slug<'a>),
    Profile(username::Username<'a>),
    NewArticle,
    EditArticle(article::slug::Slug<'a>)
}

pub fn url_to_msg_with_route<'a, MsC, Ms>(url: seed::Url, msg_constructor: MsC) -> Ms
where MsC: Fn(Option<Route<'a>>) -> Ms
{
    let mut path = url.path.into_iter();
    let route = match path.next().as_ref().map(String::as_str) {
        None | Some("") => Some(Route::Home),
        Some("login") => Some(Route::Login),
        Some("logout") => Some(Route::Logout),
        Some("settings") => Some(Route::Settings),
        Some("profile") => {
            path
                .next()
                .filter(non_empty)
                .map(username::Username::from)
                .map(Route::Profile)
        },
        Some("register") => Some(Route::Register),
        Some("article") => {
            path
                .next()
                .filter(non_empty)
                .map(article::slug::Slug::from)
                .map(Route::Article)
        },
        Some("editor") => {
            path.next()
                .filter(non_empty)
                .map(article::slug::Slug::from)
                .map(Route::EditArticle)
                .or_else(|| Some(Route::NewArticle))
        },
        _ => None,
    };
    msg_constructor(route)
}
