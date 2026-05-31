---
"@biomejs/biome": patch
---

Fixed [#10330](https://github.com/biomejs/biome/issues/10330): Biome now preserves multiline Vue interpolation layout when a component has a single `{{ ... }}` child, avoiding borrowed closing-tag formatting.
