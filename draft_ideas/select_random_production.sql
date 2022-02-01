select p.production
from production p
inner join non_terminal_symbol nts
on nts.id = p.non_terminal_symbol and nts.name = $1
order by random()
limit 1;
