use super::ViewPage;

// View

pub fn view<Ms>() -> ViewPage<'static, Ms> {
    ViewPage::new("Blank", empty!())
}