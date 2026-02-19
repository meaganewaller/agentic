# Layering Model

Layer precendence:

1. base.yaml
2. profile-<name>.yaml
3. machine-<hostname>.yaml
4. project agentic.yaml

Object keys deep-merge.
Arrays override by default.

Special arrays (e.g., `agents`) merge by `name`.

Example:

base:
```
agents:
  - name: rails
    temperature: 0.2
```

profile:
```
agents:
  - name: rails
    temperature: 0.0
```

Result:

```
agents:
  - name: rails
    temperature: 0.0
```