#[derive(Debug, Clone)]
pub struct WindowInfo {
    pub uid: u64,
    pub title: Option<String>,
    pub app_name: Option<String>,
    pub app_class: Option<String>,
}

const WM_CLASS_GOOGLE_CHROME: &'static str = "Google-chrome";
const WM_CLASS_FIREFOX: &'static str = "firefox";

impl WindowInfo {
    pub fn build(uid: u64) -> Self {
        Self {
            uid,
            title: None,
            app_class: None,
            app_name: None
        }
    }

    pub fn with_title<T> (mut self, title: T) -> Self
        where T: Into<String> {
            self.title = Some(title.into());
            self
    }

    pub fn with_app_name<T> (mut self, app_name: T) -> Self
        where T: Into<String> {
            self.app_name = Some(app_name.into());
            self
    }

    pub fn with_app_class<T> (mut self, app_class: T) -> Self
        where T: Into<String> {
            self.app_class = Some(app_class.into());
            self
        }
}

impl PartialEq for WindowInfo {
    fn eq(&self, other: &Self) -> bool {
        match self.app_class {
            Some(ref wm_class) => {
                match wm_class.as_str() {
                    // Matching browser tabs
                    WM_CLASS_FIREFOX |
                    WM_CLASS_GOOGLE_CHROME =>
                        self.uid == other.uid &&
                        self.app_class == other.app_class &&
                        self.app_name == other.app_name &&
                        self.title == other.title,

                    // Matching everything else
                    _ => self.uid == other.uid,
                }
            }
            _ => false,
        }
    }
}
impl Eq for WindowInfo {}