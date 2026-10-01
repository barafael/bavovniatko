-- 0009_updated_at_restorable: `updated_at` is set to now() on every write, unless the write supplies it
-- explicitly (dump restore), so a restored database is identical to the one that was dumped.
DEFINE FIELD OVERWRITE updated_at ON source TYPE datetime VALUE IF $input != NONE { $input } ELSE { time::now() };
DEFINE FIELD OVERWRITE updated_at ON claim  TYPE datetime VALUE IF $input != NONE { $input } ELSE { time::now() };
