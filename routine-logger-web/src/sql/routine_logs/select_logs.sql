select
    count(*) over() as count
    ,rl.log_id
    ,rl.routine_id
    ,r.name as routine_name
    ,rl.user_id
    ,rl.quantity
    ,r.unit
    ,rl.note
    ,to_char(rl.date_of_activity, 'YYYY-MM-DD') as date_of_activity
from routine_logs rl
inner join routines r on rl.routine_id = r.routine_id
order by rl.date_of_activity desc
offset $1 limit $2
