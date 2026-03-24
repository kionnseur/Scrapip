pub mod builder;

#[derive(Default)]

pub struct Views {
    header: String,
    body: String,
    footer: String,
    theme: String,
    scripts: String,
}
impl Views {
    pub fn new(
        header: String,
        body: String,
        footer: String,
        theme: String,
        scripts: String,
    ) -> Self {
        Self {
            header,
            body,
            footer,
            theme,
            scripts,
        }
    }
}

impl Into<String> for Views {
    fn into(self) -> String {
        let header = self.header;
        let body = self.body;
        let footer = self.footer;
        let theme = self.theme;
        let scripts = self.scripts;

        format!("{}{}{}{}{}", theme, header, body, footer, scripts)
    }
}

impl Into<String> for &Views {
    fn into(self) -> String {
        let header = &self.header;
        let body = &self.body;
        let footer = &self.footer;
        let theme = &self.theme;
        let scripts = &self.scripts;

        format!("{}{}{}{}{}", theme, header, body, footer, scripts)
    }
}
