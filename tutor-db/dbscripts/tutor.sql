drop table if exists ezy_tutor;

/* Table Schema */
create table ezy_tutor (
       tutor_id serial primary key,
       tutor_name varchar(200) not null,
       tutor_pic_url varchar(200) not null,
       tutor_profile varchar(2000) not null
);

grant all privileges on table ezy_tutor to siabard;
