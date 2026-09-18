# Architecture boundaries

The current scaffold has an empty React surface and a Tauri host with no product commands. The following is the intended dependency direction when features are added:

```text
React presentation
    -> typed Tauri commands and events
Tauri composition boundary
    -> application use cases
Domain <- application -> platform and security ports
                         -> curated tool adapters
```

The frontend displays state and collects intent. It cannot choose executable paths, construct commands, access credentials, or mutate the host.

The Tauri host validates serialized input and delegates immediately. Domain and application crates do not depend on Tauri. Platform-specific effects implement explicit ports and are exercised through deterministic fakes in tests.

No installer, inventory, or upgrade path is implemented in this scaffold. Add crates and typed commands only when a scoped feature needs them. A frontend control does not grant a backend capability; any host effect requires a recorded decision and a threat-model update before implementation.
