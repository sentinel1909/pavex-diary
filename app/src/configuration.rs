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
    pub assets: AssetsConfig,
}

// methods for the AppConfig type
impl AppConfig {
    pub fn templates_config(&self) -> &TemplateConfig {
        &self.templates
    }

    pub fn assets_config(&self) -> &AssetsConfig {
        &self.assets
    }
}

// struct type to represent the emplate storage configuration
#[derive(serde::Deserialize, Clone, Debug)]
pub struct TemplateConfig {
    pub dir: Cow<'static, str>,
}

// struct type to represent the assets storage configuration
#[derive(serde::Deserialize, Clone, Debug)]
pub struct AssetsConfig {
    pub dir: Cow<'static, str>,
}
