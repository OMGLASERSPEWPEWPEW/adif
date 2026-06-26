-- 035_restore_composite_pks.sql
-- Restore composite primary keys on character_bind, character_skills, and
-- character_languages. Migration 033 dropped these PKs (to work around
-- ON CONFLICT (id) in the old C++ code) but the C++ has since been fixed
-- to use the correct composite columns.
--
-- Without these constraints, ON CONFLICT (id, slot) etc. fails with:
--   "there is no unique or exclusion constraint matching the ON CONFLICT specification"

BEGIN;

-- character_bind: PK (id, slot)
DROP INDEX IF EXISTS idx_character_bind_id_slot;
ALTER TABLE character_bind ADD PRIMARY KEY (id, slot);

-- character_skills: PK (id, skill_id)
DROP INDEX IF EXISTS idx_character_skills_id_skill;
ALTER TABLE character_skills ADD PRIMARY KEY (id, skill_id);

-- character_languages: PK (id, lang_id)
DROP INDEX IF EXISTS idx_character_languages_id_lang;
ALTER TABLE character_languages ADD PRIMARY KEY (id, lang_id);

COMMIT;
