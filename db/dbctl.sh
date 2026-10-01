#!/usr/bin/env bash
# Control the local SurrealDB container for the bavovniatko knowledge base.
# Usage: db/dbctl.sh start|stop|status|logs|sql
set -euo pipefail
cd "$(dirname "$0")"
IMAGE=surrealdb/surrealdb:v3.3.0
NAME=bavovniatko-surrealdb
[ -f .env ] || { echo "missing db/.env (see db/README.md)" >&2; exit 1; }
set -a; . ./.env; set +a

case "${1:-status}" in
  start)
    mkdir -p data
    if docker ps -a --format '{{.Names}}' | grep -qx "$NAME"; then
      docker start "$NAME" >/dev/null
    else
      docker run -d --name "$NAME" --user "$(id -u):$(id -g)" \
        -p "127.0.0.1:${SURREAL_PORT}:8000" -v "$PWD/data:/data" \
        --restart unless-stopped "$IMAGE" \
        start --no-banner --log info --user "$SURREAL_USER" --pass "$SURREAL_PASS" \
        "surrealkv:///data/kb?versioned=true" >/dev/null
    fi
    for _ in $(seq 30); do curl -sf "http://127.0.0.1:${SURREAL_PORT}/health" >/dev/null && { echo "up on :${SURREAL_PORT}"; exit 0; }; sleep 0.5; done
    echo "did not become healthy" >&2; docker logs --tail 20 "$NAME" >&2; exit 1 ;;
  stop) docker stop "$NAME" >/dev/null && echo stopped ;;
  status) docker ps --filter "name=$NAME" --format '{{.Names}} {{.Status}} {{.Image}}' ;;
  logs) docker logs --tail 50 "$NAME" ;;
  sql) docker exec -it "$NAME" /surreal sql --endpoint http://127.0.0.1:8000 \
         --user "$SURREAL_USER" --pass "$SURREAL_PASS" --ns "$SURREAL_NS" --db "$SURREAL_DB" --pretty ;;
  *) echo "usage: $0 start|stop|status|logs|sql" >&2; exit 2 ;;
esac
