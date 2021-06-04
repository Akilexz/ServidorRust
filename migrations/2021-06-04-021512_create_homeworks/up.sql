-- Your SQL goes here

create table homeworks (
    id serial primary key,
    title varchar not null,
    published boolean not null default 'f'
)