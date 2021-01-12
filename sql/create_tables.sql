drop table if exists routines;
create table routines(
    routine_id serial not null
    ,user_id int not null default 0
    ,name varchar(255) not null
    ,unit_period varchar(255) not null default ''
    ,target_quantity double precision not null default 0.0
    ,unit varchar(255) not null default ''
    ,insert_datetime timestamp with time zone not null default now()
    ,primary key(routine_id)
)
;

drop table if exists routine_logs;
create table routine_logs(
    log_id serial not null
    ,user_id int not null default 0
    ,routine_id int not null
    ,quantity numeric not null default 0.0
    ,note text not null default ''
    ,date_of_activity date not null default current_date
    ,insert_datetime timestamp with time zone not null default now()
    ,primary key(log_id)
)
;