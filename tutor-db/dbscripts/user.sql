drop table if exists ezy_user;

create table ezy_user (
       username varchar(20) primary key,
       tutor_id INT,
       user_password CHAR(100) not null
);


grant all privileges on table ezy_user to siabard;
