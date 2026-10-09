CREATE TABLE "registrations"
(
    "created_at"                   TIMESTAMPTZ(6) NOT NULL,
    "verification_code_expires_at" TIMESTAMPTZ(6) NOT NULL,
    "verification_attempts"        SMALLINT NOT NULL,
    "verification_code_hash"       BYTEA    NOT NULL,
    "password_hash"                TEXT     NOT NULL,
    "email"                        TEXT     NOT NULL,
    PRIMARY KEY ("email")
);

CREATE TABLE "users"
(
    "created_at" TIMESTAMPTZ(6) NOT NULL,
    "updated_at" TIMESTAMPTZ(6) NOT NULL,
    "id"         UUID     NOT NULL,
    "status"     SMALLINT NOT NULL,
    "email"      TEXT     NOT NULL,
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "index_users_by_email" ON "users" ("email");

CREATE TABLE "user_passwords"
(
    "created_at"    TIMESTAMPTZ(6) NOT NULL,
    "updated_at"    TIMESTAMPTZ(6) NOT NULL,
    "user_id"       UUID NOT NULL,
    "password_hash" TEXT NOT NULL,
    PRIMARY KEY ("user_id")
);

CREATE TABLE "user_devices"
(
    "created_at"   TIMESTAMPTZ(6) NOT NULL,
    "last_used_at" TIMESTAMPTZ(6) NOT NULL,
    "id"           UUID  NOT NULL,
    "user_id"      UUID  NOT NULL,
    "device_name"  TEXT  NOT NULL,
    "device_hash"  BYTEA NOT NULL,
    "revoked_at"   TIMESTAMPTZ(6),
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "index_user_devices_by_user_id_and_device_hash" ON "user_devices" ("user_id", "device_hash");

CREATE TABLE "user_passkeys"
(
    "created_at"        TIMESTAMPTZ(6) NOT NULL,
    "last_used_at"      TIMESTAMPTZ(6) NOT NULL,
    "id"                UUID     NOT NULL,
    "user_id"           UUID     NOT NULL,
    "signature_counter" SMALLINT NOT NULL,
    "credential_id"     BYTEA    NOT NULL,
    "public_key"        BYTEA    NOT NULL,
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "index_user_passkeys_by_credential_id" ON "user_passkeys" ("credential_id");

CREATE TABLE "webauthn_challenges"
(
    "created_at"      TIMESTAMPTZ(6) NOT NULL,
    "expires_at"      TIMESTAMPTZ(6) NOT NULL,
    "user_id"         UUID,
    "device_id"       UUID,
    "challenge_token" BYTEA NOT NULL,
    "challenge_type"  TEXT  NOT NULL,
    PRIMARY KEY ("challenge_token")
);
CREATE INDEX "index_webauthn_challenges_by_expires_at" ON "webauthn_challenges" ("expires_at");

CREATE TABLE "user_identities"
(
    "created_at"      TIMESTAMPTZ(6) NOT NULL,
    "id"              UUID NOT NULL,
    "user_id"         UUID NOT NULL,
    "provider"        TEXT NOT NULL,
    "provider_sub_id" TEXT NOT NULL,
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "index_user_identities_by_provider_and_provider_sub_id" ON "user_identities" ("provider", "provider_sub_id");
CREATE INDEX "index_user_identities_by_user_id" ON "user_identities" ("user_id");

CREATE TABLE "oauth_challenges"
(
    "created_at"          TIMESTAMPTZ(6) NOT NULL,
    "expires_at"          TIMESTAMPTZ(6) NOT NULL,
    "state"               UUID NOT NULL,
    "nonce"               UUID,
    "pkce_code_verifier"  TEXT NOT NULL,
    "client_redirect_uri" TEXT NOT NULL,
    "provider"            TEXT NOT NULL,
    "flow_type"           TEXT NOT NULL,
    PRIMARY KEY ("state")
);

CREATE TABLE "refresh_tokens"
(
    "created_at"      TIMESTAMPTZ(6) NOT NULL,
    "expires_at"      TIMESTAMPTZ(6) NOT NULL,
    "id"              UUID    NOT NULL,
    "user_id"         UUID    NOT NULL,
    "device_id"       UUID    NOT NULL,
    "token_family_id" UUID    NOT NULL,
    "is_revoked"      BOOLEAN NOT NULL,
    "token_hash"      BYTEA   NOT NULL,
    PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "index_refresh_tokens_by_token_hash" ON "refresh_tokens" ("token_hash");
CREATE INDEX "index_refresh_tokens_by_token_family_id" ON "refresh_tokens" ("token_family_id");
