-- 0002_chapter_extraction_corrections: corrections to the chapter-dossier extraction (2026-10-01), reported by
-- the extraction and curation agents and verified against the dossiers by the coordinator. Idempotent.

LET $p = { by: 'fixes/0002', method: 'human', at: d'2026-10-01T00:00:00Z' };

-- c04: bare "Andriivka" in the Kherson chapter is the Kherson Oblast village, not the Kharkiv one
FOR $c IN [claim:cc04_0009_16, claim:cc04_0015_05, claim:cc04_0023_06] {
    DELETE about WHERE in = $c AND out = place:andriivka;
    IF count(SELECT id FROM about WHERE in = $c AND out = place:andriivka_kherson) = 0 {
        RELATE $c->about->place:andriivka_kherson SET role = 'location', prov = $p;
    };
};

-- c08: "3rd Army Corps" at Vuhledar is the Russian volunteer corps, not Ukraine's 3rd Army Corps
FOR $c IN [claim:cc08_0023_10, claim:cc08_0026_03] {
    DELETE about WHERE in = $c AND out = actor:ua_3rd_army_corps;
    IF count(SELECT id FROM about WHERE in = $c AND out = actor:ru_3rd_army_corps) = 0 {
        RELATE $c->about->actor:ru_3rd_army_corps SET role = 'subject', prov = $p;
    };
};

-- c08: the extractor added first names the dossier does not give (it has only "Bezsonov" and "Hromov")
UPDATE claim SET text = string::replace(text, "Brig. Gen. Oleksiy Hromov", "Hromov")
    WHERE topics CONTAINS topic:c08 AND string::contains(text, "Oleksiy Hromov");
UPDATE claim SET text = string::replace(text, "Daniil Bezsonov", "Bezsonov")
    WHERE topics CONTAINS topic:c08 AND string::contains(text, "Daniil Bezsonov");

-- c05: the dossier says only "Ukrainian USVs", so the claim must not name the Magura V5
DELETE about WHERE in = claim:cc05_0010_52 AND out = system:magura_v5;

-- c17: the 2009 gas war predates the 2022 war; it belongs to no era
UPDATE claim:cc17_0012_02 SET eras = [] WHERE eras != [];
