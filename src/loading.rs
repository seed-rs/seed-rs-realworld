use crate::entity::Image;
use gloo_timers::future::TimeoutFuture;
use seed::prelude::*;

const SLOW_LOADING_THRESHOLD_MS: u32 = 500;

pub async fn notify_on_slow_load<Ms>(msg: Ms) -> Result<Ms, Ms> {
    TimeoutFuture::new(SLOW_LOADING_THRESHOLD_MS).await;
    Ok(msg)
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
