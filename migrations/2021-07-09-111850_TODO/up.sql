-- Your SQL goes here

create table TODO (
  id serial primary key not null,
  name text not null,
  body text not null
);
