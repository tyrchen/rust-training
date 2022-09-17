create table todos(
    id sortableid primary key default generate_sortable_id(),
    title varchar(255) not null,
    metadata json,

    check (
        check_json_schema(metadata,
            '{
                "type": "object",
                "properties": {
                    "tags": {
                        "type": "array",
                        "items": {
                            "type": "string",
                            "maxLength": 4
                        }
                    }
                }
            }'
        )
    )
);

-- this will succeed
insert into todos(title, metadata) values('buy stuff', '{"tags": ["shopping"]}');

-- this will fail
insert into todos(title, metadata) values('bad', '{"tags": [{"a": 1}]}');
