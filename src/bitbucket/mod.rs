use crate::{
    AuthHeaderStyle, AuthKind, CodeReviews, Issues, ManagedAuthProvider, ManagedCodeReviewProvider,
    ManagedIssueProvider, ManagedOrganizationProvider, ManagedProvider, MissingCodeReviewId,
    MissingCodeReviewRepo, MissingIssueId, MissingIssueRepo, MissingOwnerName,
    MissingRepositoryName, Organizations, Pipelines, Provider, ProviderDescriptor, ProviderId,
    Releases, Repos, TransportNotConfiguredAuthentication, TransportNotConfiguredCodeReviews,
    TransportNotConfiguredIssues, TransportNotConfiguredOrganizations,
    TransportNotConfiguredPipelines, TransportNotConfiguredRepos, UnsupportedReleases,
};

mod capabilities;
mod client;
mod code_reviews;
mod issues;
mod mappers;
mod pagination;
mod pipelines;
mod provider_pipelines;
mod repos;
mod request_pagination;
mod response_fixture;

pub use client::BitbucketClient;
pub use code_reviews::{BitbucketCodeReview, BitbucketCodeReviewCollection};
pub use issues::{BitbucketIssue, BitbucketIssueCollection};
pub use pipelines::{BitbucketPipeline, BitbucketPipelineCollection};
pub use repos::{BitbucketRepo, BitbucketRepoCollection};
pub use response_fixture::BitbucketResponseBuilder;

use capabilities::bitbucket_capabilities;

pub const PROVIDER_ID: &str = "bitbucket";
pub const DISPLAY_NAME: &str = "Bitbucket";
pub const DEFAULT_BASE_URL: &str = "https://api.bitbucket.org/2.0";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitbucketProvider {
    base_url: String,
}

impl BitbucketProvider {
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn api_base_url(&self) -> &str {
        &self.base_url
    }

    pub fn repo(&self) -> crate::ManagedRepoBuilder<Self, MissingOwnerName, MissingRepositoryName> {
        crate::cognition().provider(self.clone()).repo()
    }

    pub fn code_review(
        &self,
    ) -> crate::ManagedCodeReviewBuilder<Self, MissingCodeReviewRepo, MissingCodeReviewId> {
        crate::cognition().provider(self.clone()).code_review()
    }

    pub fn issue(&self) -> crate::ManagedIssueBuilder<Self, MissingIssueRepo, MissingIssueId> {
        crate::cognition().provider(self.clone()).issue()
    }

    pub fn pagination(&self) -> crate::PaginationBuilder {
        crate::pagination()
    }
}

impl ManagedProvider for BitbucketProvider {
    fn repo_url(&self, repo: &crate::Repo) -> crate::RequestUrl {
        BitbucketRepo::make(self.api_base_url(), repo.clone()).url()
    }

    fn repo_branches_url(
        &self,
        repo: &crate::Repo,
        page: Option<&crate::PageRequest>,
    ) -> crate::RequestUrl {
        BitbucketRepo::make(self.api_base_url(), repo.clone()).branches(page)
    }

    fn repo_commits_url(
        &self,
        repo: &crate::Repo,
        page: Option<&crate::PageRequest>,
    ) -> crate::RequestUrl {
        BitbucketRepo::make(self.api_base_url(), repo.clone()).commits(page)
    }

    fn repo_list_url(&self, query: &crate::RepositoryListQuery) -> crate::RequestUrl {
        BitbucketRepoCollection::make(self.api_base_url()).list(query)
    }

    fn repo_search_url(&self, query: &crate::RepositorySearchQuery) -> crate::RequestUrl {
        BitbucketRepoCollection::make(self.api_base_url()).search(query)
    }

    fn repo_create_request(&self, draft: &crate::RepositoryDraft) -> crate::Request {
        BitbucketRepo::make(self.api_base_url(), draft.repo().clone()).create(draft)
    }

    fn repo_update_request(&self, patch: &crate::RepositoryPatch) -> crate::Request {
        BitbucketRepo::make(self.api_base_url(), patch.repo().clone()).update(patch)
    }

    fn repo_delete_request(&self, repo: &crate::Repo) -> crate::Request {
        BitbucketRepo::make(self.api_base_url(), repo.clone()).delete()
    }

    fn repo_branch_create_request(
        &self,
        draft: &crate::BranchDraft,
    ) -> crate::CognitionResult<crate::Request> {
        Ok(BitbucketRepo::make(self.api_base_url(), draft.repo().clone()).create_branch(draft))
    }

    fn repo_branch_delete_request(
        &self,
        repo: &crate::Repo,
        branch_name: &str,
    ) -> crate::CognitionResult<crate::Request> {
        Ok(BitbucketRepo::make(self.api_base_url(), repo.clone()).delete_branch(branch_name))
    }
}

impl ManagedAuthProvider for BitbucketProvider {
    fn auth_validate_url(&self) -> crate::RequestUrl {
        crate::url(self.api_base_url())
            .path_segments(["user"])
            .build()
    }
}

impl ManagedOrganizationProvider for BitbucketProvider {
    fn organization_list_url(
        &self,
        query: Option<&crate::OrganizationListQuery>,
    ) -> crate::RequestUrl {
        let url = crate::url(self.api_base_url()).path_segments(["user", "workspaces"]);

        match query.and_then(crate::OrganizationListQuery::page) {
            Some(page) => self::request_pagination::apply_page(url, Some(page)).build(),
            None => url.build(),
        }
    }
}

impl ManagedIssueProvider for BitbucketProvider {
    fn issue_url(&self, issue: &crate::Issue) -> crate::RequestUrl {
        BitbucketIssue::make(self.api_base_url(), issue.clone()).url()
    }

    fn issue_list_url(&self, query: &crate::IssueListQuery) -> crate::RequestUrl {
        BitbucketIssueCollection::make(self.api_base_url()).list(query)
    }

    fn issue_create_request(&self, draft: &crate::IssueDraft) -> crate::Request {
        BitbucketIssueCollection::make(self.api_base_url()).create(draft)
    }

    fn issue_update_request(&self, patch: &crate::IssuePatch) -> crate::Request {
        BitbucketIssue::make(self.api_base_url(), patch.issue().clone()).update(patch)
    }

    fn issue_delete_request(&self, issue: &crate::Issue) -> crate::CognitionResult<crate::Request> {
        Ok(BitbucketIssue::make(self.api_base_url(), issue.clone()).delete())
    }
}

impl ManagedCodeReviewProvider for BitbucketProvider {
    fn code_review_url(&self, code_review: &crate::CodeReview) -> crate::RequestUrl {
        BitbucketCodeReview::make(self.api_base_url(), code_review.clone()).url()
    }

    fn code_review_list_url(&self, query: &crate::CodeReviewListQuery) -> crate::RequestUrl {
        BitbucketCodeReviewCollection::make(self.api_base_url()).list(query)
    }

    fn code_review_create_request(&self, draft: &crate::CodeReviewDraft) -> crate::Request {
        BitbucketCodeReviewCollection::make(self.api_base_url()).create(draft)
    }

    fn code_review_update_request(&self, patch: &crate::CodeReviewPatch) -> crate::Request {
        BitbucketCodeReview::make(self.api_base_url(), patch.code_review().clone()).update(patch)
    }

    fn code_review_merge_request(&self, code_review: &crate::CodeReview) -> crate::Request {
        BitbucketCodeReview::make(self.api_base_url(), code_review.clone()).merge()
    }

    fn code_review_close_request(&self, code_review: &crate::CodeReview) -> crate::Request {
        BitbucketCodeReview::make(self.api_base_url(), code_review.clone()).close()
    }
}

impl Provider for BitbucketProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptor::make(
            ProviderId::make(PROVIDER_ID),
            DISPLAY_NAME,
            bitbucket_capabilities(),
        )
    }

    fn repos(&self) -> Box<dyn Repos> {
        Box::<TransportNotConfiguredRepos>::default()
    }

    fn authentication(&self) -> Box<dyn crate::Authentication> {
        Box::<TransportNotConfiguredAuthentication>::default()
    }

    fn organizations(&self) -> Box<dyn Organizations> {
        Box::<TransportNotConfiguredOrganizations>::default()
    }

    fn issues(&self) -> Box<dyn Issues> {
        Box::<TransportNotConfiguredIssues>::default()
    }

    fn code_reviews(&self) -> Box<dyn CodeReviews> {
        Box::<TransportNotConfiguredCodeReviews>::default()
    }

    fn pipelines(&self) -> Box<dyn Pipelines> {
        Box::<TransportNotConfiguredPipelines>::default()
    }

    fn releases(&self) -> Box<dyn Releases> {
        Box::<UnsupportedReleases>::default()
    }

    fn default_base_url(&self) -> &str {
        self.api_base_url()
    }

    fn auth_header_style(&self, auth_kind: AuthKind) -> AuthHeaderStyle {
        match auth_kind {
            AuthKind::Anonymous => AuthHeaderStyle::None,
            AuthKind::PersonalAccessToken => AuthHeaderStyle::AuthorizationBearer,
            AuthKind::OAuth => AuthHeaderStyle::AuthorizationBearer,
            AuthKind::AppInstallation => AuthHeaderStyle::AuthorizationBearer,
            AuthKind::Jwt => AuthHeaderStyle::AuthorizationBearer,
        }
    }
}

pub fn bitbucket() -> BitbucketProvider {
    BitbucketProvider::default()
}

impl Default for BitbucketProvider {
    fn default() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.into(),
        }
    }
}
