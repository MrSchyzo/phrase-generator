```
S -> {0:Sub:0} {1:PostIns} | {1:PreIns} {0:Sub} | {PreIns} {Sub} {PostIns} | {Curse} {Sub} | {Curse} {PreIns} {Sub} | {Curse} {Sub} {PostIns} | {Curse} {PreIns} {Sub} {PostIns}

Sub -> Name

PreIns -> BadWord | {0:PartOf} {1:BadQual} {0:Of} | {1:BadQual} {0:PartOf} {0:Of} | {1:BadQual} {0:PartOf} {1:BadQual} {0:Of}

PostIns -> BadWord | BadWord BadQual | BadWord BadQualWithComplement0 | ... | BadWord BadQualWithComplementN | BadQual BadWord | BadQual BadWord BadQual | ... | BadQual BadWord BadQualWithComplementN

Curse -> accidenti a That | accidenti At | mannaggia | mannaggia Art | mannaggia That

BadWord -> BadQual | Noun

BadQual -> Adj | PastPart

BadQualWithComplement0 -> Adj come AnyS

BadQualWithComplementN -> PastPart da AnyS | PastPart ComplementModo da AnyS
```

---

## NTS

### `{n:x:y:name}`

- `n`, `u32`: index of this NTS inside the production
- `x`, `Option<u32>`: index of the grammar dependency
- `y`, `Option<u32>`: index of the semantic dependency (same behaviour as `x`, but for semantics)
  - `None` if this NTS' grammar is independent even of context
  - `Some(n)` if this NTS' grammar depends only on context
  - `Some(m)` if this NTS' grammar depends on context and another NTS/placeholder
- `name`, `String`: symbolic name

## Placeholder

### `<n:x:y:s_tags>`

- `n`, `u32`: index of this NTS inside the production
- `x`, `Option<u32>`: index of the grammar dependency
- `y`, `Option<u32>`: index of the semantic dependency (same behaviour as `x`, but for semantics)
  - `None` if this NTS' grammar is independent even of context
  - `Some(n)` if this NTS' grammar depends only on context
  - `Some(m)` if this NTS' grammar depends on context and another NTS/placeholder
- `s_tags`, `Vec<String>`: symbolic name of the semantic tags needed 

---

### Non-repetition of words

Everytime I want to select a word, I check my `HashSet` to exclude some words;
When I select a new word that is `non_repeatable`, I add it to my `HashSet`;
