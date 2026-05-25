use vcs_provider_core::{
    CodeReview, CodeReviewListQuery, PageRequest, RequestUrl, RequestUrlBuilder, url,
};

use crate::DEFAULT_BASE_URL;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitHubCodeReview {
    base_url: String,
    code_review: CodeReview,
}

impl GitHubCodeReview {
    pub fn make(base_url: impl Into<String>, code_review: CodeReview) -> Self {
        Self {
            base_url: base_url.into(),
            code_review,
        }
    }

    pub fn url(&self) -> RequestUrl {
        url(&self.base_url)
            .path_segments([
                "repos",
                self.code_review.repo().owner().as_str(),
                self.code_review.repo().name().as_str(),
                "pulls",
                self.code_review.id().as_str(),
            ])
            .build()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitHubCodeReviewCollection {
    base_url: String,
}

impl GitHubCodeReviewCollection {
    pub fn make(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    pub fn list(&self, query: &CodeReviewListQuery) -> RequestUrl {
        apply_page(
            url(&self.base_url).path_segments([
                "repos",
                query.repo().owner().as_str(),
                query.repo().name().as_str(),
                "pulls",
            ]),
            query.page(),
        )
        .build()
    }
}

impl Default for GitHubCodeReviewCollection {
    fn default() -> Self {
        Self::make(DEFAULT_BASE_URL)
    }
}

fn apply_page(request_url: RequestUrlBuilder, page: Option<&PageRequest>) -> RequestUrlBuilder {
    match page {
        Some(page) => request_url
            .optional_query_param(
                "per_page",
                page.limit().map(|limit| limit.as_u16().to_string()),
            )
            .optional_query_param(
                "page",
                page.cursor().map(|cursor| cursor.as_str().to_owned()),
            ),
        None => request_url,
    }
}
