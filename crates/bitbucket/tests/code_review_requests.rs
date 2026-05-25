use vcs_provider_bitbucket::bitbucket;

#[test]
fn bitbucket_code_review_urls_target_repository_endpoints() {
    let code_review = bitbucket()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .code_review("42")
        .build();
    let code_reviews = bitbucket()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .code_reviews()
        .pagination()
        .limit(50)
        .cursor("2")
        .build();

    assert_eq!(
        code_review.url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/pullrequests/42"
    );
    assert_eq!(
        code_reviews.url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/pullrequests?pagelen=50&page=2"
    );
}

#[test]
fn bitbucket_code_review_builder_accepts_existing_repo() {
    let repo = bitbucket()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .build();
    let code_review = bitbucket().code_review().repo(repo).id("42").build();

    assert_eq!(
        code_review.url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/pullrequests/42"
    );
}
