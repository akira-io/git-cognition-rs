use vcs_provider_bitbucket::bitbucket;
use vcs_provider_core::{
    RepositoryDraftBuilder, RepositoryPatchBuilder, RequestMethod, Visibility,
};

#[test]
fn bitbucket_repo_get_targets_repository_endpoint() {
    let repo = bitbucket()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();

    assert_eq!(
        repo.url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs"
    );
}

#[test]
fn bitbucket_repo_branch_list_targets_repository_endpoint() {
    let repo = bitbucket()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let page = bitbucket()
        .pagination()
        .request()
        .limit(50)
        .cursor("2")
        .build();

    assert_eq!(
        repo.branches(Some(&page)).value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/refs/branches?pagelen=50&page=2"
    );
}

#[test]
fn bitbucket_repo_commit_list_targets_repository_endpoint() {
    let repo = bitbucket()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();

    assert_eq!(
        repo.commits(None).value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/commits"
    );
}

#[test]
fn bitbucket_repo_list_targets_collection_endpoint() {
    let page = bitbucket().pagination().request().limit(25).build();
    let repo = bitbucket().repo();
    let collection = repo.collection();
    let list_query = repo.query().list(Some(page.clone()));

    assert_eq!(
        collection.list(&list_query).value(),
        "https://api.bitbucket.org/2.0/repositories?pagelen=25"
    );
}

#[test]
fn bitbucket_repo_search_targets_collection_endpoint() {
    let page = bitbucket().pagination().request().limit(25).build();
    let repo = bitbucket().repo();
    let collection = repo.collection();
    let search_query = repo.query().search("vcs provider", Some(page));

    assert_eq!(
        collection.search(&search_query).value(),
        "https://api.bitbucket.org/2.0/repositories?q=name~%22vcs%20provider%22&pagelen=25"
    );
}

#[test]
fn bitbucket_repo_create_builds_put_request() {
    let repo = bitbucket()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let draft = RepositoryDraftBuilder::make(repo.clone().into())
        .visibility(Visibility::Private)
        .get();
    let create_request = repo.create(&draft);

    assert_eq!(create_request.method(), &RequestMethod::Put);
    assert!(create_request.body().is_some());
}

#[test]
fn bitbucket_repo_put_builds_put_request() {
    let repo = bitbucket()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let patch = RepositoryPatchBuilder::make(repo.clone().into())
        .visibility(Visibility::Public)
        .get();
    let put_request = repo.put(&patch);

    assert_eq!(put_request.method(), &RequestMethod::Put);
    assert!(put_request.body().is_some());
}

#[test]
fn bitbucket_repo_delete_builds_delete_request() {
    let repo = bitbucket()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();

    assert_eq!(repo.delete().method(), &RequestMethod::Delete);
}
