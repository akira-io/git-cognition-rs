# Architecture

`git-cognition` is a single Rust crate organized into a provider-neutral substrate plus optional
provider drivers behind feature flags.

```text
Application
    |
    v
git-cognition (single crate)
    |
    +-- core surface       (always compiled)
    |
    +-- github driver       #[cfg(feature = "github")]    (default)
    +-- gitlab driver       #[cfg(feature = "gitlab")]
    +-- bitbucket driver    #[cfg(feature = "bitbucket")]
```

## Core surface

The crate root owns shared contracts and domain primitives, always compiled regardless of feature
selection:

- Provider contracts.
- Capability negotiation.
- Provider-neutral errors.
- Auth primitives.
- Resource primitives.
- Pagination primitives.
- Transport contracts.
- Middleware contracts.
- Telemetry contracts.

The core surface never references a provider driver.

## Provider drivers

Each provider lives in its own module behind a feature flag:

- `git_cognition::github` (feature `github`, default)
- `git_cognition::gitlab` (feature `gitlab`)
- `git_cognition::bitbucket` (feature `bitbucket`)

Driver modules implement the core contracts. They own provider-specific defaults, terminology
mapping, endpoint behavior, payload mapping, and extensions.

Adding a new provider must not require touching the core surface; it goes in as a new module
behind a new feature flag.

## Provider Contract

Every provider module exposes a type implementing `Provider`.

The provider describes:

- Provider identity.
- Display name.
- Capabilities.
- Repos contract.
- Default endpoint.
- Supported authentication modes.

Applications and managers consume providers through `Provider`, not through concrete provider
types.

## Repos Contract

`Repos` is the provider-neutral contract for repository operations.

It exposes:

- `get`
- `list`
- `search`
- `branches`
- `commits`

The contract is async-first and object-safe. Provider modules return futures through the shared
`BoxFuture` type, so applications can consume repository operations through trait objects without
depending on provider-specific types.

Provider modules own the mapping from provider endpoints to universal `Repository`, `Branch`, and
`Commit` resources. Until transport is configured, repos return
`CognitionError::TransportNotConfigured` instead of generating placeholder data.

## Registry Contract

`ProviderRegistry` stores providers by provider identity.

Applications compose the registry explicitly:

```rust
let registry = provider()
    .register(git_cognition::github::github())?
    .register(git_cognition::gitlab::gitlab())?
    .register(git_cognition::bitbucket::bitbucket())?
    .build();
```

The registry lives in the core surface and never references a provider driver directly.
Applications decide which providers to register.

## Dependency Rules

- The core surface does not depend on driver modules.
- Driver modules depend on the core surface.
- Provider-specific logic stays inside driver modules.
- Transport contracts live in the core surface; concrete HTTP transport must not leak provider
  payloads or HTTP client types.
- Resources, errors, capabilities, auth, middleware, pagination, and telemetry stay
  provider-neutral.

## Local Git Plane

`git-cognition` also exposes a local Git plane through `cognition().local()` (and the `git()`
shortcut). This plane is independent of the provider plane and needs no feature flag:

```text
Application
    |
    +-----> cognition().provider(github())   --> HTTP transport --> remote API
    |
    +-----> cognition().local().repo(path)   --> git CLI         --> local object DB
```

It does not use HTTP transport, auth middleware, or `Provider` drivers. It shells out to the local
`git` binary and returns the same provider-neutral resources where applicable (`Commit`, `Branch`).
The capability namespace is `LocalGitCapability`, separate from `Capability`. See
`24-local-git-cognition-reads.md` for the read surface and `22-end-to-end-usage.md` for the
combined provider + local flow.

```rust
use git_cognition::cognition;

let repository = cognition().local().repo("/workspace/project");
let head = repository.show("HEAD").file("README.md")?;
let graph = repository.log().limit(50).graph()?;
```
