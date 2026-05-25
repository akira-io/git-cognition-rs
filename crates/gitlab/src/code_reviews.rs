use vcs_provider_core::{
    CodeReview, CodeReviewListQuery, PageRequest, RequestUrl, RequestUrlBuilder, url,
};

use crate::DEFAULT_BASE_URL;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitLabCodeReview {
    base_url: String,
    code_review: CodeReview,
}

impl GitLabCodeReview {
    pub fn make(base_url: impl Into<String>, code_review: CodeReview) -> Self {
        Self {
            base_url: base_url.into(),
            code_review,
        }
    }

    pub fn url(&self) -> RequestUrl {
        let project_path = project_path(self.code_review.repo());

        url(&self.base_url)
            .path_segments([
                "api",
                "v4",
                "projects",
                project_path.as_str(),
                "merge_requests",
                self.code_review.id().as_str(),
            ])
            .build()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitLabCodeReviewCollection {
    base_url: String,
}

impl GitLabCodeReviewCollection {
    pub fn make(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    pub fn list(&self, query: &CodeReviewListQuery) -> RequestUrl {
        let project_path = project_path(query.repo());

        apply_page(
            url(&self.base_url).path_segments([
                "api",
                "v4",
                "projects",
                project_path.as_str(),
                "merge_requests",
            ]),
            query.page(),
        )
        .build()
    }
}

impl Default for GitLabCodeReviewCollection {
    fn default() -> Self {
        Self::make(DEFAULT_BASE_URL)
    }
}

fn project_path(repo: &vcs_provider_core::Repo) -> String {
    format!("{}/{}", repo.owner().as_str(), repo.name().as_str())
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
