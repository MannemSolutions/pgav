# pgav

## TL/DR

### Binary

If you want to download the binary and run the tool directly:
```
curl -OL https://github.com/MannemSolutions/pg_Ravailability/releases/download/v0.1.0/pgav_v0.1.0_x86_64-unknown-linux-musl.zip
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
- ...
- Needs to be documented


Example:
```
# Needs to be documented
```
This will do the following:
- ...
- Needs to be documented

### Example output
This could be the output of running the tool:
```
Initializing
dsn:dbname='postgres' host='postgres' sslcert='' sslcrl='' sslkey='' sslmode=prefer sslrootcert='' user=''
...
Needs to be documented
```

### Environment variables
pg_tps_optimizer supports these environment variables to be used.

PGHOST=/tmp
PGUSER=$(id -u -n) # Defaults to current user
PGDATABASE=${PGUSER}
PGPASSWORD=***** # Defaults to emptystring which basically cannot work. For ident, trust, cert, etc. Just set a dummy password.
PGSSLMODE=prefer
PGSSLCERT=~/.postgresql/postgresql.crt
PGSSLKEY=~/.postgresql/postgresql.key
PGSSLROOTCERT=~/.postgresql/root.crt
PGSSLCRL=~/.postgresql/root.crl

Others need to be documented...

**Note** that Arguments have precedence over Environment variables.

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
