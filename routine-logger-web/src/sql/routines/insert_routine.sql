insert into routines(
    name
    ,unit_period
    ,target_quantity
    ,unit
)
values(
    $1
    ,$2
    ,$3
    ,$4
)
returning
    routine_id
    ,user_id
    ,name
    ,unit_period
    ,target_quantity
    ,unit
    ,to_char(insert_datetime, 'YYYY-MM-DD HH24:MI:SS') as insert_datetime
