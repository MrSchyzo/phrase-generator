select w.id as id, w."content" as content, w.non_repeatable as non_repeatable,
	coalesce((
		select array_agg(sem.semantic_tag) 
		from word_semantic sem 
		inner join semantic_tag s 
		on s.id = sem.semantic_tag and s.sticky 
		where sem.word = w.id
	), array[]::integer[]) as semantic_output,
	coalesce((
		select array_agg(gram.grammar_tag) 
		from word_grammar_requirements gram
		where gram.word = w.id
	), array[]::integer[]) as grammar_output
from word w
where array_contains_and_intersects(
	(
    select array_agg(ws.semantic_tag)
    from word_semantic ws
    where ws.word = w.id
  ),
	array[<SELECTED_SEMANTIC_TAGS_PLACEHOLDERS>]::integer[],
	array[<CONTEXTUAL_SEMANTIC_TAGS_PLACEHOLDERS>]::integer[]
)
and coalesce((
  select array_agg(wg.grammar_tag)
  from word_grammar_compatibility wg
  where wg.word = w.id
), array[]::integer[]) @> array[<CONTEXTUAL_GRAMMAR_TAGS_PLACEHOLDERS>]::integer[]
and (
  id not in (<USED_WORDS>)
)
order by random()
limit 1;
