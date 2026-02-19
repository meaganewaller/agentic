# Architecture

`agentic` is a compiler pipeline.

```
layers -> merge -> validate -> resolve agents -> adapter compile -> write
```

## Pipeline Phases

1. Load layers (base, profile, machine, project)
2. Deep merge with keyed-array rules
3. Validate merged config via JSON schema
4. Resolve active agent
5. Load agent YAML
6. Load referenced skills
7. Build final system prompt bundle
8. Pass merged config + resolved prompt into adapters
9. Write vendor-specific output

Each phase is deterministic and testable in isolation.