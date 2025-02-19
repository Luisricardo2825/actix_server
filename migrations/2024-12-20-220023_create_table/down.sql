-- This file should undo anything in `up.sql`
DO $$
DECLARE
    table_name TEXT;
BEGIN
    FOR table_name IN
        SELECT name FROM tables
    LOOP
        EXECUTE format('DROP TABLE IF EXISTS %I CASCADE', table_name);
    END LOOP;
END $$;

DROP TABLE IF EXISTS "tables" CASCADE;