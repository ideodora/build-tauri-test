-- CREATE DATABASE IF NOT EXISTS rivers;
-- USE rivers;

-- Adminer 4.7.7 MySQL dump

-- SET NAMES utf8;
-- SET time_zone = '+00:00';
-- SET foreign_key_checks = 0;
-- SET sql_mode = 'NO_AUTO_VALUE_ON_ZERO';
-- SET NAMES utf8mb4;

DROP TABLE IF EXISTS Curve;
CREATE TABLE `Curve` (
  `id` varchar(255) NOT NULL,
  `segments` mediumtext NOT NULL,
  `prefectureCode` int NOT NULL,
  PRIMARY KEY (`id`,`prefectureCode`)
);


DROP TABLE IF EXISTS `Point`;
CREATE TABLE `Point` (
  `id` varchar(255) NOT NULL,
  `lat` double NOT NULL,
  `lng` double NOT NULL,
  `prefectureCode` int NOT NULL,
  PRIMARY KEY (`id`,`prefectureCode`)
);


DROP TABLE IF EXISTS `RiverNode`;
CREATE TABLE `RiverNode` (
  `id` varchar(255) NOT NULL,
  `waterSystemCode` int NOT NULL,
  `position` varchar(255) NOT NULL,
  `elevation` int NOT NULL,
  `prefectureCode` int NOT NULL,
  PRIMARY KEY (`id`,`prefectureCode`)
);


DROP TABLE IF EXISTS `Stream`;
CREATE TABLE `Stream` (
  `id` varchar(255) NOT NULL,
  `waterSystemCode` int NOT NULL,
  `location` varchar(255) NOT NULL,
  `riverCode` bigint NOT NULL,
  `sectionType` int NOT NULL,
  `riverName` varchar(255) NOT NULL,
  `originalDataType` int NOT NULL,
  `flowDirection` int NOT NULL,
  `startRiverNode` varchar(255) NOT NULL,
  `endRiverNode` varchar(255) NOT NULL,
  `startStreamNode` varchar(255) NOT NULL,
  `endStreamNode` varchar(255) NOT NULL,
  `prefectureName` varchar(255) NOT NULL,
  `prefectureCode` int NOT NULL,
  PRIMARY KEY (`id`,`prefectureCode`)
);


DROP TABLE IF EXISTS `prefectures`;
CREATE TABLE `prefectures` (
  `id` int NOT NULL,
  `short_name` varchar(8) NOT NULL,
  `long_name` varchar(8) NOT NULL,
  `furi_kana` varchar(8) NOT NULL,
  PRIMARY KEY (`id`)
);


DROP TABLE IF EXISTS `rivers`;
CREATE TABLE `rivers` (
  `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  `name` varchar(255) NOT NULL,
  `segments` json NOT NULL
);


DROP TABLE IF EXISTS `rivers_prefectures`;

PRAGMA foreign_keys = ON;

CREATE TABLE `rivers_prefectures` (
  `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  `river_id` INTEGER NOT NULL,
  `prefecture_id` INTEGER NOT NULL,
  FOREIGN KEY (`prefecture_id`) REFERENCES `prefectures` (`id`),
  FOREIGN KEY (`river_id`) REFERENCES `rivers` (`id`) ON DELETE CASCADE ON UPDATE RESTRICT
);
CREATE INDEX `prefecture_id` ON `rivers_prefectures`(`prefecture_id`);
CREATE INDEX `river_id` ON `rivers_prefectures`(`river_id`);

-- 2020-11-08 06:31:57