pub struct Image(String);

impl Image {
    pub fn new(filename: &str) -> Self {
        Self(format!("/assets/images/{}", filename))
    }

    pub fn url(&self) -> &str {
        &self.0
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn into_url(self) -> String {
        self.0
    }
}

// Images

pub fn error() -> Image {
    Image::new("error.jpg")
}

pub fn loading() -> Image {
    Image::new("loading.svg")
}

pub fn default_avatar() -> Image {
    Image::new("smiley-cyrus.jpg")
}
