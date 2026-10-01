-- 0003_hromov_wording: tidy the sentence left by 0002 (the dossier names only "Hromov", on the Ukrainian side).
UPDATE claim SET text = string::replace(text, "Ukrainian Hromov said", "Hromov, on the Ukrainian side, said")
    WHERE topics CONTAINS topic:c08 AND string::contains(text, "Ukrainian Hromov said");
