-- Add up migration script here
CREATE TABLE `users`(
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `username` VARCHAR(255) NOT NULL,
    `hash` VARCHAR(255) NOT NULL,
    `totp` VARCHAR(255) NULL,
    `bot_owner_user_id` BIGINT NULL,
    `display_name` VARCHAR(255) NULL,
    `bio` TEXT NULL,
    `created_at` BIGINT NOT NULL,
    `updated_at` BIGINT NOT NULL,
    `deleted_at` BIGINT NULL,
    CONSTRAINT `uniq_users_username` UNIQUE(`username`)
) DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci;
CREATE TABLE `notes`(
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `user_id` BIGINT NOT NULL,
    `content` TEXT NULL,
    `reply_of_note_id` BIGINT NULL,
    `renote_of_note_id` BIGINT NULL,
    `edit_of_note_id` BIGINT NULL,
    `created_at` BIGINT NOT NULL,
    `deleted_at` BIGINT NULL
) DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci;
CREATE TABLE `tokens`(
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `user_id` BIGINT NOT NULL,
    `created_at` BIGINT NOT NULL,
    `deleted_at` BIGINT NULL
) DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci;
CREATE TABLE `audits`(
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `token_id` BIGINT NOT NULL,
    `event` VARCHAR(255) NOT NULL,
    `detail` JSON NULL,
    `created_at` BIGINT NOT NULL
) DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci;
CREATE TABLE `configs`(
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `key` VARCHAR(255) NOT NULL,
    `value` TEXT NOT NULL
) DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_0900_ai_ci;