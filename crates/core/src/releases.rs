use serde::{Deserialize, Serialize};

use crate::{BoxFuture, Page, PageRequest, Repo, VcsResult, transport_not_configured};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReleaseId(String);

impl ReleaseId {
    pub fn make(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Release {
    repo: Repo,
    id: ReleaseId,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReleaseDraft {
    repo: Repo,
    tag: String,
    name: Option<String>,
    body: Option<String>,
}

impl ReleaseDraft {
    pub fn repo(&self) -> &Repo {
        &self.repo
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseDraftBuilder<RepoState, TagState> {
    repo: RepoState,
    tag: TagState,
    name: Option<String>,
    body: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingReleaseTag;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedReleaseTag {
    tag: String,
}

impl Release {
    pub fn builder() -> ReleaseBuilder<MissingReleaseRepo, MissingReleaseId> {
        ReleaseBuilder {
            repo: MissingReleaseRepo,
            id: MissingReleaseId,
        }
    }

    pub fn make(repo: Repo, id: ReleaseId) -> Self {
        Self { repo, id }
    }

    pub fn repo(&self) -> &Repo {
        &self.repo
    }

    pub fn id(&self) -> &ReleaseId {
        &self.id
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReleasePatch {
    release: Release,
    name: Option<String>,
    body: Option<String>,
}

impl ReleasePatch {
    pub fn release(&self) -> &Release {
        &self.release
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleasePatchBuilder {
    release: Release,
    name: Option<String>,
    body: Option<String>,
}

impl ReleasePatchBuilder {
    pub fn make(release: Release) -> Self {
        Self {
            release,
            name: None,
            body: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn build(self) -> ReleasePatch {
        ReleasePatch {
            release: self.release,
            name: self.name,
            body: self.body,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingReleaseRepo;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedReleaseRepo {
    repo: Repo,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingReleaseId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedReleaseId {
    id: ReleaseId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseBuilder<RepoState, ReleaseIdState> {
    repo: RepoState,
    id: ReleaseIdState,
}

impl<ReleaseIdState> ReleaseBuilder<MissingReleaseRepo, ReleaseIdState> {
    pub fn repo(
        self,
        repo: impl Into<Repo>,
    ) -> ReleaseBuilder<ProvidedReleaseRepo, ReleaseIdState> {
        ReleaseBuilder {
            repo: ProvidedReleaseRepo { repo: repo.into() },
            id: self.id,
        }
    }
}

impl<RepoState> ReleaseBuilder<RepoState, MissingReleaseId> {
    pub fn id(self, id: impl Into<String>) -> ReleaseBuilder<RepoState, ProvidedReleaseId> {
        ReleaseBuilder {
            repo: self.repo,
            id: ProvidedReleaseId {
                id: ReleaseId::make(id),
            },
        }
    }
}

impl ReleaseBuilder<ProvidedReleaseRepo, ProvidedReleaseId> {
    pub fn build(self) -> Release {
        Release {
            repo: self.repo.repo,
            id: self.id.id,
        }
    }
}

impl ReleaseBuilder<MissingReleaseRepo, MissingReleaseId> {
    pub fn query(self) -> ReleaseQueryBuilder {
        ReleaseQueryBuilder
    }

    pub fn draft(self) -> ReleaseDraftBuilder<MissingReleaseRepo, MissingReleaseTag> {
        ReleaseDraftBuilder {
            repo: MissingReleaseRepo,
            tag: MissingReleaseTag,
            name: None,
            body: None,
        }
    }
}

impl<TagState> ReleaseDraftBuilder<MissingReleaseRepo, TagState> {
    pub fn repo(self, repo: impl Into<Repo>) -> ReleaseDraftBuilder<ProvidedReleaseRepo, TagState> {
        ReleaseDraftBuilder {
            repo: ProvidedReleaseRepo { repo: repo.into() },
            tag: self.tag,
            name: self.name,
            body: self.body,
        }
    }
}

impl<RepoState> ReleaseDraftBuilder<RepoState, MissingReleaseTag> {
    pub fn tag(self, tag: impl Into<String>) -> ReleaseDraftBuilder<RepoState, ProvidedReleaseTag> {
        ReleaseDraftBuilder {
            repo: self.repo,
            tag: ProvidedReleaseTag { tag: tag.into() },
            name: self.name,
            body: self.body,
        }
    }
}

impl<RepoState, TagState> ReleaseDraftBuilder<RepoState, TagState> {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }
}

impl ReleaseDraftBuilder<ProvidedReleaseRepo, ProvidedReleaseTag> {
    pub fn build(self) -> ReleaseDraft {
        ReleaseDraft {
            repo: self.repo.repo,
            tag: self.tag.tag,
            name: self.name,
            body: self.body,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ReleaseQueryBuilder;

impl ReleaseQueryBuilder {
    pub fn list(self, repo: Repo, page: Option<PageRequest>) -> ReleaseListQuery {
        ReleaseListQuery { repo, page }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReleaseListQuery {
    repo: Repo,
    page: Option<PageRequest>,
}

impl ReleaseListQuery {
    pub fn repo(&self) -> &Repo {
        &self.repo
    }

    pub fn page(&self) -> Option<&PageRequest> {
        self.page.as_ref()
    }
}

pub trait Releases: Send + Sync {
    fn get(&self, repo: Repo, id: ReleaseId) -> BoxFuture<'_, VcsResult<Release>>;

    fn list(&self, query: ReleaseListQuery) -> BoxFuture<'_, VcsResult<Page<Release>>>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TransportNotConfiguredReleases;

impl Releases for TransportNotConfiguredReleases {
    fn get(&self, _repo: Repo, _id: ReleaseId) -> BoxFuture<'_, VcsResult<Release>> {
        transport_not_configured()
    }

    fn list(&self, _query: ReleaseListQuery) -> BoxFuture<'_, VcsResult<Page<Release>>> {
        transport_not_configured()
    }
}
