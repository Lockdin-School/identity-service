-- Add migration script here
CREATE TABLE account
(
    id             VARCHAR PRIMARY KEY,
    cognito_sub    VARCHAR UNIQUE NOT NULL,
    email          VARCHAR UNIQUE NOT NULL,
    email_verified BOOLEAN        NOT NULL DEFAULT FALSE,
    status         VARCHAR        NOT NULL DEFAULT 'active',
    created_at     TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    updated_at     TIMESTAMPTZ    NOT NULL DEFAULT NOW()
);

-- Auto update updated_at on every row change
CREATE OR REPLACE FUNCTION update_updated_at()
    RETURNS TRIGGER AS
$$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER users_updated_at
    BEFORE UPDATE
    ON account
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

-- Indexes
CREATE INDEX idx_users_cognito_sub ON account (cognito_sub);
CREATE INDEX idx_users_email ON account (email);
CREATE INDEX idx_users_status ON account (status);

CREATE TABLE student_profiles
(
    id                   VARCHAR PRIMARY KEY,
    account_id              VARCHAR UNIQUE NOT NULL REFERENCES account (id) ON DELETE CASCADE,

    -- Identity
    first_name           VARCHAR        NOT NULL,
    last_name            VARCHAR        NOT NULL,
    avatar_url           VARCHAR,

    -- Academic
    grade                INT            NOT NULL,
    curriculum VARCHAR NOT NULL DEFAULT 'CAPS' CHECK (curriculum IN ('CAPS', 'IEB')),
    school_name          VARCHAR,
    province             VARCHAR,

    -- Onboarding
    onboarding_completed BOOLEAN        NOT NULL DEFAULT FALSE,

    -- Timestamps
    created_at           TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ    NOT NULL DEFAULT NOW()
);

CREATE TRIGGER student_profiles_updated_at
    BEFORE UPDATE
    ON student_profiles
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

CREATE INDEX idx_student_profiles_user_id ON student_profiles (account_id);
CREATE INDEX idx_student_profiles_grade ON student_profiles (grade);