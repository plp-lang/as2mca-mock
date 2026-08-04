# Mock-сервер для сервера приложений 2 MCA АБС ЦФТ

## Пример использования

### 1. Запуск сервера в режиме записи (cache+proxy)

Загружаем официальный образ:

```sh
docker pull plp-lang/as2mca-mock:latest
```

Запускаем контейнер в режиме `cache+proxy`.
В этом режиме сервер проксирует запросы к серверу приложений и одновременно сохраняет все ответы в локальный кэш:

```sh
docker run --rm --name as2mca-mock -p 3000:3000 \
    -u root \
    -v $(pwd)/.cache:/data \
    -e AS2MCA_MOCK_MODE="cache+proxy" \
    -e AS2MCA_MOCK_URL="http://127.0.0.1:7000/TEST/" \
    -e AS2MCA_MOCK_WEB_APP_NAME="TEST" \
    -e AS2MCA_MOCK_USERNAME="IBS" \
    -e AS2MCA_MOCK_PASSWORD="password" \
    plp-lang/as2mca-mock:latest
```
> Примечание: Убедитесь, что локальная директория `./.cache` доступна для чтения и записи пользователю внутри контейнера.

### 2. Настройка и запуск тестов клиента

Кнонируем тестируемый проект, например [as2mca-api-ts](https://github.com/plp-lang/as2mca-api-ts):

```sh
git clone https://github.com/plp-lang/as2mca-api-ts
cd as2mca-api-ts
bun install
```

Создайте файл `.env` (или экспортируйте переменные) и укажите адрес запущенного **mock-сервера**:

```sh
AS2MCA_API_USERNAME="IBS"
AS2MCA_API_PASSWORD="password"
AS2MCA_API_URL="http://localhost:3000/NIGHT/"
```

Запустите тесты:

```sh
bun test
```

Если тесты прошли успешно — вы всё сделали правильно! В директории `./.cache` должен появиться файл `cache.db`.

### 3. Переход в автономный режим (`cache`)

Остановите текущий контейнер и запустите его заново в режиме чтения из кэша (cache). 
Теперь сервер не будет делать внешние сетевые вызовы, а будет отвечать строго на основе ранее записанных данных:

```sh
docker stop as2mca-mock
docker run -d --name as2mca-mock -p 3000:3000 \
    -u root \
    -v $(pwd)/.cache:/data \
    -e AS2MCA_MOCK_MODE="cache" \
    -e AS2MCA_MOCK_WEB_APP_NAME="TEST" \
    plp-lang/as2mca-mock:latest
```

Снова запускаем тесты:

```sh
bun test
```

Если тесты прошли успешно, значит сервер успешно воспроизвел сохраненные ответы.
Файл `./.cache/cache.db` можно транспортировать для локальной разработки,
коммитить в репозиторий (через Git LFS), использовать в CI/CD пайплайнах.

## Разработчику

Локальная сборка образа:

```sh
docker build -t as2mca-mock:dev .
# или со своей версией Rust
docker build --build-arg RUST_VERSION="1.95" -t as2mca-mock:dev .
```

Запуск локально собранного образа:

```sh
docker run --rm -p 3000:3000 \
    -u root \
    -v $(pwd)/.cache:/data \
    -e AS2MCA_MOCK_MODE="cache" \
    -e AS2MCA_MOCK_WEB_APP_NAME="TEST" \
    plp-lang/as2mca-mock:latest
```
