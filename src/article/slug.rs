pub struct Slug(String);

impl From<String> for Slug {
    fn from(string: String) -> Slug {
        Slug(string)
    }
}