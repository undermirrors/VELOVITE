-- Your SQL goes here
CREATE TABLE "stations"(
	"id" INTEGER NOT NULL PRIMARY KEY,
	"name" VARCHAR NOT NULL,
	"latitude" FLOAT NOT NULL,
	"longitude" FLOAT NOT NULL,
	"adress" VARCHAR NOT NULL,
	"area" VARCHAR NOT NULL,
	"capacity" INTEGER NOT NULL
);

CREATE TABLE "forecasts"(
	"id" INTEGER NOT NULL,
	"timestamp" TIMESTAMP NOT NULL,
	"available" INTEGER NOT NULL,
	PRIMARY KEY("id", "timestamp")
);

