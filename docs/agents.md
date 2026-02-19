# Agents & Skills

Agents are YAML definitions.
Skills are Markdown modules.

Agents reference skills by slug.

Final prompt assembly order:

1. Agent system_prompt
2. Skill content (in declared order)
3. Local config append rules
