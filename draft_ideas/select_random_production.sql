select p.id, p.production
from production p
where p.non_terminal_symbol = 1
order by random()
limit 1;