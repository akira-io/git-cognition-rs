use vcs_provider_bitbucket::bitbucket;
use vcs_provider_core::{
    RepositoryDraftBuilder, RepositoryPatchBuilder, RequestMethod, Visibility,
};

#[test]
fn bitbucket_repo_urls_target_repository_endpoints() {
    let repo = bitbucket()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .build();
    let page = bitbucket()
        .pagination()
        .request()
        .limit(50)
        .cursor("2")
        .build();

    assert_eq!(
        repo.url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs"
    );
    assert_eq!(
        repo.branches(Some(&page)).value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/refs/branches?pagelen=50&page=2"
    );
    assert_eq!(
        repo.commits(None).value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/commits"
    );
}

#[test]
fn bitbucket_repo_urls_target_collection_endpoints() {
    let page = bitbucket().pagination().request().limit(25).build();
    let repo = bitbucket().repo();
    let collection = repo.collection();
    let list_query = repo.query().list(Some(page.clone()));
    let search_query = repo.query().search("vcs provider", Some(page));

    assert_eq!(
        collection.list(&list_query).value(),
        "https://api.bitbucket.org/2.0/repositories?pagelen=25"
    );
    assert_eq!(
        collection.search(&search_query).value(),
        "https://api.bitbucket.org/2.0/repositories?q=name~%22vcs%20provider%22&pagelen=25"
    );
}

#[test]
fn bitbucket_repo_requests_build_mutation_requests() {
    let repo = bitbucket()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .build();
    let draft = RepositoryDraftBuilder::make(repo.clone().into())
        .visibility(Visibility::Private)
        .build();
    let patch = RepositoryPatchBuilder::make(repo.clone().into())
        .visibility(Visibility::Public)
        .build();
    let create_request = repo.create(&draft);
    let update_request = repo.update(&patch);
    let delete_request = repo.delete();

    assert_eq!(create_request.method(), &RequestMethod::Put);
    assert!(create_request.body().is_some());
    assert_eq!(update_request.method(), &RequestMethod::Put);
    assert!(update_request.body().is_some());
    assert_eq!(delete_request.method(), &RequestMethod::Delete);
}
