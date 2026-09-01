-- Add migration script here
CREATE TYPE account_status AS ENUM ('active', 'inactive');

ALTER TABLE account DROP COLUMN status;
ALTER TABLE account ADD COLUMN status account_status NOT NULL DEFAULT 'active';