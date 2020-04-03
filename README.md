# Rust-обработчик изображений

##### Шаги, необходимые для запуска приложения:

1: Установить diesel CLI (с SQLite фичей)
```shell script
cargo install diesel_cli --no-default-features --features-sqlite
```
2: 
```shell script
diesel setup
diesel migration run
cargo run
```

После успешного запуска страница OpenAPI 3.0 документации сервиса доступна по адресу: http://localhost:8000
