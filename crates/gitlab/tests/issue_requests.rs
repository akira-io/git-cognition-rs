use vcs_provider_gitlab::gitlab;

#[test]
fn gitlab_issue_urls_target_repository_endpoints() {
    let repo = gitlab()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .build();
    let issue = gitlab().issue();
    let collection = issue.collection();
    let page = gitlab()
        .pagination()
        .request()
        .limit(50)
        .cursor("2")
        .build();
    let query = issue.query().list(repo.clone(), Some(page));
    let issue_resource = issue.repo(repo).id("42").build();

    assert_eq!(
        issue_resource.url().value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fvcs-providers-rs/issues/42"
    );
    assert_eq!(
        collection.list(&query).value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fvcs-providers-rs/issues?per_page=50&page=2"
    );
}
