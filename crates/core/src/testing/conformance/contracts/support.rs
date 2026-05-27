use crate::{
    Capability, CodeReview, Issue, Pipeline, Provider, Release, Repo, VcsError, VcsResult,
    code_review, error, issue, pipeline, release, repo,
};

pub(super) fn sample_repo_location() -> Repo {
    repo().owner("akira-io").name("vcs-providers-rs").get()
}

pub(super) fn sample_issue(repo_location: Repo) -> Issue {
    issue().repo(repo_location).id("42").get()
}

pub(super) fn sample_code_review(repo_location: Repo) -> CodeReview {
    code_review().repo(repo_location).id("42").get()
}

pub(super) fn sample_pipeline(repo_location: Repo) -> Pipeline {
    pipeline().repo(repo_location).id("42").get()
}

pub(super) fn sample_release(repo_location: Repo) -> Release {
    release().repo(repo_location).id("v1.0.0").get()
}

pub(super) fn provider_supports(provider: &impl Provider, capability: Capability) -> bool {
    provider.descriptor().capabilities().supports(&capability)
}

pub(super) fn assert_transport_not_configured<T>(
    operation: &str,
    result: VcsResult<T>,
) -> VcsResult<()> {
    match result {
        Err(VcsError::TransportNotConfigured) => Ok(()),
        Err(_) => Err(error().invalid_input(format!("{operation} returned wrong error"))),
        Ok(_) => Err(error().invalid_input(format!(
            "{operation} succeeded without configured transport"
        ))),
    }
}

pub(super) fn assert_capability_contract_error<T>(
    operation: &str,
    result: VcsResult<T>,
    supported: bool,
) -> VcsResult<()> {
    if supported {
        return assert_transport_not_configured(operation, result);
    }

    match result {
        Err(VcsError::UnsupportedOperation(_)) => Ok(()),
        Err(_) => Err(error().invalid_input(format!("{operation} returned wrong error"))),
        Ok(_) => {
            Err(error().invalid_input(format!("{operation} succeeded without provider capability")))
        }
    }
}
