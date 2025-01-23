-- Your SQL goes here
CREATE TABLE "forecast"(
	"id" INT4 NOT NULL,
	"timestamp" TIMESTAMP NOT NULL,
	"available" INT4 NOT NULL,
	PRIMARY KEY("id", "timestamp")
);

CREATE TABLE "station"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"name" VARCHAR NOT NULL,
	"latitude" FLOAT8 NOT NULL,
	"longitude" FLOAT8 NOT NULL,
	"adress" VARCHAR NOT NULL,
	"area" VARCHAR NOT NULL,
	"capacity" INT4 NOT NULL
);

