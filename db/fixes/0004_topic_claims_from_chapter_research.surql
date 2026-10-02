-- 0004_topic_claims_from_chapter_research: corrections to topic-file claims that the chapter research
-- (2026-10-01/02) showed to be wrong or mis-attributed. Each states its basis. Idempotent.

-- 05-0009-02: order of the Aerorozvidka targets. The Guardian original (cited in research/chapters/c01) has the
-- lead vehicles first, then the depot; Defense Express's rewrite, which the claim cites, reversed it.
UPDATE claim:c05_0009_02 SET
    text = "Aerorozvidka's first targets in the night ambushes north of Kyiv were the Russian column's lead vehicles and then its forward supply depot; the unit's own claims were only partly verified.",
    confidence_note = "Order corrected per the Guardian original (research/chapters/c01-march-on-kyiv.md §11); the cited Defense Express rewrite puts the depot first."
    WHERE string::contains(text, "forward supply depot and its lead vehicles");

-- 00-0055-04: "deliberate regrouping" is the analyst Oleksandr Kovalenko's description, not an official statement
-- (research/chapters/c15-pokrovsk-myrnohrad.md §1).
UPDATE claim:c00_0055_04 SET
    text = "Over the winter of 2025–26 Ukraine lost most of Pokrovsk and Myrnohrad and withdrew in what the analyst Oleksandr Kovalenko called a \"deliberate regrouping\".",
    confidence_note = "Attribution corrected: an analyst's phrase, not an official statement (research/chapters/c15-pokrovsk-myrnohrad.md)."
    WHERE string::contains(text, "withdrew in a \"deliberate regrouping\"");

-- 00-0063-04: computed DeepState geometry shows no change in the Vivaldi box before 14 Sep 2026, so the gains
-- appear only in mid/late-September versions, not August (research/chapters/c19-vivaldi.md §1, §11).
UPDATE claim:c00_0063_04 SET status = "disputed",
    confidence_note = "Disputed by computed DeepState geometry: no change in the Vivaldi box from 1 Feb to 13 Sep 2026 (research/chapters/c19-vivaldi.md)."
    WHERE status != "disputed";
