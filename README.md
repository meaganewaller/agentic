# agentic

Layered, deterministic configuration and agent compiler for multi-vendor AI tooling.

`agentic` compiles layered YAML configuration + reusable agent/skill repositories into vendor-specific runtime configs (Claude, Codex, etc.).

## Why This Exists

Managing AI tooling across:

- Multiple machines
- Multiple projects
- Multiple vendors
- Multiple authentication contexts
- Reusable agent definitions

...becomes unmaintainable quickly.

`agentic` solves this by separating:

- Layered configuration
- Agent + skill definitions
- Vendor adapters
- Deterministic compilation

## Core Concepts

### 1. Layered Config

Config is merged in order:

```
base.yml
-> profile-.yaml
-> machine-.yaml
-> project agentic.yaml
```

Object keys deep-merge. Arrays normally override. Certain arrays (e.g., `agents`) merge by `name`.

### 2. Agents Repo

Agents and skills are defined by the user and should live in a reusable repo:

```
skills-and-agents-repo/
|- agents/
  |-- agents/
    |-- rails-architect.yaml
  |-- skills/
    |-- tdd.md 
```

Agents define:

- system_prompt
- skill slugs

Skills are markdown with optional YAML frontmatter.

### 3. Resolution

Pipeline:

```
merge layers
-> validate schema
-> resolve active agent
-> load skills
-> assemble system prompt
-> compile vendor config
```

### 4. Vendor Adapters

Adapters implement:

```rust
trait VendorAdapter {
    fn name(&self) -> &'static str;
    fn compile(&self, input: CompileInput<'_>) -> Result<Value>;
    fn default_output_path(&self) -> Result<PathBuf>;
}
```

Each vendor adapter transforms merged config + resolved prompt into its required format.

## Example

### layers/base.yaml

```yaml

agents_repo: "/path/to/agent/and/skills/repo"

agent:
  active: "rails-architect"

vendors:
  claude:
    model: "claude-3-opus"
```

### layers/work.yaml
```yaml
vendors:
  claude:
    temperature: 0.0
```

Run:

```bash
agentic --profile work
```

Output:

- Merged config
- Resolved system prompt
- Claude settings written to `~/.claude/settings.json`

## Features

- Deterministic layered merging
- Keyed array merging (agents, skills)
- JSON Schema validation (embedded)
- Snapshot-tested output
- Agent + skill resolution
- Multi-vendor adapter registry
- Auto machine detection
- Project-level config auto-detection
- Dry-run mode

## Development

Run tests:

```bash
cargo test
```

## Roadmap

- Parent-directory traversal for project config
- Agent version pinning
- Prompt bundle lockfile
- Adapter plugin loading
- Agent resolution caching
- Vendor feature flags

## Philosophy

Explicit and deterministic over magic and clever.
This tool performs as a **compiler**, not a runtime.