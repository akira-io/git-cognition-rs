#[path = "core/support/mod.rs"]
mod support;

#[path = "core/support/local_git_conflict.rs"]
mod local_git_conflict_support;

#[path = "core/auth.rs"]
mod auth;
#[path = "core/capabilities.rs"]
mod capabilities;
#[path = "core/errors.rs"]
mod errors;
#[path = "core/http.rs"]
mod http;
#[path = "core/local_git.rs"]
mod local_git;
#[path = "core/local_git_cognition.rs"]
mod local_git_cognition;
#[path = "core/local_git_support.rs"]
mod local_git_support;
#[path = "core/middleware.rs"]
mod middleware;
#[path = "core/pagination.rs"]
mod pagination;
#[path = "core/provider_contracts.rs"]
mod provider_contracts;
#[path = "core/rate_limit.rs"]
mod rate_limit;
#[path = "core/repos.rs"]
mod repos;
#[path = "core/retry.rs"]
mod retry;
#[path = "core/runtime.rs"]
mod runtime;
#[path = "core/telemetry.rs"]
mod telemetry;
#[path = "core/transport.rs"]
mod transport;
#[path = "core/url.rs"]
mod url;
