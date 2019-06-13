use super::ViewPage;

// View

pub fn view<Ms>() -> ViewPage<Ms> {
    ViewPage {
        title: "".into(),
        content: empty!()
    }
}