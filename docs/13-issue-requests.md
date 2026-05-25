# Issue Requests

Issue request builders translate universal issue resources into provider-specific REST endpoints.

Use the provider facade when the provider is known:

```rust
let repo = github()
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .build();

let issue = github()
    .issue()
    .repo(repo)
    .id("42")
    .build();

let url = issue.url();
```

Use the collection builder for list URLs:

```rust
let repo = gitlab()
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .build();

let issue = gitlab().issue();
let page = gitlab()
    .pagination()
    .request()
    .limit(50)
    .build();
let query = issue.query().list(repo, Some(page));
let url = issue.collection().list(&query);
```

Use `vcs(driver)` when the provider is injected:

```rust
let provider = vcs(bitbucket());

let repo = provider
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .build();

let issue = provider
    .issue()
    .repo(repo)
    .id("42")
    .build();
```

## Provider Shapes

GitHub issues use the repository path:

```text
/repos/{owner}/{repo}/issues/{issue}
/repos/{owner}/{repo}/issues
```

GitLab issues use the URL-encoded project path:

```text
/api/v4/projects/{owner%2Frepo}/issues/{issue}
/api/v4/projects/{owner%2Frepo}/issues
```

Bitbucket Cloud issues use workspace and repository slug:

```text
/2.0/repositories/{workspace}/{repo_slug}/issues/{issue}
/2.0/repositories/{workspace}/{repo_slug}/issues
```

Pagination remains provider-neutral in the caller. Providers map it to their own query names.
