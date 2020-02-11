## Cloud SQL Proxyのインストール

```bash
# gcloud components install cloud_sql_proxy # だめっぽい
curl -o cloud_sql_proxy https://dl.google.com/cloudsql/cloud_sql_proxy.darwin.amd64
chmod +x cloud_sql_proxy
```

## Cloud SQLの準備

1. コンソールでSQL作成
postgresqlを作成した。password: ikedaosushi

2. 情報の確認

```bash
gcloud sql instances describe polls-instance
# connectionName: ikedaosushi:asia-northeast1:polls-instance
```

3. Cloud SQL Proxyで接続

```bash
cloud_sql_proxy -instances="ikedaosushi:asia-northeast1:polls-instance"=tcp:5433
```

4. User作成

コンソールから

username: polluser, password: pollpassword

5. DB作成

```bash
psql --host 127.0.0.1 --user postgres --password --port 5433
```

```sql
CREATE DATABASE polls;
```

## Django Setup

1. ライブラリインストール

```sh
pip install -r requirements.txt
```

2. migration

```sh
python manage.py makemigrations
python manage.py makemigrations polls
python manage.py migrate
```

3. サーバー起動

```sh
python manage.py runserver
```

```sh
open http://localhost:8000
```

4. スーパーユーザー作成

```bash
python manage.py createsuperuser
```

username: pollsuperuser, password: pollsuperpassword

## デプロイ

```bash
python manage.py collectstatic
```