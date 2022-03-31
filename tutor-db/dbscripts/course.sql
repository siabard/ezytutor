drop table if exists ezy_course cascade;

create table ezy_course (
       course_id serial primary key,
       tutor_id int not null,
       course_name varchar(140) not null,
       course_description varchar(2000),
       course_format varchar(30),
       course_structure varchar(200),
       course_duration varchar(30),
       course_price INT,
       course_language varchar(30),
       course_level varchar(30),
       posted_time timestamp default now(),
       CONSTRAINT fk_tutor
       FOREIGN KEY(tutor_id)
       REFERENCES ezy_tutor(tutor_id)
);

grant all privileges on table ezy_course to siabard;
