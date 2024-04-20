DO $$
DECLARE
    i INTEGER := 1;
BEGIN
    WHILE i <= 5 LOOP
        INSERT INTO users (name) VALUES ('testuser' || i);
        i := i + 1;
    END LOOP;
END $$;
