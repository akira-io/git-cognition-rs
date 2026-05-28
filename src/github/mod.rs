use crate::{
    AuthHeaderStyle, AuthKind, BranchDraft, CodeReviews, Issues, ManagedAuthProvider,
    ManagedCodeReviewProvider, ManagedIssueProvider, ManagedProvider, Pipelines, Provider,
    ProviderDescriptor, ProviderId, Releases, Repos, TransportNotConfiguredAuthentication,
    TransportNotConfiguredCodeReviews, TransportNotConfiguredIssues,
    TransportNotConfiguredOrganizations, TransportNotConfiguredPipelines,
    TransportNotConfiguredReleases, TransportNotConfiguredRepos,
};

mod capabilities;
mod client;
mod code_reviews;
mod issues;
mod mappers;
mod pagination;
mod pipelines;
mod provider_fluent;
mod provider_pipelines;
mod releases;
mod repos;
mod request_pagination;
mod response_fixture;

pub use client::GitHubClient;
pub use code_reviews::{GitHubCodeReview, GitHubCodeReviewCollection};
pub use issues::{GitHubIssue, GitHubIssueCollection};
pub use pipelines::{GitHubPipeline, GitHubPipelineCollection};
pub use releases::{GitHubRelease, GitHubReleaseCollection};
pub use repos::{GitHubRepo, GitHubRepoCollection};
pub use response_fixture::GitHubResponseBuilder;

use capabilities::github_capabilities;

pub const PROVIDER_ID: &str = "github";
pub const DISPLAY_NAME: &str = "GitHub";
pub const DEFAULT_BASE_URL: &str = "https://api.github.com";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitHubProvider {
    base_url: String,
}

impl ManagedProvider for GitHubProvider {
    fn repo_url(&self, repo: &crate::Repo) -> crate::RequestUrl {
        GitHubRepo::make(self.api_base_url(), repo.clone()).url()
    }

    fn repo_branches_url(
        &self,
        repo: &crate::Repo,
        page: Option<&crate::PageRequest>,
    ) -> crate::RequestUrl {
        GitHubRepo::make(self.api_base_url(), repo.clone()).branches(page)
    }

    fn repo_commits_url(
        &self,
        repo: &crate::Repo,
        page: Option<&crate::PageRequest>,
    ) -> crate::RequestUrl {
        GitHubRepo::make(self.api_base_url(), repo.clone()).commits(page)
    }

    fn repo_list_url(&self, query: &crate::RepositoryListQuery) -> crate::RequestUrl {
        GitHubRepoCollection::make(self.api_base_url()).list(query)
    }

    fn repo_search_url(&self, query: &crate::RepositorySearchQuery) -> crate::RequestUrl {
        GitHubRepoCollection::make(self.api_base_url()).search(query)
    }

    fn repo_create_request(&self, draft: &crate::RepositoryDraft) -> crate::Request {
        GitHubRepoCollection::make(self.api_base_url()).create(draft)
    }

    fn repo_update_request(&self, patch: &crate::RepositoryPatch) -> crate::Request {
        GitHubRepo::make(self.api_base_url(), patch.repo().clone()).update(patch)
    }

    fn repo_delete_request(&self, repo: &crate::Repo) -> crate::Request {
        GitHubRepo::make(self.api_base_url(), repo.clone()).delete()
    }

    fn repo_branch_create_request(
        &self,
        draft: &BranchDraft,
    ) -> crate::CognitionResult<crate::Request> {
        Ok(GitHubRepo::make(self.api_base_url(), draft.repo().clone()).create_branch(draft))
    }

    fn repo_branch_delete_request(
        &self,
        repo: &crate::Repo,
        branch_name: &str,
    ) -> crate::CognitionResult<crate::Request> {
        Ok(GitHubRepo::make(self.api_base_url(), repo.clone()).delete_branch(branch_name))
    }
}

impl ManagedAuthProvider for GitHubProvider {
    fn auth_validate_url(&self) -> crate::RequestUrl {
        crate::url(self.api_base_url())
            .path_segments(["user"])
            .build()
    }
}

impl crate::ManagedOrganizationProvider for GitHubProvider {
    fn organization_list_url(
        &self,
        query: Option<&crate::OrganizationListQuery>,
    ) -> crate::RequestUrl {
        let url = crate::url(self.api_base_url()).path_segments(["user", "orgs"]);

        match query.and_then(crate::OrganizationListQuery::page) {
            Some(page) => self::request_pagination::apply_page(url, Some(page)).build(),
            None => url.build(),
        }
    }
}

impl ManagedIssueProvider for GitHubProvider {
    fn issue_url(&self, issue: &crate::Issue) -> crate::RequestUrl {
        GitHubIssue::make(self.api_base_url(), issue.clone()).url()
    }

    fn issue_list_url(&self, query: &crate::IssueListQuery) -> crate::RequestUrl {
        GitHubIssueCollection::make(self.api_base_url()).list(query)
    }

    fn issue_create_request(&self, draft: &crate::IssueDraft) -> crate::Request {
        GitHubIssueCollection::make(self.api_base_url()).create(draft)
    }

    fn issue_update_request(&self, patch: &crate::IssuePatch) -> crate::Request {
        GitHubIssue::make(self.api_base_url(), patch.issue().clone()).update(patch)
    }
}

impl ManagedCodeReviewProvider for GitHubProvider {
    fn code_review_url(&self, code_review: &crate::CodeReview) -> crate::RequestUrl {
        GitHubCodeReview::make(self.api_base_url(), code_review.clone()).url()
    }

    fn code_review_list_url(&self, query: &crate::CodeReviewListQuery) -> crate::RequestUrl {
        GitHubCodeReviewCollection::make(self.api_base_url()).list(query)
    }

    fn code_review_create_request(&self, draft: &crate::CodeReviewDraft) -> crate::Request {
        GitHubCodeReviewCollection::make(self.api_base_url()).create(draft)
    }

    fn code_review_update_request(&self, patch: &crate::CodeReviewPatch) -> crate::Request {
        GitHubCodeReview::make(self.api_base_url(), patch.code_review().clone()).update(patch)
    }

    fn code_review_merge_request(&self, code_review: &crate::CodeReview) -> crate::Request {
        GitHubCodeReview::make(self.api_base_url(), code_review.clone()).merge()
    }

    fn code_review_close_request(&self, code_review: &crate::CodeReview) -> crate::Request {
        GitHubCodeReview::make(self.api_base_url(), code_review.clone()).close()
    }
}

impl crate::ManagedReleaseProvider for GitHubProvider {
    fn release_url(&self, release: &crate::Release) -> crate::RequestUrl {
        GitHubRelease::make(self.api_base_url(), release.clone()).url()
    }

    fn release_list_url(&self, query: &crate::ReleaseListQuery) -> crate::RequestUrl {
        GitHubReleaseCollection::make(self.api_base_url()).list(query)
    }

    fn release_create_request(&self, draft: &crate::ReleaseDraft) -> crate::Request {
        GitHubReleaseCollection::make(self.api_base_url()).create(draft)
    }

    fn release_update_request(&self, patch: &crate::ReleasePatch) -> crate::Request {
        GitHubRelease::make(self.api_base_url(), patch.release().clone()).update(patch)
    }

    fn release_delete_request(&self, release: &crate::Release) -> crate::Request {
        GitHubRelease::make(self.api_base_url(), release.clone()).delete()
    }
}

impl Provider for GitHubProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptor::make(
            ProviderId::make(PROVIDER_ID),
            DISPLAY_NAME,
            github_capabilities(),
        )
    }

    fn authentication(&self) -> Box<dyn crate::Authentication> {
        Box::<TransportNotConfiguredAuthentication>::default()
    }

    fn organizations(&self) -> Box<dyn crate::Organizations> {
        Box::<TransportNotConfiguredOrganizations>::default()
    }

    fn repos(&self) -> Box<dyn Repos> {
        Box::<TransportNotConfiguredRepos>::default()
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
        Box::<TransportNotConfiguredReleases>::default()
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

pub fn github() -> GitHubProvider {
    GitHubProvider::default()
}
