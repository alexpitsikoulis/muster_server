ALTER TABLE IF EXISTS users
ADD COLUMN email_confirmed BOOLEAN NOT NULL DEFAULT false;