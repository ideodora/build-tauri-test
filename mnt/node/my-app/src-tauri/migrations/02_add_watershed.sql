-- add watersheds

PRAGMA foreign_keys = ON;
DROP TABLE IF EXISTS `watersheds`;
CREATE TABLE `watersheds` (
  `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  `name` varchar(255) NOT NULL,
  `bounds` json
);
DROP TABLE IF EXISTS `watershed_items`;
CREATE TABLE `watershed_items` (
  `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  `watershed_id` INTEGER NOT NULL,
  `key` varchar(255) NOT NULL,
  `data` json NOT NULL,
  FOREIGN KEY (`watershed_id`) REFERENCES `watersheds` (`id`) ON DELETE CASCADE ON UPDATE RESTRICT
);