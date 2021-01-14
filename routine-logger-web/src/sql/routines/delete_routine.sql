delete from routines
where routine_id = $1
returning
    routine_id
    ,user_id
    ,name
    ,unit_period
    ,target_quantity
    ,unit
    ,to_char(insert_datetime, 'YYYY-MM-DD HH24:MI:SS') as insert_datetime
