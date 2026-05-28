---
"@biomejs/biome": patch
---

Fixed [#10330](https://github.com/biomejs/biome/issues/10330): Biome now preserves multiline Vue interpolation bodies instead of collapsing them to one line.

```diff
-{{ $t("nav.my-rooms") }}
+{{
+  $t("nav.my-rooms")
+}}
```
