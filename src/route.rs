use seed;
use crate::{username, article};

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

pub fn url_to_msg_with_route<'a, MsC, Ms>(url: &seed::Url, msg_constructor: MsC) -> Ms
where MsC: Fn(Option<Route<'a>>) -> Ms
{
    let mut path = url.path.to_owned().into_iter();
    let route = match path.next() {
        None=> Some(Route::Home),
        Some(path_item) => match path_item.as_str() {
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
        }
    };
    msg_constructor(route)
}
