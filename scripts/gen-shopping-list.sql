-- psql postgres://postgres:postgres@localhost:5432/postgres -f ./scripts/gen-shopping-list.sql
delete from "ShoppingListItem";

with RI AS (
    select
        i.name,
        i.amount,
        i.unit,
        i.raw_id
    from
        "WeekPlanMeal" as w
        inner join "Recipe" as r on w.recipe_id = r.id
        inner join "RecipesIngredient" as i on r.id = i.recipe_id
    where
        w.week = 36
        -- SUNDAY = 1 MONDAY = 2 TUESDAY = 3 WEDNESDAY = 4 THURSDAY = 5 FRIDAY = 6 SATURDAY = 7
        and w.day > 1
        and w.day < 5
) 
insert into "ShoppingListItem" ("id", "name", "amount", "unit")
(
    select
        gen_random_uuid() as id,
        -- ri.raw_id,
        raw.name,
        sum(ri.amount),
        ri.unit
    from
        RI as ri
        inner join "Raw" as raw on ri.raw_id = raw.id
    where
        ri.raw_id is not null
    group by
        ri.raw_id,
        raw.name,
        ri.unit
    order by
        ri.raw_id
)
union
all (
    select
        gen_random_uuid() as id,
        -- null,
        ri.name,
        ri.amount,
        ri.unit
    from
        RI as ri
    where
        ri.raw_id is null
)