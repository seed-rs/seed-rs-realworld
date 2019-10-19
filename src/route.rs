use crate::entity::{Slug, Username};
use crate::GMsg;
use seed::prelude::*;
use std::{borrow::Cow, convert::TryFrom, fmt};
use tool::non_empty;

pub fn go_to<Ms: 'static + Clone>(route: Route<'static>, orders: &mut impl Orders<Ms, GMsg>) {
    seed::push_route(route.clone());
    orders.send_g_msg(GMsg::RoutePushed(route));
}

// ------ Route ------

#[derive(Clone)]
pub enum Route<'a> {
    Home,
    Root,
    Login,
    Logout,
    Register,
    Settings,
    Article(Slug),
    Profile(Cow<'a, Username<'a>>),
    NewArticle,
    EditArticle(Slug),
}

impl<'a> Route<'a> {
    pub fn path(&self) -> Vec<&str> {
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
                .map(Username::from)
                .map(Cow::Owned)
                .map(Route::Profile),
            Some("register") => Some(Route::Register),
            Some("article") => path
                .next()
                .filter(non_empty)
                .map(Slug::from)
                .map(Route::Article),
            Some("editor") => path
                .next()
                .filter(non_empty)
                .map(Slug::from)
                .map(Route::EditArticle)
                .or_else(|| Some(Route::NewArticle)),
            _ => None,
        }
        .ok_or(())
    }
}

// ====== ====== TESTS ====== ======

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::convert::TryInto;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn home_route_test() {
        // ====== ARRANGE ======
        let url: seed::Url = "".to_string().into();

        // ====== ACT ======
        let route = url.try_into();

        // ====== ASSERT ======
        assert!(if let Ok(Route::Home) = route {
            true
        } else {
            false
        })
    }

    #[wasm_bindgen_test]
    fn home_route_trailing_slash_test() {
        // ====== ARRANGE ======
        let url: seed::Url = "/".to_string().into();

        // ====== ACT ======
        let route = url.try_into();

        // ====== ASSERT ======
        assert!(if let Ok(Route::Home) = route {
            true
        } else {
            false
        })
    }

    #[wasm_bindgen_test]
    fn login_route_test() {
        // ====== ARRANGE ======
        let url: seed::Url = "/login".to_string().into();

        // ====== ACT ======
        let route = url.try_into();

        // ====== ASSERT ======
        assert!(if let Ok(Route::Login) = route {
            true
        } else {
            false
        })
    }

    #[wasm_bindgen_test]
    fn logout_route_test() {
        // ====== ARRANGE ======
        let url: seed::Url = "/logout".to_string().into();

        // ====== ACT ======
        let route = url.try_into();

        // ====== ASSERT ======
        assert!(if let Ok(Route::Logout) = route {
            true
        } else {
            false
        })
    }

    #[wasm_bindgen_test]
    fn settings_route_test() {
        // ====== ARRANGE ======
        let url: seed::Url = "/settings".to_string().into();

        // ====== ACT ======
        let route = url.try_into();

        // ====== ASSERT ======
        assert!(if let Ok(Route::Settings) = route {
            true
        } else {
            false
        })
    }

    #[wasm_bindgen_test]
    fn profile_route_test() {
        // ====== ARRANGE ======
        let url: seed::Url = "/profile/john".to_string().into();

        // ====== ACT ======
        let route = url.try_into();

        // ====== ASSERT ======
        assert!(if let Ok(Route::Profile(username)) = route {
            username.as_str() == "john"
        } else {
            false
        })
    }

    #[wasm_bindgen_test]
    fn register_route_test() {
        // ====== ARRANGE ======
        let url: seed::Url = "/register".to_string().into();

        // ====== ACT ======
        let route = url.try_into();

        // ====== ASSERT ======
        assert!(if let Ok(Route::Register) = route {
            true
        } else {
            false
        })
    }

    #[wasm_bindgen_test]
    fn article_route_test() {
        // ====== ARRANGE ======
        let url: seed::Url = "/article/my_article".to_string().into();

        // ====== ACT ======
        let route = url.try_into();

        // ====== ASSERT ======
        assert!(if let Ok(Route::Article(slug)) = route {
            slug.as_str() == "my_article"
        } else {
            false
        })
    }

    #[wasm_bindgen_test]
    fn edit_article_route_test() {
        // ====== ARRANGE ======
        let url: seed::Url = "/editor/my_article".to_string().into();

        // ====== ACT ======
        let route = url.try_into();

        // ====== ASSERT ======
        assert!(if let Ok(Route::EditArticle(slug)) = route {
            slug.as_str() == "my_article"
        } else {
            false
        })
    }

    #[wasm_bindgen_test]
    fn new_article_route_test() {
        // ====== ARRANGE ======
        let url: seed::Url = "/editor".to_string().into();

        // ====== ACT ======
        let route = url.try_into();

        // ====== ASSERT ======
        assert!(if let Ok(Route::NewArticle) = route {
            true
        } else {
            false
        })
    }

    #[wasm_bindgen_test]
    fn invalid_route_test() {
        // ====== ARRANGE ======
        let url: seed::Url = "/unknown_url".to_string().into();

        // ====== ACT ======
        let route: Result<Route, ()> = url.try_into();

        // ====== ASSERT ======
        assert!(route.is_err())
    }
}
