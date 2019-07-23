static IMAGES_BASE_URL: &str = "/assets/images";

pub struct Image(String);

impl Image {
    pub fn new(filename: &str) -> Self {
        Self(format!("{}/{}", IMAGES_BASE_URL, filename))
    }

    pub fn url(&self) -> &str {
        &self.0
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn into_url(self) -> String {
        self.0
    }

    // -- Images --

    pub fn error() -> Self {
        Self::new("error.jpg")
    }

    pub fn loading() -> Self {
        Self::new("loading.svg")
    }

    pub fn default_avatar() -> Self {
        Self::new("smiley-cyrus.jpg")
    }
}
