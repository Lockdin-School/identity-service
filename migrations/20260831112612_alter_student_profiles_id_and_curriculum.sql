-- Add migration script here
CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TYPE curriculum_enum AS ENUM ('CAPS', 'IEB');

ALTER TABLE student_profiles
    ALTER COLUMN id DROP DEFAULT;

ALTER TABLE student_profiles
    ALTER COLUMN id TYPE UUID USING id::uuid;

ALTER TABLE student_profiles
    ALTER COLUMN id SET DEFAULT gen_random_uuid();

ALTER TABLE student_profiles
    ALTER COLUMN curriculum DROP DEFAULT;

ALTER TABLE student_profiles
    DROP CONSTRAINT IF EXISTS student_profiles_curriculum_check;

ALTER TABLE student_profiles
    ALTER COLUMN curriculum TYPE curriculum_enum USING curriculum::curriculum_enum;

ALTER TABLE student_profiles
    ALTER COLUMN curriculum SET DEFAULT 'CAPS';