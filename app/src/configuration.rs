use pavex::blueprint::Blueprint;
use pavex::t;
use std::borrow::Cow;

pub fn register(bp: &mut Blueprint) {
    bp.prebuilt(t!(self::AppConfig));
}

#[derive(serde::Deserialize, Debug, Clone)]
/// The configuration object holding all the values required
/// to configure the application.
pub struct AppConfig {
    pub templates: TemplateConfig,
}

// methods for the AppConfig type
impl AppConfig {
    pub fn templates_config(&self) -> &TemplateConfig {
        &self.templates
    }
}

// struct type to represent the Template storage configuration
#[derive(serde::Deserialize, Clone, Debug)]
pub struct TemplateConfig {
    pub dir: Cow<'static, str>
}
