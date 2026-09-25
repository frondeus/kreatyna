---
name: forward-only-migrations
description: Apply this repository's forward-only policy when adding or changing Diesel/SQLite migrations in vibetyna.
---

# Forward-only migrations

This repository advances database schemas through new `up.sql` migrations.
Leave every new `down.sql` empty, matching the existing counter migration.

Do not test rollback or describe a migration as reversible. To correct an
already-applied schema, add another forward migration; do not edit the old one.
Test upgrades from the prior schema, repeated startup with no pending migrations,
and preservation of existing data.
