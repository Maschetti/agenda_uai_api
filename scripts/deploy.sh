#!/usr/bin/env bash
set -euo pipefail

# ========================
# Config padrão (override por env ou flags)
# ========================
COMPOSE_CMD="${COMPOSE_CMD:-docker compose}"
COMPOSE_FILE="${COMPOSE_FILE:-docker-compose.yml}"
SERVICE_DB="${SERVICE_DB:-postgres}"
RUN_TESTS="${RUN_TESTS:-true}"
ROLLBACK_ON_FAIL="${ROLLBACK_ON_FAIL:-false}"
WAIT_HEALTH_TIMEOUT="${WAIT_HEALTH_TIMEOUT:-120}"   # segundos

# ========================
# CLI simples
# ========================
usage() {
  cat <<EOF
Usage: $(basename "$0") [options]

Options:
  -f, --file <compose.yml>   Compose file (default: $COMPOSE_FILE)
  -s, --service-db <name>    Nome do serviço de DB (default: $SERVICE_DB)
  --no-tests                 Não rodar scripts/db_check.sh após subir
  --rollback-on-fail         Faz rollback (down -v) se teste falhar
  -h, --help                 Ajuda

Ambiente:
  COMPOSE_CMD, COMPOSE_FILE, SERVICE_DB, RUN_TESTS, ROLLBACK_ON_FAIL, WAIT_HEALTH_TIMEOUT
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    -f|--file) COMPOSE_FILE="$2"; shift 2 ;;
    -s|--service-db) SERVICE_DB="$2"; shift 2 ;;
    --no-tests) RUN_TESTS="false"; shift ;;
    --rollback-on-fail) ROLLBACK_ON_FAIL="true"; shift ;;
    -h|--help) usage; exit 0 ;;
    *) echo "Flag desconhecida: $1"; usage; exit 1 ;;
  esac
done

# ========================
# Utilitários
# ========================
log() { printf "\033[1;36m[deploy]\033[0m %s\n" "$*"; }
ok()  { printf "\033[1;32m[  ok  ]\033[0m %s\n" "$*"; }
err() { printf "\033[1;31m[error ]\033[0m %s\n" "$*" >&2; }

compose() {
  $COMPOSE_CMD -f "$COMPOSE_FILE" "$@"
}

collect_logs() {
  log "coletando logs recentes…"
  compose logs --since=10m || true
}

rollback() {
  if [[ "$ROLLBACK_ON_FAIL" == "true" ]]; then
    err "rollback habilitado — derrubando stack e volumes…"
    compose down -v || true
  fi
}

wait_for_health() {
  local service="$1" deadline=$(( $(date +%s) + WAIT_HEALTH_TIMEOUT ))
  log "aguardando saúde do serviço '$service' (timeout ${WAIT_HEALTH_TIMEOUT}s)…"
  while true; do
    local status
    status="$(compose ps --format json 2>/dev/null | jq -r ".[] | select(.Service==\"$service\") | .State" || echo "")"
    # Fallback: se jq indisponível ou format não suportado, tenta pg_isready
    if [[ -z "$status" || "$status" == "null" ]]; then
      if compose exec -T "$service" pg_isready -U postgres -d postgres >/dev/null 2>&1; then
        ok "pg_isready respondeu para '$service'."
        return 0
      fi
    else
      case "$status" in
        "running") 
          # Se tiver healthcheck, verifique explicitamente
          if compose ps --format json | jq -e ".[] | select(.Service==\"$service\") | .Health==\"healthy\"" >/dev/null 2>&1; then
            ok "serviço '$service' está healthy."
            return 0
          fi
          ;;
      esac
    fi
    if (( $(date +%s) > deadline )); then
      err "timeout aguardando saúde do serviço '$service'."
      return 1
    fi
    sleep 2
  done
}

# ========================
# Deploy
# ========================
log "usando compose file: $COMPOSE_FILE"

# Puxa imagens (se houver) e/ou constrói
log "pull/build e criação/atualização dos serviços…"
compose pull || true
compose build || true

# Sobe/atualiza em modo detach
log "subindo serviços em segundo plano…"
compose up -d

# Espera Postgres ficar saudável
if ! wait_for_health "$SERVICE_DB"; then
  collect_logs
  rollback
  exit 1
fi

ok "serviços no ar."

# ========================
# Testes pós-deploy
# ========================
if [[ "$RUN_TESTS" == "true" ]]; then
  if [[ ! -x "scripts/db_check.sh" ]]; then
    err "scripts/db_check.sh não encontrado ou não executável. Pule com --no-tests ou torne-o executável (chmod +x)."
    collect_logs
    rollback
    exit 1
  fi
  log "executando testes de saúde do banco…"
  if ! ./scripts/db_check.sh; then
    err "testes falharam."
    collect_logs
    rollback
    exit 1
  fi
  ok "testes passaram! 🎉"
else
  log "testes desabilitados (--no-tests)."
fi

ok "deploy concluído com sucesso."
