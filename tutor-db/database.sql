/* DROP table if it already exists */
drop table if exists ezy_course cascade;
drop table if exists ezy_tutor;

/* Table Schema */
create table ezy_tutor (
       tutor_id serial primary key,
       tutor_name varchar(200) not null,
       tutor_pic_url varchar(200) not null,
       tutor_profile varchar(2000) not null
);

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

grant all privileges on table ezy_tutor to siabard;
grant all privileges on table ezy_course to siabard;

/* testing data */
insert into ezy_tutor(tutor_id, tutor_name, tutor_pic_url, tutor_profile)
values(1, 'Merlene', 'http://s3.amazon.aws.com/pic1', 'Merlene is an experienced finance professional');

insert into ezy_tutor(tutor_id, tutor_name, tutor_pic_url, tutor_profile)
values(2, 'Frank', 'http://s3.amazon.aws.com/pic2', 'Frank is an experienced nuclear engineer');

insert into ezy_course(course_id, tutor_id, course_name, course_level, posted_time)
values(1,1,'First Course', 'Beginner', '2020-12-17 05:40:00');

insert into ezy_course(course_id, tutor_id, course_name, course_format, posted_time)
values(2, 1, 'Second course', 'ebook', '2020-12-18 05:45:00');
