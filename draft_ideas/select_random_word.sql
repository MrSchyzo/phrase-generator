select w.id, w."content", w.non_repeatable,
	( 
		select array_agg(sem.semantic_tag) 
		from word_semantic sem 
		inner join semantic_tag s 
		on s.id = sem.semantic_tag and s.sticky 
		where sem.word = w.id
	) as semantic_output,
	(
		select array_agg(gram.grammar_tag) 
		from word_grammar gram
		where gram.word = w.id
	) as grammar_output
from word w
where array_contains_and_intersects(
	(
        select array_agg(ws.semantic_tag) 
        from word_semantic ws 
        where ws.word = w.id
    ), 
	array[]::integer[], 
	array[3,4]::integer[]
)
and (
    select array_agg(wg.grammar_tag) 
    from word_grammar wg 
    where wg.word = w.id
) && array[1,3,8]::integer[]
order by random()
limit 1;