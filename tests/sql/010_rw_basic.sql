-- 010_rw_basic.sql
SET client_min_messages TO WARNING;

-- Tabela temporária para não “sujar” o schema
CREATE TEMP TABLE IF NOT EXISTS tmp_healthcheck (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  name TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

INSERT INTO tmp_healthcheck (name) VALUES ('ping'), ('pong');

-- Deve retornar 2
DO $$
DECLARE c INT;
BEGIN
  SELECT COUNT(*) INTO c FROM tmp_healthcheck;
  IF c <> 2 THEN
    RAISE EXCEPTION 'Contagem esperada 2, obtido %', c;
  END IF;
END$$;

-- Testa função do pgcrypto (apenas para validar disponibilidade)
SELECT digest('ok', 'sha256');

-- Cleanup automático pois é TEMP TABLE (fecha ao encerrar a sessão)
