-- Add migration script here
ALTER TABLE student_profiles
    DROP CONSTRAINT student_profiles_account_id_fkey;

ALTER TABLE account
    ALTER COLUMN id DROP DEFAULT;

ALTER TABLE account
    ALTER COLUMN id TYPE UUID USING id::uuid;

ALTER TABLE account
    ALTER COLUMN id SET DEFAULT gen_random_uuid();

ALTER TABLE student_profiles
    ALTER COLUMN account_id TYPE UUID USING account_id::uuid;

ALTER TABLE student_profiles
    ADD CONSTRAINT student_profiles_account_id_fkey
        FOREIGN KEY (account_id)
            REFERENCES account(id)
            ON DELETE CASCADE;