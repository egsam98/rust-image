create table images (
    id integer primary key AUTOINCREMENT not null,
    title varchar not null,
    content_type varchar not null,
    bytes binary not null,
    preview_bytes binary
)