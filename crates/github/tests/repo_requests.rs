use vcs_provider_core::{
    RepositoryDraftBuilder, RepositoryPatchBuilder, RequestMethod, Visibility,
};
use vcs_provider_github::github;

#[test]
fn github_repo_urls_target_repository_endpoints() {
    let repo = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .build();
    let page = github()
        .pagination()
        .request()
        .limit(50)
        .cursor("2")
        .build();

    assert_eq!(
        repo.url().value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs"
    );
    assert_eq!(
        repo.branches(Some(&page)).value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/branches?per_page=50&page=2"
    );
    assert_eq!(
        repo.commits(None).value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/commits"
    );
}

#[test]
fn github_repo_urls_target_collection_endpoints() {
    let page = github().pagination().request().limit(25).build();
    let repo = github().repo();
    let collection = repo.collection();
    let list_query = repo.query().list(Some(page.clone()));
    let search_query = repo.query().search("vcs provider", Some(page));

    assert_eq!(
        collection.list(&list_query).value(),
        "https://api.github.com/user/repos?per_page=25"
    );
    assert_eq!(
        collection.search(&search_query).value(),
        "https://api.github.com/search/repositories?q=vcs%20provider&per_page=25"
    );
}

#[test]
fn github_repo_requests_build_mutation_requests() {
    let repo = github()
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
    let create_request = github().repo().collection().create(&draft);
    let update_request = repo.update(&patch);
    let delete_request = repo.delete();

    assert_eq!(create_request.method(), &RequestMethod::Post);
    assert!(create_request.body().is_some());
    assert_eq!(update_request.method(), &RequestMethod::Patch);
    assert!(update_request.body().is_some());
    assert_eq!(delete_request.method(), &RequestMethod::Delete);
}
