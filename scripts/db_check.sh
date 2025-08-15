#!/usr/bin/env bash
set -euo pipefail

# Config
COMPOSE=${COMPOSE:-docker compose}
SERVICE=${SERVICE:-postgres}   # nome do serviço no compose
SQL_DIR=${SQL_DIR:-tests/sql}

# Carrega .env da raiz (se existir)
if [[ -f ".env" ]]; then
  # shellcheck disable=SC2046
  export $(grep -v '^\s*#' .env | xargs -0 -I {} bash -c 'echo {}' 2>/dev/null || true)
fi

APP_DB=${APP_DB:-app_db}
APP_DB_USER=${APP_DB_USER:-app_user}
APP_DB_PASSWORD=${APP_DB_PASSWORD:-app_password}
POSTGRES_PORT=${POSTGRES_PORT:-5432}

log() { printf "\033[1;36m[db-check]\033[0m %s\n" "$*"; }
ok()  { printf "\033[1;32m[ ok    ]\033[0m %s\n" "$*"; }
err() { printf "\033[1;31m[ error ]\033[0m %s\n" "$*" >&2; }

wait_for_db() {
  log "esperando Postgres ficar healthy…"
  # tenta via pg_isready dentro do container
  for i in {1..30}; do
    if $COMPOSE exec -T "$SERVICE" pg_isready -U postgres -d postgres >/dev/null 2>&1; then
      ok "Postgres respondeu ao pg_isready."
      return 0
    fi
    sleep 2
  done
  err "timeout esperando o Postgres."
  return 1
}

run_sql_file() {
  local file="$1"
  # Usa o usuário de aplicação e password para validar permissões reais
  log "rodando $(basename "$file")"
  $COMPOSE exec -T "$SERVICE" \
    env PGPASSWORD="$APP_DB_PASSWORD" \
    psql \
      -h 127.0.0.1 \
      -U "$APP_DB_USER" \
      -d "$APP_DB" \
      -v ON_ERROR_STOP=1 \
      -f "/dev/stdin" < "$file" >/dev/null
  ok "$(basename "$file") passou."
}

main() {
  # sanity: o serviço existe?
  if ! $COMPOSE ps "$SERVICE" >/dev/null 2>&1; then
    err "serviço '$SERVICE' não encontrado. Ajuste SERVICE= no env ou verifique o docker-compose.yml."
    exit 1
  fi

  wait_for_db

  # Garante que o diretório existe
  if [[ ! -d "$SQL_DIR" ]]; then
    err "diretório de testes não encontrado: $SQL_DIR"
    exit 1
  fi

  # Roda todos os .sql em ordem
  shopt -s nullglob
  mapfile -t files < <(ls "$SQL_DIR"/*.sql | sort)
  if [[ ${#files[@]} -eq 0 ]]; then
    err "nenhum arquivo SQL encontrado em $SQL_DIR"
    exit 1
  fi

  for f in "${files[@]}"; do
    run_sql_file "$f"
  done

  ok "todos os testes passaram! 🎉"
}

main "$@"
