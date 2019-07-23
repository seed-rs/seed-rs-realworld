use super::ViewPage;

// ------ ------
//     View
// ------ ------

pub fn view<'a, Ms>() -> ViewPage<'a, Ms> {
    ViewPage::new("Blank", empty!())
}
