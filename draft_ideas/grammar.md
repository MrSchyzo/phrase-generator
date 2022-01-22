```
S -> {0:N:N:F:F:Sub} {1:O(0)::PostIns} | {1:PreIns} {0:Sub} | {PreIns} {Sub} {PostIns} | {Curse} {Sub} | {Curse} {PreIns} {Sub} | {Curse} {Sub} {PostIns} | {Curse} {PreIns} {Sub} {PostIns}

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

### `{n:x:pg:y:ps:name}`

- `n`, `u32`: index of this NTS inside the production
- `x`, `OnNothing|OnContext|On(u32)|OnContextAnd(u32)`: index of the grammar dependency
  - `OnNothing` if this NTS is independent
  - `OnContext` if this NTS depends on context only
  - `On(u32)` if this NTS depends on NTS/placeholder only
  - `OnContextAnd(u32)` if this NTS depends on context and another NTS/placeholder
- `pg`, `bool`: whether it propagates grammar to the parent
- `y`, `Option<u32>`: index of the semantic dependency (same behaviour as `x`, but for semantics)
- `ps`, `bool`: whether it propagates semantics to the parent
- `name`, `String`: symbolic name

### Resulting grammar

### Resulting semantics
Simplest case, only one placeholder:
1- resulting semantics is the placeholder's semantics;

More placeholders:
1- resulting semantics is `placeholder1.semanticsIfPropagates() U ... U placeholderN.semanticsIfPropagates()`;

Different types of children:
1- resulting semantics is `child1.semanticsIfPropagates() U ... U childM.semanticsIfPropagates()`;

## Placeholder

### `<n:x:pg:y:ps:s_tags>`

- `n`, `u32`: index of this placeholder inside the production
- `x`, `OnNothing|OnContext|On(u32)|OnContextAnd(u32)`: index of the grammar dependency
  - `OnNothing` if this placeholder is independent
  - `OnContext` if this placeholder depends on context only
  - `On(u32)` if this placeholder depends on NTS/placeholder only
  - `OnContextAnd(u32)` if this placeholder depends on context and another NTS/placeholder
- `pg`, `bool`: whether it propagates grammar to the parent
- `y`, `OnNothing|OnContext|On(u32)|OnContextAnd(u32)`: index of the semantic dependency (same behaviour as `x`, but for semantics)
- `ps`, `bool`: whether it propagates semantics to the parent
- `s_tags`, `Vec<String>`: symbolic name of the semantic tags needed 

### Resulting grammar

### Resulting semantics
Simplest case, no dependencies and context:
1- find a word with all `s_tags`, let's name it `w`;
2- resulting semantics is `s_tags.filter(Tag::is_sticky) U w.semantics.filter(Tag::is_sticky)`.

With context only, no dependencies:
1- find a word with all `s_tags` and at least a tag in `context.tags`, let's name it `w`;
2- resulting semantics is `s_tags.filter(Tag::is_sticky) U w.semantics.filter(Tag::is_sticky)`

With dependencies and context:
1- find a word with all `s_tags` and at least a tag in `context.tags U dependency.tags`, let's name it `w`;
2- resulting semantics is `s_tags.filter(Tag::is_sticky) U w.semantics.filter(Tag::is_sticky)`

---

### Non-repetition of words

Everytime I want to select a word, I check my `HashSet` to exclude some words;
When I select a new word that is `non_repeatable`, I add it to my `HashSet`.
