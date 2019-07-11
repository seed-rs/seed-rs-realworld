use seed::prelude::*;
use seed::gloo_timers::future::TimeoutFuture;
use futures::Future;
use crate::asset;

const SLOW_LOADING_THRESHOLD_MS: u32 = 500;

pub fn slow_threshold<Ms>(msg: Ms, error_msg: Ms) -> impl Future<Item = Ms, Error = Ms> {
    TimeoutFuture::new(SLOW_LOADING_THRESHOLD_MS)
        .map(|_| msg)
        .map_err(|_| error_msg)
}

pub fn icon<Ms>() -> Node<Ms> {
    img![
        attrs!{
            At::Src => asset::loading().url(),
            At::Width => 64,
            At::Height => 64,
            At::Alt => "Loading..."
        }
    ]
}

pub fn error<Ms>(subject: &str) -> Node<Ms> {
    div![
        format!("Error loading {}.", subject)
    ]
}
