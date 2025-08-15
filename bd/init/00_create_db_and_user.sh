#!/usr/bin/env bash
set -e

# Esse script roda como superusuário "postgres" dentro do container
psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" <<-EOSQL
    -- Cria banco e usuário de aplicação
    CREATE DATABASE "${APP_DB}";
    CREATE USER "${APP_DB_USER}" WITH ENCRYPTED PASSWORD '${APP_DB_PASSWORD}';

    -- Permissões básicas
    GRANT ALL PRIVILEGES ON DATABASE "${APP_DB}" TO "${APP_DB_USER}";
EOSQL
