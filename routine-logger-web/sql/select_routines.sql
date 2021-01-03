select
    routine_id
    ,user_id
    ,name
    ,unit_period
    ,target_quantity
    ,to_char(insert_datetime, 'YYYY-MM-DD HH24:MI:SS') as insert_datetime
from routines
order by insert_datetime
