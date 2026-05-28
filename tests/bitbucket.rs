#![cfg(feature = "bitbucket")]

#[path = "bitbucket/auth_organizations.rs"]
mod auth_organizations;
#[path = "bitbucket/code_review_requests.rs"]
mod code_review_requests;
#[path = "bitbucket/collaboration_hydration.rs"]
mod collaboration_hydration;
#[path = "bitbucket/conformance.rs"]
mod conformance;
#[path = "bitbucket/issue_hydration.rs"]
mod issue_hydration;
#[path = "bitbucket/issue_requests.rs"]
mod issue_requests;
#[path = "bitbucket/manager_facade.rs"]
mod manager_facade;
#[path = "bitbucket/pagination_hydration.rs"]
mod pagination_hydration;
#[path = "bitbucket/pipeline_hydration.rs"]
mod pipeline_hydration;
#[path = "bitbucket/pipeline_requests.rs"]
mod pipeline_requests;
#[path = "bitbucket/provider.rs"]
mod provider;
#[path = "bitbucket/repo_hydration.rs"]
mod repo_hydration;
#[path = "bitbucket/repo_requests.rs"]
mod repo_requests;
#[path = "bitbucket/runtime.rs"]
mod runtime;
#[path = "bitbucket/transport_facade.rs"]
mod transport_facade;
