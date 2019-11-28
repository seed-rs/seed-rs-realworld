use crate::entity::Image;
use futures::Future;
use gloo_timers::future::TimeoutFuture;
use seed::prelude::*;

const SLOW_LOADING_THRESHOLD_MS: u32 = 500;

pub fn notify_on_slow_load<Ms>(msg: Ms, error_msg: Ms) -> impl Future<Item = Ms, Error = Ms> {
    TimeoutFuture::new(SLOW_LOADING_THRESHOLD_MS)
        .map(|_| msg)
        .map_err(|_| error_msg)
}

// ------ view functions ------

pub fn view_icon<Ms>() -> Node<Ms> {
    img![attrs! {
        At::Src => Image::loading().url(),
        At::Width => 64,
        At::Height => 64,
        At::Alt => "Loading..."
    }]
}

pub fn view_error<Ms>(subject: &str) -> Node<Ms> {
    div![format!("Error loading {}.", subject)]
}
