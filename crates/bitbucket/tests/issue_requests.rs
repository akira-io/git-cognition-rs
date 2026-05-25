use vcs_provider_bitbucket::bitbucket;

#[test]
fn bitbucket_issue_urls_target_repository_endpoints() {
    let repo = bitbucket()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .build();
    let issue = bitbucket().issue();
    let collection = issue.collection();
    let page = bitbucket()
        .pagination()
        .request()
        .limit(50)
        .cursor("2")
        .build();
    let query = issue.query().list(repo.clone(), Some(page));
    let issue_resource = issue.repo(repo).id("42").build();

    assert_eq!(
        issue_resource.url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/issues/42"
    );
    assert_eq!(
        collection.list(&query).value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/issues?pagelen=50&page=2"
    );
}
