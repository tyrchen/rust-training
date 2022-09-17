-- create a ids table
create table ids(id sortableid primary key default generate_sortable_id());

-- insert 10 rows
insert into ids select generate_sortable_id() from my_generate_series(1,1000000);

-- use between to select a range of ids
select * from ids where id between xxx and xxx;

--
select (extract(epoch from now()) * 1000)::bigint;

--
select generate_sorted_id((extract(epoch from timestamptz '2022-11-01 00:00:00 UTC')*1000)::bigint);
