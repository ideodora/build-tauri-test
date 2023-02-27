-- SQLite
PRAGMA foreign_keys = ON;
CREATE TABLE `prefectures_watersheds` (
  `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  `prefecture_id` INTEGER NOT NULL,
  `watershed_id` INTEGER NOT NULL,
  FOREIGN KEY (`prefecture_id`) REFERENCES `prefectures` (`id`),
  FOREIGN KEY (`watershed_id`) REFERENCES `watersheds` (`id`) ON DELETE CASCADE ON UPDATE RESTRICT
);