insert into routine_logs(
    routine_id
    ,quantity
    ,date_of_activity
)
values(
    $1
    ,$2
    ,to_date($3, 'YYYY-MM-DD')
)
returning
    log_id
    ,routine_id
    ,user_id
    ,quantity
    ,note
    ,to_char(date_of_activity, 'YYYY-MM-DD') as date_of_activity
