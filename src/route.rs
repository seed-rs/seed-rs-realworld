use seed;

#[derive(Clone, Copy)]
pub enum Route {
}

//impl Default for Route {
//    fn default() -> Self {
//        Route::Home
//    }
//}

pub fn url_to_msg_with_route<Ms>(url: &seed::Url, msg_constructor: fn(Option<Route>) -> Ms) -> Ms  {
    msg_constructor(None)
}
