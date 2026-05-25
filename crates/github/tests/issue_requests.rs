use vcs_provider_github::github;

#[test]
fn github_issue_urls_target_repository_endpoints() {
    let repo = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .build();
    let issue = github().issue();
    let collection = issue.collection();
    let page = github()
        .pagination()
        .request()
        .limit(50)
        .cursor("2")
        .build();
    let query = issue.query().list(repo.clone(), Some(page));
    let issue_resource = issue.repo(repo).id("42").build();

    assert_eq!(
        issue_resource.url().value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/issues/42"
    );
    assert_eq!(
        collection.list(&query).value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/issues?per_page=50&page=2"
    );
}
