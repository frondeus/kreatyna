-- psql postgres://postgres:postgres@localhost:5432/postgres -f ./scripts/shopping-list.sql
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
        w.day > 1
        and w.day < 6
) (
    select
        ri.raw_id,
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
        null,
        ri.name,
        ri.amount,
        ri.unit
    from
        RI as ri
    where
        ri.raw_id is null
)