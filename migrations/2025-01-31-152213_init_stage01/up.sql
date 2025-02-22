-- Your SQL goes here
CREATE TABLE "users"(
	"id" BIGSERIAL NOT NULL PRIMARY KEY,
	"username" VARCHAR(255) NOT NULL UNIQUE,
	"hash" VARCHAR(255) NOT NULL,
	"totp" VARCHAR(255),
	"bot_owner_user_id" BIGINT REFERENCES users,
	"display_name" VARCHAR(255),
	"bio" TEXT,
	"created_at" TIMESTAMPTZ NOT NULL,
	"updated_at" TIMESTAMPTZ NOT NULL,
	"deleted_at" TIMESTAMPTZ
);

CREATE TABLE "notes"(
	"id" BIGSERIAL NOT NULL PRIMARY KEY,
	"user_id" BIGINT NOT NULL REFERENCES users,
	"content" TEXT,
	"reply_of_note_id" BIGINT REFERENCES notes,
	"renote_of_note_id" BIGINT REFERENCES notes,
	"edit_of_note_id" BIGINT REFERENCES notes,
	"created_at" TIMESTAMPTZ NOT NULL,
	"deleted_at" TIMESTAMPTZ
);

CREATE TABLE "tokens"(
	"id" BIGSERIAL NOT NULL PRIMARY KEY,
	"user_id" BIGINT NOT NULL REFERENCES users,
	"created_at" TIMESTAMPTZ NOT NULL,
	"deleted_at" TIMESTAMPTZ
);

CREATE TABLE "audits"(
	"id" BIGSERIAL NOT NULL PRIMARY KEY,
	"token_id" BIGINT NOT NULL REFERENCES tokens,
	"event" VARCHAR(255) NOT NULL,
	"detail" JSON,
	"created_at" TIMESTAMPTZ NOT NULL
);

CREATE TABLE "configs"(
	"id" BIGSERIAL NOT NULL PRIMARY KEY,
	"key" VARCHAR(255) NOT NULL UNIQUE,
	"value" TEXT NOT NULL
);
