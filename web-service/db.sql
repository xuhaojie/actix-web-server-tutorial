drop TABLE if exists course;

CREATE TABLE course(
	id serial primary key,
	teacher_id int not null,
	name VARCHAR(140) not null,
	time TIMESTAMP default now(),
	description VARCHAR(2000),
	format VARCHAR(30),
	structure VARCHAR(200),
	duration VARCHAR(200),
	price int,
	language VARCHAR(30),
	level VARCHAR(50)
);

drop TABLE if exists teacher;
CREATE TABLE teacher(
	id serial primary key,
	name VARCHAR(100) not null,
	picture_url VARCHAR(200) not null,
	profile VARCHAR(2000) not null
);


// bad
CREATE TABLE course(
	id serial primary key,
	teacher_id int not null,
	name VARCHAR(140) not null,
	time TIMESTAMP default now(),
	format VARCHAR(30),
	structure VARCHAR(200),
	duration VARCHAR(200),
	price int,
	language VARCHAR(30),
	level VARCHAR(50)
);