use vcs_provider_core::{
    RepositoryDraftBuilder, RepositoryPatchBuilder, RequestMethod, Visibility,
};
use vcs_provider_gitlab::gitlab;

#[test]
fn gitlab_repo_urls_target_repository_endpoints() {
    let repo = gitlab()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .build();
    let page = gitlab()
        .pagination()
        .request()
        .limit(50)
        .cursor("2")
        .build();

    assert_eq!(
        repo.url().value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fvcs-providers-rs"
    );
    assert_eq!(
        repo.branches(Some(&page)).value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fvcs-providers-rs/repository/branches?per_page=50&page=2"
    );
    assert_eq!(
        repo.commits(None).value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fvcs-providers-rs/repository/commits"
    );
}

#[test]
fn gitlab_repo_urls_target_collection_endpoints() {
    let page = gitlab().pagination().request().limit(25).build();
    let repo = gitlab().repo();
    let collection = repo.collection();
    let list_query = repo.query().list(Some(page.clone()));
    let search_query = repo.query().search("vcs provider", Some(page));

    assert_eq!(
        collection.list(&list_query).value(),
        "https://gitlab.com/api/v4/projects?per_page=25"
    );
    assert_eq!(
        collection.search(&search_query).value(),
        "https://gitlab.com/api/v4/projects?search=vcs%20provider&per_page=25"
    );
}

#[test]
fn gitlab_repo_requests_build_mutation_requests() {
    let repo = gitlab()
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
    let create_request = gitlab().repo().collection().create(&draft);
    let update_request = repo.update(&patch);
    let delete_request = repo.delete();

    assert_eq!(create_request.method(), &RequestMethod::Post);
    assert!(create_request.body().is_some());
    assert_eq!(update_request.method(), &RequestMethod::Put);
    assert!(update_request.body().is_some());
    assert_eq!(delete_request.method(), &RequestMethod::Delete);
}
