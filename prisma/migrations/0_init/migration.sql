-- CreateTable
CREATE TABLE "Profile" (
    "id" UUID NOT NULL,
    "updated_at" TIMESTAMPTZ(6),
    "full_name" TEXT,
    "is_admin" BOOLEAN NOT NULL DEFAULT false,

    CONSTRAINT "Profile_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Raw" (
    "id" SERIAL NOT NULL,
    "created_by" UUID,
    "name" TEXT NOT NULL,
    "container" INTEGER NOT NULL,
    "proteins" DECIMAL NOT NULL,
    "carbs" DECIMAL NOT NULL,
    "fat" DECIMAL NOT NULL,

    CONSTRAINT "Raw_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Recipie" (
    "id" BIGSERIAL NOT NULL,
    "created_at" TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "name" TEXT NOT NULL DEFAULT '',
    "portions" BIGINT NOT NULL DEFAULT 1,
    "steps" JSON NOT NULL DEFAULT '[]',

    CONSTRAINT "Recipie_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "RecipiesIngredient" (
    "id" BIGSERIAL NOT NULL,
    "recipe_id" BIGINT NOT NULL,
    "raw_id" INTEGER,
    "name" TEXT NOT NULL DEFAULT '',
    "amount" DECIMAL NOT NULL DEFAULT 0,
    "unit" TEXT,

    CONSTRAINT "RecipiesIngredient_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "Recipie_id_key" ON "Recipie"("id");

-- CreateIndex
CREATE UNIQUE INDEX "RecipiesIngredient_id_key" ON "RecipiesIngredient"("id");

-- AddForeignKey
ALTER TABLE "Raw" ADD CONSTRAINT "Raw_created_by_fkey" FOREIGN KEY ("created_by") REFERENCES "Profile"("id") ON DELETE NO ACTION ON UPDATE NO ACTION;

-- AddForeignKey
ALTER TABLE "RecipiesIngredient" ADD CONSTRAINT "RecipiesIngredient_raw_id_fkey" FOREIGN KEY ("raw_id") REFERENCES "Raw"("id") ON DELETE SET NULL ON UPDATE NO ACTION;

-- AddForeignKey
ALTER TABLE "RecipiesIngredient" ADD CONSTRAINT "RecipiesIngredient_recipe_id_fkey" FOREIGN KEY ("recipe_id") REFERENCES "Recipie"("id") ON DELETE CASCADE ON UPDATE NO ACTION;

