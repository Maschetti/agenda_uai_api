-- 001_extensions.sql
-- Falha se uuid-ossp ou pgcrypto não estiverem instaladas
DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'uuid-ossp') THEN
    RAISE EXCEPTION 'Extensão % ausente', 'uuid-ossp';
  END IF;
  IF NOT EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'pgcrypto') THEN
    RAISE EXCEPTION 'Extensão % ausente', 'pgcrypto';
  END IF;
END$$;
