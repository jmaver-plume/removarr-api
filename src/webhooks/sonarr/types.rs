use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub cover_type: String,
    pub remote_url: Option<String>,
}

pub fn get_best_image_url(images: &Option<Vec<Image>>) -> Option<String> {
    images.as_ref()?.iter()
        .find(|img| img.cover_type == "poster")
        .and_then(|img| img.remote_url.clone())
}

