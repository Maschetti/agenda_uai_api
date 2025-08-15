#!/usr/bin/env bash
set -e

# Habilita extensões na base de dados
psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$APP_DB" <<-EOSQL
    CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
    CREATE EXTENSION IF NOT EXISTS "pgcrypto";
EOSQL

# uuid-ossp for unique identifier (UUID)
# pgcrypto for cryptography