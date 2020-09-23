CREATE TYPE "USERGROUP" AS ENUM (
  'user',
  'admin'
);

CREATE TABLE "users" (
  "id" UUID PRIMARY KEY NOT NULL,
  "email" text UNIQUE NOT NULL,
  "password" text NOT NULL,
  "user_group" "USERGROUP" NOT NULL DEFAULT 'user'
);

CREATE TABLE "tasks" (
  "id" UUID PRIMARY KEY NOT NULL,
  "owner_id" UUID NOT NULL,
  "duration" int NOT NULL,
  "start" timestamp NOT NULL,
  "label" text
);

CREATE TABLE "tags" (
  "id" UUID PRIMARY KEY NOT NULL,
  "owner_id" UUID NOT NULL,
  "label" text NOT NULL,
  "is_generic" boolean NOT NULL DEFAULT false
);

CREATE TABLE "task_tag" (
  "id" SERIAL PRIMARY KEY NOT NULL,
  "task_id" UUID,
  "tag_id" UUID
);

ALTER TABLE "tasks" ADD FOREIGN KEY ("owner_id") REFERENCES "users" ("id");

ALTER TABLE "tags" ADD FOREIGN KEY ("owner_id") REFERENCES "users" ("id");

ALTER TABLE "task_tag" ADD FOREIGN KEY ("task_id") REFERENCES "tasks" ("id");

ALTER TABLE "task_tag" ADD FOREIGN KEY ("tag_id") REFERENCES "tags" ("id");

COMMENT ON TABLE "tasks" IS 'Tasks are used for tracking time on activities.';

COMMENT ON COLUMN "tasks"."duration" IS 'This is the duration of the task in seconds.';

COMMENT ON TABLE "tags" IS 'Tags are used for categorising tasks.';

COMMENT ON COLUMN "tags"."is_generic" IS 'Generic tags can be used by anyone. These will only be created by project team.';

COMMENT ON TABLE "task_tag" IS 'This table tracks which tasks and tags are associated.';