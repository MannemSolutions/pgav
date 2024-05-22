# pgav

## TL/DR

### Binary

If you want to download the binary and run the tool directly:

```
curl -OL https://github.com/MannemSolutions/pgav/releases/download/v0.1.1/pgav_v0.1.1_x86_64-unknown-linux-musl.zip
unzip pgav*.zip
./pgav
```

### Container

If you wanna use the container instead:

```
docker run -e PGHOST=postgres,PGUSER=postgres,PGDATABASE=postgres,PGPASSWORD=password123 ghcr.io/mannemsolutions/pgav
```

**Note** that with the container you can set environment variables and leave other arguments default.

### Arguments

If you wanna change options, you can:

- Enable debug using the `--debug` option (default off)
  Alternatively you can set environment variable PGAVDEBUG to any non-empty value
- Select a custom timeout by supplying the `--timeout [t]` option (default 10s)
  timeout can be specified as any supported duration (e.a. 1m, 1s, 1h for 1 minute, 1 second or 1 hour resp.)
  Alternatively you can set environment variable PGAVTIMEOUT accordingly.
- Select a custom sleeptime by supplying the `--sleeptime [t]` option (default 5s)
  sleeptime can be specified as any supported duration (e.a. 1m, 1s, 1h for 1 minute, 1 second or 1 hour resp.)
  Alternatively you can set environment variable PGAVSLEEPTIME accordingly.
- Supply a dsn using the `--dsn [custom dsn]` option (default emptystring)
  **note** we highly recommend using standard PostgreSQL environment vars over suppying a DSN.
  Alternatively you can set environment variable PGAVDSN accordingly.

Example:

```
./pgav --debug --timeout 5s --sleeptime 1s --dsn 'host=localhost user=me password=very_secret'
```

This will do the following:

- Connect to postgres on localhost:5432, as user me, with password very_secret (--dsn '...')
- Create a table called pgav
- Insert a value with current datetime
- loop:
  - sleep 1 second (--sleeptime 1s)
  - read back record, check time difference (should be about 1s), when longer then 5s (--timeout 5s) log a message
  - update record (round and round we go)
- For all statements debug information is printed as well (--debug)

### Example output

This could be the output of running the tool:

```text
2024-11-02T08:54:27.203Z INFO  [pgav] Initializing
2024-11-02T08:54:27.208Z INFO  [pgav] dsn: dbname='postgres' host='localhost' password='*****' port='5432' sslcert='' sslcrl='' sslkey='' sslmode='prefer' sslrootcert='' user='me'
2024-11-02T08:54:27.217Z DEBUG [select] retrieving timestamp
2024-11-02T08:54:27.218Z DEBUG [tokio_postgres::prepare] preparing query s0: select min(extract(epoch from now()-last))::real from pgavailability
2024-11-02 08:54:27.219 UTC [2831] ERROR:  relation "pgavailability" does not exist at character 55
2024-11-02 08:54:27.219 UTC [2831] STATEMENT:  select min(extract(epoch from now()-last))::real from pgavailability
2024-11-02T08:54:27.221Z DEBUG [create] creating table
2024-11-02T08:54:27.221Z DEBUG [tokio_postgres::prepare] preparing query s1: create table pgavailability (last timestamp)
2024-11-02T08:54:27.221Z DEBUG [tokio_postgres::query] executing statement s1 with parameters: []
2024-11-02T08:54:27.223Z DEBUG [insert] inserting timestamp 2
2024-11-02T08:54:27.223Z DEBUG [tokio_postgres::prepare] preparing query s2: insert into pgavailability values(now())
2024-11-02T08:54:27.224Z DEBUG [tokio_postgres::query] executing statement s2 with parameters: []
2024-11-02T08:54:27.224Z DEBUG [select] retrieving timestamp
2024-11-02T08:54:27.224Z DEBUG [tokio_postgres::prepare] preparing query s3: select min(extract(epoch from now()-last))::real from pgavailability
2024-11-02T08:54:27.225Z DEBUG [tokio_postgres::query] executing statement s3 with parameters: []
2024-11-02T08:54:27.226Z DEBUG [check] expired=0.00129
2024-11-02T08:54:27.226Z DEBUG [update] updating timestamp
2024-11-02T08:54:27.226Z DEBUG [tokio_postgres::prepare] preparing query s4: update pgavailability set last = now()
2024-11-02T08:54:27.226Z DEBUG [tokio_postgres::query] executing statement s4 with parameters: []
2024-11-02T08:54:27.227Z DEBUG [sleep] sleep 0.99871 seconds
2024-11-02T08:54:28.235Z DEBUG [select] retrieving timestamp
2024-11-02T08:54:28.235Z DEBUG [tokio_postgres::prepare] preparing query s5: select min(extract(epoch from now()-last))::real from pgavailability
2024-11-02T08:54:28.237Z DEBUG [tokio_postgres::query] executing statement s5 with parameters: []
2024-11-02T08:54:28.237Z DEBUG [check] expired=1.010447
2024-11-02T08:54:28.237Z DEBUG [update] updating timestamp
```

### Environment variables

pg_tps_optimizer supports these environment variables to be used.

```bash
PGAVDEBUG=yesplease                     # or any other non-zero value
PGAVSLEEPTIME=5s                        # 5s is default
PGAVTIMEOUT=10s                         # 10s is default
PGAVDSN='host=/tmp user=me dbname=mydb' # (or any other DSN paramters)
PGHOST=/tmp                             # /tmp is default, any other path, hostname or ip can be used instead
PGUSER=$(id -u -n)                      # defaults to current user
PGDATABASE=${PGUSER}                    # defaults to user
PGPASSWORD=**\***                       # defaults to emptystring which should be set to any dummy password for ident, trust, cert, etc.
PGSSLMODE=prefer                        # (or any other supported SSL mode)
PGSSLCERT=~/.postgresql/postgresql.crt
PGSSLKEY=~/.postgresql/postgresql.key
PGSSLROOTCERT=~/.postgresql/root.crt
PGSSLCRL=~/.postgresql/root.crl
```

**Note** that Arguments have precedence over Environment variables.
**Also note** that all options not set in DSN default to environment variables, which default to known defaults.

## General information

This project is about testing availability, which includes recovery times of failures, and logging exact failure moments and durations.

## License

We love our software and want them to grow. Therefore we embrace open Source, and invite you to join the community and contribute.
Therefore we feel that GPL-3.0 license best meets the needs of our users and the community.
In general:

- feel free to use, distribute and even change the code
- if you wanna distribute changed versions we wuld appreciate if yu also upstream your changes so we can expand this project to be even more awesome
  Thank you...

## Contributing

We are open source, and are always open to contributions.

- If you experience issues, please submit a github [issue](https://github.com/MannemSolutions/pgav/issues).
- If you wanna expand features, or fix bugs, please submit a github [Pull Request](https://github.com/MannemSolutions/pgav/pulls).
