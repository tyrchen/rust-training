---
title: AI Based Postgres Assistant
id: ai-based-postgres-assistant
proposal_date: 2023-04-01 15:21:39
status: pre-draft
---

## Summary

Use GPT-3 to generate SQL queries based on natural language input.

## Motivation

The goal of this project is to make it easier for developers to interact with databases. The current process of writing SQL queries is tedious and error-prone. This project aims to solve this problem by using GPT-3 to generate SQL queries based on natural language input.

## Guide-level explanation

We want to provide several functions in this extension:

### h(prompt, schema = "public", force_execute = true)

User can provide a prompt to the function, and the function will return a SQL query based on the prompt.

- h('find all users'): it will return 'SELECT * FROM users', and if the SQL is a qeury, execute it and return both the query and the result.
- h('SELECT * FROM users'): it will explain the query in natural language
- h('generate 10 sample data for users table'): it will return SQL like 'INSERT INTO users (name, age) VALUES ('John', 20), ('Mary', 30), ...', for this kind of SQL that mutates the database, it will not execute the SQL, but return the SQL. Unless user run it with `h('...', true)`.
- h('list all tables'): it will mimic the behavior of `\dt` in psql, and return a list of tables in the database.

Before h() is being called, we need to find a way to retrieve all the schema information for the current database. Then feed it to ChatGPT. We need:

- all custom type schemas: in SQL format, e.g. CREATE TYPE(...)
- all table schemas: in SQL format, e.g. CREATE TABLE(...)
- all view schemas: in SQL format, e.g. CREATE VIEW(...)
- all materialized view schemas: in SQL format, e.g. CREATE MATERIALIZED VIEW(...)
- all function schemas: in SQL format, e.g. CREATE FUNCTION(...)
- all trigger schemas: in SQL format, e.g. CREATE TRIGGER(...)

## Reference-level explanation

To retrieve all tables under a namespace, we can use the following function (thanks to ChatGPT):

```sql
CREATE SCHEMA IF NOT EXISTS gassist;
CREATE OR REPLACE FUNCTION gassist.tables(p_schema_name TEXT)
RETURNS TABLE(create_table_statement TEXT) AS $$
BEGIN
  RETURN QUERY
  WITH table_columns AS (
    SELECT
      n.nspname AS schema_name,
      c.relname AS table_name,
      a.attname AS column_name,
      pg_catalog.format_type(a.atttypid, a.atttypmod) AS column_type,
      a.attnotnull AS is_not_null,
      a.attnum AS column_position
    FROM
      pg_catalog.pg_attribute a
      JOIN pg_catalog.pg_class c ON a.attrelid = c.oid
      JOIN pg_catalog.pg_namespace n ON c.relnamespace = n.oid
    WHERE
      a.attnum > 0
      AND NOT a.attisdropped
      AND n.nspname = p_schema_name
      AND c.relkind = 'r'
  ),
  table_constraints AS (
    SELECT
      tc.constraint_name,
      tc.table_schema,
      tc.table_name,
      kcu.column_name,
      tc.constraint_type
    FROM
      information_schema.table_constraints tc
      JOIN information_schema.key_column_usage kcu
        ON tc.constraint_catalog = kcu.constraint_catalog
        AND tc.constraint_schema = kcu.constraint_schema
        AND tc.constraint_name = kcu.constraint_name
    WHERE
      tc.constraint_type IN ('PRIMARY KEY', 'FOREIGN KEY', 'UNIQUE')
      AND tc.table_schema = p_schema_name
  ),
  formatted_columns AS (
    SELECT
      tc.schema_name,
      tc.table_name,
      tc.column_name,
      tc.column_type,
      tc.is_not_null,
      tc.column_position,
      STRING_AGG(
        tcs.constraint_type || ' (' || tc.column_name || ')',
        ', '
        ORDER BY tcs.constraint_type DESC
      ) AS column_constraints
    FROM
      table_columns tc
      LEFT JOIN table_constraints tcs
        ON tc.schema_name = tcs.table_schema
        AND tc.table_name = tcs.table_name
        AND tc.column_name = tcs.column_name
    GROUP BY
      tc.schema_name,
      tc.table_name,
      tc.column_name,
      tc.column_type,
      tc.is_not_null,
      tc.column_position
  ),
  create_table_statements AS (
    SELECT
      fc.schema_name,
      fc.table_name,
      STRING_AGG(
        fc.column_name || ' ' || fc.column_type || (CASE WHEN fc.is_not_null THEN ' NOT NULL' ELSE '' END) || COALESCE(' ' || fc.column_constraints, ''),
        ', '
        ORDER BY fc.column_position
      ) AS formatted_columns
    FROM
      formatted_columns fc
    GROUP BY
      fc.schema_name,
      fc.table_name
  )
  SELECT
    'CREATE TABLE ' || schema_name || '.' || table_name || ' (' || formatted_columns || ');' AS create_table_statement
  FROM
    create_table_statements;
END;
$$ LANGUAGE plpgsql;
```

The result of this query looks like this:

```sql
CREATE TABLE rsvp.reservation_changes (id integer NOT NULL PRIMARY KEY (id), reservation_id bigint NOT NULL, old jsonb, new jsonb, op rsvp.reservation_update_type NOT NULL);
CREATE TABLE rsvp.reservations (id bigint NOT NULL PRIMARY KEY (id), user_id character varying(64) NOT NULL, status rsvp.reservation_status NOT NULL, resource_id character varying(64) NOT NULL, timespan tstzrange NOT NULL, note text);
CREATE TABLE rsvp.server_read_cursor (server_id character varying(64) NOT NULL PRIMARY KEY (server_id), last_change_id bigint NOT NULL);
```

To retrieve all views under a namespace, we can use the following function (thanks to ChatGPT):

```sql
CREATE OR REPLACE FUNCTION gassist.views(p_schema_name TEXT)
RETURNS TABLE(create_view_statement TEXT) AS $$
BEGIN
  RETURN QUERY
  SELECT
    'CREATE VIEW ' || n.nspname || '.' || c.relname || ' AS ' || pg_get_viewdef(c.oid)
  FROM
    pg_catalog.pg_class c
    JOIN pg_catalog.pg_namespace n ON c.relnamespace = n.oid
  WHERE
    c.relkind = 'v' -- Select views
    AND n.nspname = p_schema_name;
END;
$$ LANGUAGE plpgsql;
```

To retrieve all materialized views under a namespace, we can use the following function (thanks to ChatGPT):

```sql
CREATE OR REPLACE FUNCTION gassist.mviews(p_schema_name TEXT)
RETURNS TABLE(create_materialized_view_statement TEXT) AS $$
BEGIN
  RETURN QUERY
  SELECT
    'CREATE MATERIALIZED VIEW ' || n.nspname || '.' || c.relname || ' AS ' || pg_get_viewdef(c.oid) || ';'
  FROM
    pg_catalog.pg_class c
    JOIN pg_catalog.pg_namespace n ON c.relnamespace = n.oid
  WHERE
    c.relkind = 'm' -- Select materialized views
    AND n.nspname = p_schema_name;
END;
$$ LANGUAGE plpgsql;
```

To retrieve all functions under a namespace, we can use the following function (thanks to ChatGPT):

```sql
CREATE OR REPLACE FUNCTION gassist.functions(p_schema_name TEXT)
RETURNS TABLE(create_function_statement TEXT) AS $$
BEGIN
  RETURN QUERY
  SELECT
    'CREATE OR REPLACE FUNCTION ' || n.nspname || '.' || p.proname || '(' || pg_get_function_arguments(p.oid) || ') RETURNS '
    || pg_get_function_result(p.oid) || ' AS $function_body$ ' || pg_get_functiondef(p.oid) || '$function_body$ LANGUAGE ' || l.lanname || ';'
  FROM
    pg_catalog.pg_proc p
    JOIN pg_catalog.pg_namespace n ON p.pronamespace = n.oid
    JOIN pg_catalog.pg_language l ON p.prolang = l.oid
  WHERE
    n.nspname = p_schema_name
    AND p.prokind = 'f'; -- Select functions
END;
$$ LANGUAGE plpgsql;
```

To retrieve all triggers under a namespace, we can use the following function (thanks to ChatGPT):

```sql
CREATE OR REPLACE FUNCTION gassist.triggers(p_schema_name TEXT)
RETURNS TABLE(create_trigger_statement TEXT) AS $$
BEGIN
  RETURN QUERY
  SELECT
    'CREATE TRIGGER ' || t.tgname
    || ' ' || CASE
      WHEN t.tgtype & 2 > 0 THEN 'BEFORE'
      WHEN t.tgtype & 4 > 0 THEN 'AFTER'
      WHEN t.tgtype & 64 > 0 THEN 'INSTEAD OF'
    END
    || ' ' || CASE
      WHEN t.tgtype & 8 > 0 THEN 'INSERT'
      WHEN t.tgtype & 16 > 0 THEN 'DELETE'
      WHEN t.tgtype & 32 > 0 THEN 'UPDATE'
    END
    || ' ON ' || n.nspname || '.' || c.relname
    || ' FOR EACH ' || CASE WHEN t.tgtype & 1 > 0 THEN 'ROW' ELSE 'STATEMENT' END
    || ' EXECUTE FUNCTION ' || np.nspname || '.' || p.proname || '();'
  FROM
    pg_catalog.pg_trigger t
    JOIN pg_catalog.pg_class c ON t.tgrelid = c.oid
    JOIN pg_catalog.pg_namespace n ON c.relnamespace = n.oid
    JOIN pg_catalog.pg_proc p ON t.tgfoid = p.oid
    JOIN pg_catalog.pg_namespace np ON p.pronamespace = np.oid
  WHERE
    n.nspname = p_schema_name
    AND NOT t.tgisinternal;
END;
$$ LANGUAGE plpgsql;
```

To retrieve all custom composite type, we can use the following function (thanks to ChatGPT):

```sql
CREATE OR REPLACE FUNCTION gassist.types(p_schema_name TEXT)
RETURNS TABLE(create_type_statement TEXT) AS $$
BEGIN
  RETURN QUERY
  SELECT
    'CREATE TYPE ' || n.nspname || '.' || t.typname || ' AS (' || string_agg(a.attname || ' ' || pg_catalog.format_type(a.atttypid, a.atttypmod), ', ') || ');'
  FROM
    pg_catalog.pg_type t
    JOIN pg_catalog.pg_namespace n ON t.typnamespace = n.oid
    JOIN pg_catalog.pg_class c ON t.typrelid = c.oid
    JOIN pg_catalog.pg_attribute a ON t.typrelid = a.attrelid
  WHERE
    n.nspname = p_schema_name
    AND t.typtype = 'c'
    AND c.relkind = 'c'
  GROUP BY
    n.nspname, t.typname;
END;
$$ LANGUAGE plpgsql;
```

To retrieve all enum types under a namespace, we can use the following function (thanks to ChatGPT):

```sql
CREATE OR REPLACE FUNCTION gassist.enums(p_schema_name TEXT)
RETURNS TABLE(create_type_statement TEXT) AS $$
BEGIN
  RETURN QUERY
  SELECT
    'CREATE TYPE ' || n.nspname || '.' || t.typname || ' AS ENUM (' || string_agg(quote_literal(e.enumlabel), ', ') || ');'
  FROM
    pg_catalog.pg_type t
    JOIN pg_catalog.pg_namespace n ON t.typnamespace = n.oid
    JOIN pg_catalog.pg_enum e ON t.oid = e.enumtypid
  WHERE
    n.nspname = p_schema_name
    AND t.typtype = 'e'
  GROUP BY
    n.nspname, t.typname;
END;
$$ LANGUAGE plpgsql;
```

To retrieve all indexes under a namespace:

```sql
CREATE OR REPLACE FUNCTION gassist.indexes(p_schema_name TEXT)
RETURNS TABLE(create_index_statement TEXT) AS $$
BEGIN
  RETURN QUERY
  SELECT
    indexdef
  FROM
    pg_indexes
  WHERE
    schemaname = p_schema_name
  ORDER BY
    tablename,
    indexname;
END;
$$ LANGUAGE plpgsql;
```

## Drawbacks

Why should we *not* do this?

## Rationale and alternatives

- Why is this design the best in the space of possible designs?
- What other designs have been considered and what is the rationale for not choosing them?
- What is the impact of not doing this?

## Prior art

TBD

## Unresolved questions

TBD

## Future possibilities

TBD
