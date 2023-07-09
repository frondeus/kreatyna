create table profiles (
  id uuid references auth.users on delete cascade not null primary key,
  updated_at timestamp with time zone,
  full_name text,
);

alter table profiles
  enable row level security;


CREATE POLICY "User can modify his profile as he wants" ON "public"."profiles"
AS PERMISSIVE FOR ALL
TO public
USING (auth.uid() = id)
WITH CHECK (auth.uid() = id)

create function public.handle_new_user()
returns trigger as $$
begin
  insert into public.profiles (id, full_name)
  values (new.id, '')
  return new;
end;
$$ language plpgsql security definer;
create trigger on_auth_user_created
  after insert on auth.users
  for each row execute procedure public.handle_new_user();

