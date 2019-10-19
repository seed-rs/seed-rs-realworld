use super::ViewPage;

// ------ ------
//     View
// ------ ------

pub fn view<'a, Ms: Clone>() -> ViewPage<'a, Ms> {
    ViewPage::new("Blank", empty!())
}
