pub mod dto;
pub mod entities;
pub mod error;

use std::env;

use crate::{
    dto::{list::ListResultDTO, search::SearchResultDTO, thumbnail::Size},
    entities::{galleryinfo::GalleryInfo, info::Info, resolved_image::ResolvedImage, tags::Tags},
    error::DelphiniumError,
};

const API_PREFIX: &str = "/api/hitomi";

pub struct Delphinium {
    base_url: String,
    client: reqwest::Client,
}

impl Delphinium {
    pub fn new(base_url: String, client: reqwest::Client) -> Self {
        Self { base_url, client }
    }

    pub fn with_default_reqwest_client(base_url: String) -> Self {
        let client = reqwest::Client::new();
        Self { base_url, client }
    }

    pub async fn request<T: serde::de::DeserializeOwned>(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        json_body: Option<serde_json::Value>,
    ) -> Result<T, DelphiniumError> {
        let url = format!("{}{}{}", self.base_url, API_PREFIX, endpoint);

        let request_builder = self.client.request(method, &url).header(
            "User-Agent",
            format!("Delphinium/{}", env!("CARGO_PKG_VERSION")),
        );

        let response = if let Some(json) = json_body {
            request_builder.json(&json).send().await?
        } else {
            request_builder.send().await?
        };

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let message = response.text().await.unwrap_or_default();
            return Err(DelphiniumError::Http { status, message });
        }

        let parsed = response.json::<T>().await?;
        Ok(parsed)
    }

    pub async fn get_galleryinfo(&self, id: i32) -> Result<GalleryInfo, DelphiniumError> {
        self.request(reqwest::Method::GET, &format!("/galleryinfo/{}", id), None)
            .await
    }

    pub async fn get_image(&self, id: i32) -> Result<Vec<ResolvedImage>, DelphiniumError> {
        self.request(reqwest::Method::GET, &format!("/image/{}", id), None)
            .await
    }

    pub async fn get_info(&self, id: i32) -> Result<Info, DelphiniumError> {
        self.request(reqwest::Method::GET, &format!("/info/{}", id), None)
            .await
    }

    pub async fn get_list(&self, index: i32) -> Result<ListResultDTO, DelphiniumError> {
        self.request(reqwest::Method::GET, &format!("/list/{}", index), None)
            .await
    }

    pub async fn get_tags(&self) -> Result<Tags, DelphiniumError> {
        self.request(reqwest::Method::GET, "/tags", None).await
    }

    pub async fn get_thumbnail(
        &self,
        id: i32,
        size: Size,
        single: Option<bool>,
    ) -> Result<Vec<ResolvedImage>, DelphiniumError> {
        let single_str = if single.unwrap_or(true) {
            "true"
        } else {
            "false"
        };
        let endpoint = format!(
            "/thumbnail/{}?size={}&single={}",
            id,
            size.as_str(),
            single_str
        );
        self.request(reqwest::Method::GET, &endpoint, None).await
    }

    pub async fn post_random(&self, query: Vec<String>) -> Result<Info, DelphiniumError> {
        self.request(
            reqwest::Method::POST,
            &format!("/random"),
            Some(serde_json::json!({"query": query})),
        )
        .await
    }

    pub async fn post_search(
        &self,
        query: Vec<String>,
        offset: i32,
    ) -> Result<SearchResultDTO, DelphiniumError> {
        self.request(
            reqwest::Method::POST,
            &format!("/search?offset={}", offset),
            Some(serde_json::json!({"query": query})),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_galleryinfo() {
        let client =
            Delphinium::with_default_reqwest_client("https://heliotrope.saebasol.org".to_string());
        let result = client.get_galleryinfo(1).await;
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_image() {
        let client =
            Delphinium::with_default_reqwest_client("https://heliotrope.saebasol.org".to_string());

        let result = client.get_image(1).await;
        assert!(result.is_ok());
        assert!(result.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn test_get_info() {
        let client =
            Delphinium::with_default_reqwest_client("https://heliotrope.saebasol.org".to_string());

        let result = client.get_info(1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_list() {
        let client =
            Delphinium::with_default_reqwest_client("https://heliotrope.saebasol.org".to_string());

        let result = client.get_list(1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_tags() {
        let client =
            Delphinium::with_default_reqwest_client("https://heliotrope.saebasol.org".to_string());

        let result = client.get_tags().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_thumbnail() {
        let client =
            Delphinium::with_default_reqwest_client("https://heliotrope.saebasol.org".to_string());

        let result = client.get_thumbnail(1, Size::SmallSmall, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_post_random() {
        let client =
            Delphinium::with_default_reqwest_client("https://heliotrope.saebasol.org".to_string());

        let result = client.post_random(vec!["test".to_string()]).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_post_search() {
        let client =
            Delphinium::with_default_reqwest_client("https://heliotrope.saebasol.org".to_string());

        let result = client
            .post_search(vec!["artist:tamano_kedama".to_string()], 0)
            .await;
        assert!(result.is_ok());
        assert!(
            result.unwrap().results[0]
                .artists
                .contains(&"tamano_kedama".to_string())
        );
    }
}
