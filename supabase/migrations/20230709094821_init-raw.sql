CREATE TABLE raw (
    id serial PRIMARY KEY,
    created_by uuid NOT NULL REFERENCES public.profiles(id),
    name text NOT NULL,
    container int NOT NULL,
    proteins decimal NOT NULL,
    carbs decimal NOT NULL,
    fat decimal NOT NULL
);

ALTER TABLE raw ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Everyone can see the raw ingredients" 
ON "public"."raw"
FOR SELECT 
TO authenticated
USING ( true );

CREATE POLICY "Everyone can create new ingredients" 
ON "public"."raw"
FOR INSERT 
TO authenticated
WITH CHECK (true);

CREATE POLICY "User can update ingredients created by him"
ON "public"."raw"
FOR UPDATE
TO authenticated
USING ( auth.uid() = created_by )
WITH CHECK ( auth.uid() = created_by );

CREATE POLICY "User can delete ingredients created by him"
ON "public"."raw"
FOR DELETE
TO authenticated
USING ( auth.uid() = created_by );
