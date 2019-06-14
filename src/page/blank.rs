use super::ViewPage;

// View

pub fn view<Ms>() -> ViewPage<'static, Ms> {
    ViewPage {
        title: "",
        content: empty!()
    }
}