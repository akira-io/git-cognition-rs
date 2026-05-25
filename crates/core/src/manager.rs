use crate::Provider;
use crate::{
    CodeReview, CodeReviewListQuery, Issue, IssueListQuery, MissingCodeReviewId,
    MissingCodeReviewRepo, MissingIssueId, MissingIssueRepo, MissingOwnerName,
    MissingRepositoryName, PageRequest, ProvidedCodeReviewId, ProvidedCodeReviewRepo,
    ProvidedIssueId, ProvidedIssueRepo, ProvidedOwnerName, ProvidedRepositoryName, Repo,
    RepoBuilder, RepoQueryBuilder, RepositoryListQuery, RepositorySearchQuery, RequestUrl,
    code_review, issue, repo,
};

mod code_reviews;
mod issues;

pub use code_reviews::{
    ManagedCodeReview, ManagedCodeReviewBuilder, ManagedCodeReviewCollection,
    ManagedRepoCodeReviews, ManagedRepoCodeReviewsPagination,
};
pub use issues::{
    ManagedIssue, ManagedIssueBuilder, ManagedIssueCollection, ManagedRepoIssues,
    ManagedRepoIssuesPagination,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VcsManager<Driver> {
    driver: Driver,
}

impl<Driver> VcsManager<Driver>
where
    Driver: ManagedProvider,
{
    pub fn repo(&self) -> ManagedRepoBuilder<Driver, MissingOwnerName, MissingRepositoryName> {
        ManagedRepoBuilder {
            manager: self.clone(),
            repo: repo(),
        }
    }

    pub fn issue(&self) -> ManagedIssueBuilder<Driver, MissingIssueRepo, MissingIssueId>
    where
        Driver: ManagedIssueProvider,
    {
        ManagedIssueBuilder {
            manager: self.clone(),
            issue: issue(),
        }
    }

    pub fn code_review(
        &self,
    ) -> ManagedCodeReviewBuilder<Driver, MissingCodeReviewRepo, MissingCodeReviewId>
    where
        Driver: ManagedCodeReviewProvider,
    {
        ManagedCodeReviewBuilder {
            manager: self.clone(),
            code_review: code_review(),
        }
    }

    pub fn driver(&self) -> &Driver {
        &self.driver
    }

    pub fn pagination(&self) -> crate::PaginationBuilder {
        crate::pagination()
    }
}

pub trait ManagedProvider: Clone + Provider {
    fn repo_url(&self, repo: &Repo) -> RequestUrl;

    fn repo_branches_url(&self, repo: &Repo, page: Option<&PageRequest>) -> RequestUrl;

    fn repo_commits_url(&self, repo: &Repo, page: Option<&PageRequest>) -> RequestUrl;

    fn repo_list_url(&self, query: &RepositoryListQuery) -> RequestUrl;

    fn repo_search_url(&self, query: &RepositorySearchQuery) -> RequestUrl;
}

pub trait ManagedIssueProvider: ManagedProvider {
    fn issue_url(&self, issue: &Issue) -> RequestUrl;

    fn issue_list_url(&self, query: &IssueListQuery) -> RequestUrl;
}

pub trait ManagedCodeReviewProvider: ManagedProvider {
    fn code_review_url(&self, code_review: &CodeReview) -> RequestUrl;

    fn code_review_list_url(&self, query: &CodeReviewListQuery) -> RequestUrl;
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VcsManagerBuilder;

impl VcsManagerBuilder {
    pub fn driver<Driver>(self, driver: Driver) -> VcsManagerWithDriverBuilder<Driver>
    where
        Driver: ManagedProvider,
    {
        VcsManagerWithDriverBuilder { driver }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VcsManagerWithDriverBuilder<Driver> {
    driver: Driver,
}

impl<Driver> VcsManagerWithDriverBuilder<Driver>
where
    Driver: ManagedProvider,
{
    pub fn build(self) -> VcsManager<Driver> {
        VcsManager {
            driver: self.driver,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedRepoBuilder<Driver, OwnerNameState, RepositoryNameState> {
    manager: VcsManager<Driver>,
    repo: RepoBuilder<OwnerNameState, RepositoryNameState>,
}

impl<Driver> ManagedRepoBuilder<Driver, MissingOwnerName, MissingRepositoryName>
where
    Driver: ManagedProvider,
{
    pub fn collection(&self) -> ManagedRepoCollection<Driver> {
        ManagedRepoCollection {
            manager: self.manager.clone(),
        }
    }

    pub fn query(&self) -> crate::RepoQueryBuilder {
        RepoQueryBuilder
    }
}

impl<Driver, RepositoryNameState> ManagedRepoBuilder<Driver, MissingOwnerName, RepositoryNameState>
where
    Driver: ManagedProvider,
{
    pub fn owner(
        self,
        owner_name: impl Into<String>,
    ) -> ManagedRepoBuilder<Driver, ProvidedOwnerName, RepositoryNameState> {
        ManagedRepoBuilder {
            manager: self.manager,
            repo: self.repo.owner(owner_name),
        }
    }
}

impl<Driver, OwnerNameState> ManagedRepoBuilder<Driver, OwnerNameState, MissingRepositoryName>
where
    Driver: ManagedProvider,
{
    pub fn name(
        self,
        repository_name: impl Into<String>,
    ) -> ManagedRepoBuilder<Driver, OwnerNameState, ProvidedRepositoryName> {
        ManagedRepoBuilder {
            manager: self.manager,
            repo: self.repo.name(repository_name),
        }
    }
}

impl<Driver> ManagedRepoBuilder<Driver, ProvidedOwnerName, ProvidedRepositoryName>
where
    Driver: ManagedProvider,
{
    pub fn build(self) -> ManagedRepo<Driver> {
        ManagedRepo {
            manager: self.manager,
            repo: self.repo.build(),
        }
    }
}

impl<Driver> ManagedRepoBuilder<Driver, ProvidedOwnerName, ProvidedRepositoryName>
where
    Driver: ManagedIssueProvider,
{
    pub fn issue(
        self,
        id: impl Into<String>,
    ) -> ManagedIssueBuilder<Driver, ProvidedIssueRepo, ProvidedIssueId> {
        ManagedIssueBuilder {
            manager: self.manager,
            issue: issue().repo(self.repo.build()).id(id),
        }
    }

    pub fn issues(self) -> ManagedRepoIssues<Driver> {
        ManagedRepoIssues {
            manager: self.manager,
            repo: self.repo.build(),
            page: None,
        }
    }
}

impl<Driver> ManagedRepoBuilder<Driver, ProvidedOwnerName, ProvidedRepositoryName>
where
    Driver: ManagedCodeReviewProvider,
{
    pub fn code_review(
        self,
        id: impl Into<String>,
    ) -> ManagedCodeReviewBuilder<Driver, ProvidedCodeReviewRepo, ProvidedCodeReviewId> {
        ManagedCodeReviewBuilder {
            manager: self.manager,
            code_review: code_review().repo(self.repo.build()).id(id),
        }
    }

    pub fn code_reviews(self) -> ManagedRepoCodeReviews<Driver> {
        ManagedRepoCodeReviews {
            manager: self.manager,
            repo: self.repo.build(),
            page: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedRepo<Driver> {
    manager: VcsManager<Driver>,
    repo: Repo,
}

impl<Driver> ManagedRepo<Driver>
where
    Driver: ManagedProvider,
{
    pub fn url(&self) -> RequestUrl {
        self.manager.driver.repo_url(&self.repo)
    }

    pub fn branches(&self, page: Option<&PageRequest>) -> RequestUrl {
        self.manager.driver.repo_branches_url(&self.repo, page)
    }

    pub fn commits(&self, page: Option<&PageRequest>) -> RequestUrl {
        self.manager.driver.repo_commits_url(&self.repo, page)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedRepoCollection<Driver> {
    manager: VcsManager<Driver>,
}

impl<Driver> ManagedRepoCollection<Driver>
where
    Driver: ManagedProvider,
{
    pub fn list(&self, query: &RepositoryListQuery) -> RequestUrl {
        self.manager.driver.repo_list_url(query)
    }

    pub fn search(&self, query: &RepositorySearchQuery) -> RequestUrl {
        self.manager.driver.repo_search_url(query)
    }
}

impl<Driver> From<ManagedRepo<Driver>> for Repo {
    fn from(managed_repo: ManagedRepo<Driver>) -> Self {
        managed_repo.repo
    }
}
