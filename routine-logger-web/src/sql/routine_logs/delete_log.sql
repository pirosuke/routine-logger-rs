delete from routine_logs
where log_id = $1
returning
    log_id
    ,routine_id
    ,user_id
    ,quantity
    ,note
    ,to_char(date_of_activity, 'YYYY-MM-DD') as date_of_activity
