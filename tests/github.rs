#![cfg(feature = "github")]

#[path = "github/auth_organizations.rs"]
mod auth_organizations;
#[path = "github/code_review_requests.rs"]
mod code_review_requests;
#[path = "github/collaboration_hydration.rs"]
mod collaboration_hydration;
#[path = "github/conformance.rs"]
mod conformance;
#[path = "github/issue_requests.rs"]
mod issue_requests;
#[path = "github/manager_facade.rs"]
mod manager_facade;
#[path = "github/pagination_hydration.rs"]
mod pagination_hydration;
#[path = "github/pipeline_hydration.rs"]
mod pipeline_hydration;
#[path = "github/pipeline_requests.rs"]
mod pipeline_requests;
#[path = "github/provider.rs"]
mod provider;
#[path = "github/release_requests.rs"]
mod release_requests;
#[path = "github/repo_hydration.rs"]
mod repo_hydration;
#[path = "github/repo_requests.rs"]
mod repo_requests;
#[path = "github/runtime.rs"]
mod runtime;
#[path = "github/transport_facade.rs"]
mod transport_facade;
