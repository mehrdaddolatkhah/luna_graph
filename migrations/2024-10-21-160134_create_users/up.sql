-- Your SQL goes here
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50),
    birth_date DATE,
    email VARCHAR(50) UNIQUE,
    mobile VARCHAR(20) UNIQUE,
    password VARCHAR(255) NOT NULL,
    email_verified BOOLEAN DEFAULT FALSE,
    token TEXT,
    provider_id VARCHAR(100),
    provider_access_token TEXT,
    provider_refresh_token TEXT,
    provider_user_id VARCHAR(100),
    public_key_credential TEXT,
    credential_type VARCHAR(50),
    role_id UUID,
    status VARCHAR(50),
    last_login TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
