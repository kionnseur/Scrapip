use crate::views::Views;
pub trait Builder {
    type Output;

    fn set_header(&mut self, header: &str) -> &mut Self;
    fn set_body(&mut self, body: &str) -> &mut Self;
    fn set_footer(&mut self, footer: &str) -> &mut Self;
    fn set_style(&mut self, theme: &str) -> &mut Self;
    fn set_script(&mut self, scripts: &str) -> &mut Self;
    fn build(&self) -> Self::Output;
}

#[derive(Default)]
pub struct ViewsBuilder {
    header: Option<String>,
    body: Option<String>,
    footer: Option<String>,
    style: Option<String>,
    script: Option<String>,
}

impl Builder for ViewsBuilder {
    type Output = Views;

    fn set_header(&mut self, header: &str) -> &mut Self {
        self.header = Some(header.to_string());
        self
    }

    fn set_body(&mut self, body: &str) -> &mut Self {
        self.body = Some(body.to_string());
        self
    }

    fn set_footer(&mut self, footer: &str) -> &mut Self {
        self.footer = Some(footer.to_string());
        self
    }

    fn set_style(&mut self, theme: &str) -> &mut Self {
        self.style = Some(theme.to_string());
        self
    }

    fn set_script(&mut self, scripts: &str) -> &mut Self {
        self.script = Some(scripts.to_string());
        self
    }

    fn build(&self) -> Self::Output {
        Views::new(
            self.header.clone().expect("Header is required"),
            self.body.clone().expect("Body is required"),
            self.footer.clone().expect("Footer is required"),
            self.style.clone().expect("Style is required"),
            self.script.clone().expect("Script is required"),
        )
    }
}
