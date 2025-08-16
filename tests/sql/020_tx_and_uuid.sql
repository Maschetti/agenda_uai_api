-- 020_tx_and_uuid.sql
BEGIN;

CREATE TEMP TABLE tmp_tx_test(
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  val INT NOT NULL
);

INSERT INTO tmp_tx_test (val) VALUES (1), (2), (3);

-- Confere se gerou 3 linhas e UUIDs válidos
DO $$
DECLARE c INT;
BEGIN
  SELECT COUNT(*) INTO c FROM tmp_tx_test;
  IF c <> 3 THEN
    RAISE EXCEPTION 'Transação: contagem esperada 3, obtido %', c;
  END IF;
END$$;

-- Rollback: nada deve persistir
ROLLBACK;
