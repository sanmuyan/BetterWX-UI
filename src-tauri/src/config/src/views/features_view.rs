use crate::features::Features;
use serde::Serialize;

#[derive(Debug, Default, Clone, Serialize)]

pub struct FeaturesView(pub Vec<String>);

impl From<&Features> for FeaturesView {
    fn from(featrues: &Features) -> Self {
        let mut views = Vec::new();
        featrues.0.iter().for_each(|feature| {
            if feature.method == "patch" && feature.status {
                views.push(feature.code.clone());
            }
        });
        Self(views)
    }
}
