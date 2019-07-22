use crate::{article, username, GMsg};
use seed::prelude::*;
use std::{borrow::Cow, convert::TryFrom, fmt};
use tool::non_empty;

type Path<'a> = Vec<&'a str>;

#[derive(Clone)]
pub enum Route<'a> {
    Home,
    Root,
    Login,
    Logout,
    Register,
    Settings,
    Article(article::slug::Slug),
    Profile(Cow<'a, username::Username<'a>>),
    NewArticle,
    EditArticle(article::slug::Slug),
}

impl<'a> Route<'a> {
    pub fn path(&self) -> Path {
        use Route::*;
        match self {
            Home | Root => vec![],
            Login => vec!["login"],
            Logout => vec!["logout"],
            Register => vec!["register"],
            Settings => vec!["settings"],
            Article(slug) => vec!["article", slug.as_str()],
            Profile(username) => vec!["profile", username.as_str()],
            NewArticle => vec!["editor"],
            EditArticle(slug) => vec!["editor", slug.as_str()],
        }
    }
}

impl<'a> fmt::Display for Route<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/{}", self.path().join("/"))
    }
}

impl<'a> From<Route<'a>> for seed::Url {
    fn from(route: Route) -> Self {
        route.path().into()
    }
}

impl<'a> TryFrom<seed::Url> for Route<'a> {
    type Error = ();

    fn try_from(url: seed::Url) -> Result<Self, Self::Error> {
        let mut path = url.path.into_iter();

        match path.next().as_ref().map(String::as_str) {
            None | Some("") => Some(Route::Home),
            Some("login") => Some(Route::Login),
            Some("logout") => Some(Route::Logout),
            Some("settings") => Some(Route::Settings),
            Some("profile") => path
                .next()
                .filter(non_empty)
                .map(username::Username::from)
                .map(Cow::Owned)
                .map(Route::Profile),
            Some("register") => Some(Route::Register),
            Some("article") => path
                .next()
                .filter(non_empty)
                .map(article::slug::Slug::from)
                .map(Route::Article),
            Some("editor") => path
                .next()
                .filter(non_empty)
                .map(article::slug::Slug::from)
                .map(Route::EditArticle)
                .or_else(|| Some(Route::NewArticle)),
            _ => None,
        }
        .ok_or(())
    }
}

// Public helpers

pub fn go_to<Ms: 'static>(route: Route<'static>, orders: &mut impl Orders<Ms, GMsg>) {
    seed::push_route(route.clone());
    orders.send_g_msg(GMsg::RoutePushed(route));
}
