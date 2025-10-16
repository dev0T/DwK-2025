-- Add migration script here
BEGIN;
UPDATE todos
SET done = false
WHERE done IS NULL;
ALTER TABLE todos
ALTER COLUMN done
SET NOT NULL;
COMMIT;