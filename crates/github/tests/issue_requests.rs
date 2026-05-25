use vcs_provider_core::{IssuePatchBuilder, RequestMethod, issue};
use vcs_provider_github::github;

#[test]
fn github_issue_get_targets_repository_endpoint() {
    let issue_resource = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .issue("42")
        .get();

    assert_eq!(
        issue_resource.url().value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/issues/42"
    );
}

#[test]
fn github_issue_list_targets_repository_endpoint() {
    let issues = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .issues()
        .pagination()
        .limit(50)
        .cursor("2")
        .list();

    assert_eq!(
        issues.url().value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/issues?per_page=50&page=2"
    );
}

#[test]
fn github_issue_builder_accepts_existing_repo() {
    let repo = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let issue_resource = github().issue().repo(repo).id("42").get();

    assert_eq!(
        issue_resource.url().value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/issues/42"
    );
}

#[test]
fn github_issue_create_builds_post_request() {
    let repo = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let draft = issue()
        .draft()
        .repo(repo.clone())
        .title("Track mutable issue requests")
        .body("Details")
        .get();
    let collection = github().issue().collection();

    assert_eq!(collection.create(&draft).method(), &RequestMethod::Post);
    assert!(collection.create(&draft).body().is_some());
}

#[test]
fn github_issue_patch_builds_patch_request() {
    let repo = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let issue_resource = github().issue().repo(repo).id("42").get();
    let patch = IssuePatchBuilder::make(issue_resource.issue().clone())
        .closed()
        .get();

    assert_eq!(issue_resource.patch(&patch).method(), &RequestMethod::Patch);
}

#[test]
fn github_issue_delete_builds_close_request() {
    let repo = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let issue_resource = github().issue().repo(repo).id("42").get();

    assert_eq!(issue_resource.delete().method(), &RequestMethod::Patch);
}
