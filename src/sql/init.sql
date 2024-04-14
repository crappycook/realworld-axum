DROP TABLE IF EXISTS `player`;

CREATE TABLE `player` (
    `id` bigint UNSIGNED NOT NULL AUTO_INCREMENT,
    `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    `name` varchar(128) NOT NULL DEFAULT '',
    `club` varchar(64) NOT NULL DEFAULT '',
    PRIMARY KEY (`id`),
    KEY `idx_name` (`name`)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4;