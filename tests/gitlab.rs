#![cfg(feature = "gitlab")]

#[path = "gitlab/auth_organizations.rs"]
mod auth_organizations;
#[path = "gitlab/code_review_requests.rs"]
mod code_review_requests;
#[path = "gitlab/collaboration_hydration.rs"]
mod collaboration_hydration;
#[path = "gitlab/conformance.rs"]
mod conformance;
#[path = "gitlab/issue_requests.rs"]
mod issue_requests;
#[path = "gitlab/manager_facade.rs"]
mod manager_facade;
#[path = "gitlab/pagination_hydration.rs"]
mod pagination_hydration;
#[path = "gitlab/pipeline_hydration.rs"]
mod pipeline_hydration;
#[path = "gitlab/pipeline_requests.rs"]
mod pipeline_requests;
#[path = "gitlab/provider.rs"]
mod provider;
#[path = "gitlab/release_requests.rs"]
mod release_requests;
#[path = "gitlab/repo_hydration.rs"]
mod repo_hydration;
#[path = "gitlab/repo_requests.rs"]
mod repo_requests;
#[path = "gitlab/runtime.rs"]
mod runtime;
#[path = "gitlab/transport_facade.rs"]
mod transport_facade;
