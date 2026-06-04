import { integer, pgTable } from "drizzle-orm/pg-core";

const example = pgTable("example", {
	id: integer().primaryKey().generatedByDefaultAsIdentity({ name: "example_id_seq", startWith: 1, increment: 1, minValue: 1, maxValue: 2147483647, cache: 1 }),
});
