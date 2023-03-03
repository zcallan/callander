CREATE TYPE met_at_accuracy_enum AS ENUM ('day', 'month', 'year');

ALTER TABLE friends ADD COLUMN met_at DATE;
ALTER TABLE friends ADD COLUMN met_at_accuracy met_at_accuracy_enum;
