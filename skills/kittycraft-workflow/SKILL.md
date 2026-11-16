---
name: kittycraft-workflow
description: Step-by-step workflows in KittyCraft for breed matching, emergency toxin checks, and logging diary/sitter events.
---

# KittyCraft Workflows

1. **Match a cat breed:** Call `kitty_match_breeds` with housing (`wohnung`, `balkon`, `freigang`), household (`single`, `senioren`, `familie_kinder`, `hund`), and allergies flag.
2. **Check a toxin:** Call `kitty_toxic_check` with the substance/plant name; if severe/fatal (like lilies or paracetamol), highlight the nearest 24h veterinary hospital immediately.
3. **Log diary or sitter events:** Propose `kitty_diary_add` or `kitty_calendar_add` and execute only after user confirmation.
