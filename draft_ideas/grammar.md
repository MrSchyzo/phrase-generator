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

### `{x:name:y}`

x -> grammar order (depends on x-1, if x-1 exists, and context; "-" resets dependency)
name -> identifier of the NTS
y -> semantic order (depends on y-1, if y-1 exists, and context; "-" resets dependency)


### `<x:s_tag:y>`

x -> grammar order (depends on x-1, if x-1 exists, and context; "-" resets dependency)
s_tag -> semantic tag to pick
y -> semantic order (depends on y-1, if y-1 exists, and context; "-" resets dependency)

---

### Non-repetition of words

Everytime I want to select a word, I check my HashSet to exclude some words;
When I select a new word that is `non_repeatable`, I add it to my hashset;
