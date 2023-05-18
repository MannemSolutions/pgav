#!/bin/bash
set -e

docker-compose down --remove-orphans || echo new or partial install
docker rmi pgav_pgav || echo image was not there
if [ ! -e config/tls/int_server/certs/localhost.pem ]; then
  rm -rf config/tls
  docker-compose up chainsmith
fi
docker-compose up -d postgres

for ((i=0;i<60;i++)); do
  docker-compose exec -u postgres postgres pg_isready && break
  sleep 1
done

export PGUSER=postgres
export PGHOST=127.0.0.1
export PGSSLCERT=config/tls/int_client/certs/postgres.pem
export PGSSLKEY=config/tls/int_client/private/postgres.key.pem
export PGSSLROOTCERT=config/tls/int_server/certs/ca-chain-bundle.cert.pem

cargo test -- --include-ignored

docker-compose up pgav
