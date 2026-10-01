-- 0001_extraction_corrections: corrections to the initial claim extraction (reported by the extraction agents,
-- verified by the coordinator on 2026-09-30). Idempotent; applied with `migrate.py --fixes` after loading.
-- Staging files are historical input: do not reload them over these corrections.

LET $p = { by: 'fixes/0001', method: 'human', at: d'2026-09-30T00:00:00Z' };

-- 00: '(under 1% of Ukraine)' is a share of Ukraine gained in 2025, not the occupied share
UPDATE observation:c00_0058_03_02 SET metric = metric:territory_gained_share, qualifier = 'at_most', prov = $p;

-- 01: 'Geran-type drones' means the family, not Geran-2 specifically
DELETE claim:c01_0024_26->about WHERE out = system:geran_2;
IF count(SELECT id FROM about WHERE in = claim:c01_0024_26 AND out = system:shahed_geran_family) = 0 {
    RELATE claim:c01_0024_26->about->system:shahed_geran_family SET role = 'subject', prov = $p;
};

-- 03: Ka-52 altitude was filed under count_generic without numbers
UPDATE observation:c03_0016_07_00 SET metric = metric:flight_altitude, low = 50, high = 100, qualifier = 'range', prov = $p;

-- 03: claim texts with ISO months ('in 2024-02') -> 'in February 2024'
UPDATE claim:c03_0029_007 SET text = 'The Ukraine:Russia artillery shell ratio was 1:10 in February 2024.' WHERE text = 'The Ukraine:Russia artillery shell ratio was 1:10 in 2024-02.';
UPDATE claim:c03_0029_008 SET text = 'The Ukraine:Russia artillery shell ratio was 1:3 in August 2024.' WHERE text = 'The Ukraine:Russia artillery shell ratio was 1:3 in 2024-08.';
UPDATE claim:c03_0029_009 SET text = 'The Ukraine:Russia artillery shell ratio was 1:1.5 in December 2024.' WHERE text = 'The Ukraine:Russia artillery shell ratio was 1:1.5 in 2024-12.';
UPDATE claim:c03_0029_010 SET text = 'The Ukraine:Russia artillery shell ratio was 1:1.8 in July 2025.' WHERE text = 'The Ukraine:Russia artillery shell ratio was 1:1.8 in 2025-07.';
UPDATE claim:c03_0029_016 SET text = 'The Ukrainian General Staff counted about 113,000 Russian artillery strike events in January 2026 (strike events, not rounds).' WHERE text = 'The Ukrainian General Staff counted about 113,000 Russian artillery strike events in 2026-01 (strike events, not rounds).';
UPDATE claim:c03_0029_017 SET text = 'The Ukrainian General Staff counted more than 115,000 Russian artillery strike events in March 2026, including 2,834 by MLRS (strike events, not rounds).' WHERE text = 'The Ukrainian General Staff counted more than 115,000 Russian artillery strike events in 2026-03, including 2,834 by MLRS (strike events, not rounds).';
UPDATE claim:c03_0029_018 SET text = 'The Ukrainian General Staff counted about 97,000 Russian artillery strike events in July 2026 (strike events, not rounds).' WHERE text = 'The Ukrainian General Staff counted about 97,000 Russian artillery strike events in 2026-07 (strike events, not rounds).';
UPDATE claim:c03_0029_042 SET text = 'Ukrainian Air Force/MoD figures put Russian KAB launches at 3,200 in May 2024.' WHERE text = 'Ukrainian Air Force/MoD figures put Russian KAB launches at 3,200 in 2024-05.';
UPDATE claim:c03_0029_044 SET text = 'Ukrainian Air Force/MoD figures put Russian KAB launches at 5,328 in October 2025.' WHERE text = 'Ukrainian Air Force/MoD figures put Russian KAB launches at 5,328 in 2025-10.';
UPDATE claim:c03_0029_045 SET text = 'Ukrainian Air Force/MoD figures put Russian KAB launches at about 3,500 in November 2025.' WHERE text = 'Ukrainian Air Force/MoD figures put Russian KAB launches at about 3,500 in 2025-11.';
UPDATE claim:c03_0029_046 SET text = 'Ukrainian Air Force/MoD figures put Russian KAB launches at 5,171–5,700 in January 2026.' WHERE text = 'Ukrainian Air Force/MoD figures put Russian KAB launches at 5,171–5,700 in 2026-01.';
UPDATE claim:c03_0029_047 SET text = 'Ukrainian Air Force/MoD figures put Russian KAB launches at about 6,400 in February 2026 (derived figure).' WHERE text = 'Ukrainian Air Force/MoD figures put Russian KAB launches at about 6,400 in 2026-02 (derived figure).';
UPDATE claim:c03_0029_048 SET text = 'Ukrainian Air Force/MoD figures put Russian KAB launches at 7,987 in March 2026.' WHERE text = 'Ukrainian Air Force/MoD figures put Russian KAB launches at 7,987 in 2026-03.';
UPDATE claim:c03_0029_049 SET text = 'Ukrainian Air Force/MoD figures put Russian KAB launches at about 7,000 in April 2026.' WHERE text = 'Ukrainian Air Force/MoD figures put Russian KAB launches at about 7,000 in 2026-04.';
UPDATE claim:c03_0029_050 SET text = 'Ukrainian Air Force/MoD figures put Russian KAB launches at about 7,500 in May 2026.' WHERE text = 'Ukrainian Air Force/MoD figures put Russian KAB launches at about 7,500 in 2026-05.';
UPDATE claim:c03_0029_051 SET text = 'Ukrainian Air Force/MoD figures put Russian KAB launches at 8,266 in June 2026.' WHERE text = 'Ukrainian Air Force/MoD figures put Russian KAB launches at 8,266 in 2026-06.';
UPDATE claim:c03_0029_052 SET text = 'Ukrainian Air Force/MoD figures put Russian KAB launches at more than 8,300 in July 2026.' WHERE text = 'Ukrainian Air Force/MoD figures put Russian KAB launches at more than 8,300 in 2026-07.';
UPDATE claim:c03_0029_053 SET text = 'Ukrainian Air Force/MoD figures put Russian KAB launches at 8,766 in August 2026.' WHERE text = 'Ukrainian Air Force/MoD figures put Russian KAB launches at 8,766 in 2026-08.';
UPDATE claim:c03_0029_060 SET text = 'Russia launched 33 ballistic missiles at Ukraine in October 2024.' WHERE text = 'Russia launched 33 ballistic missiles at Ukraine in 2024-10.';
UPDATE claim:c03_0029_061 SET text = 'Russia launched 28 ballistic missiles at Ukraine in January 2025.' WHERE text = 'Russia launched 28 ballistic missiles at Ukraine in 2025-01.';
UPDATE claim:c03_0029_062 SET text = 'Russia launched 148 ballistic missiles at Ukraine in October 2025.' WHERE text = 'Russia launched 148 ballistic missiles at Ukraine in 2025-10.';
UPDATE claim:c03_0029_063 SET text = 'Russia launched 91 ballistic missiles at Ukraine in January 2026.' WHERE text = 'Russia launched 91 ballistic missiles at Ukraine in 2026-01.';
UPDATE claim:c03_0029_064 SET text = 'Russia launched 121 ballistic missiles at Ukraine in February 2026.' WHERE text = 'Russia launched 121 ballistic missiles at Ukraine in 2026-02.';
UPDATE claim:c03_0029_065 SET text = 'Russia launched 195 ballistic missiles at Ukraine in July 2026.' WHERE text = 'Russia launched 195 ballistic missiles at Ukraine in 2026-07.';

-- EW spacing: 'N systems per X km of front' -> systems per km
UPDATE observation:c01_0014_01_00 SET unit = unit:per_km, value = 0.1, prov = $p;
UPDATE observation:c01_0029_10_00 SET unit = unit:per_km, value = 0.1, prov = $p;
UPDATE observation:c01_0032_05_00 SET unit = unit:per_km, value = 0.1, prov = $p;
UPDATE observation:c03_0010_07_00 SET unit = unit:per_km, value = 0.5, prov = $p;

-- frequencies: parse ranges from value_text into MHz
UPDATE observation:c02_0006_02_03 SET unit = unit:mhz, value = 900, prov = $p;
UPDATE observation:c02_0006_03_00 SET unit = unit:mhz, low = 300, high = 1100, qualifier = 'range', prov = $p;
UPDATE observation:c02_0010_112_00 SET unit = unit:mhz, low = 14000, high = 14500, qualifier = 'range', prov = $p;
UPDATE observation:c02_0010_116_00 SET unit = unit:mhz, low = 4100, high = 4500, qualifier = 'range', prov = $p;
UPDATE observation:c02_0010_116_01 SET unit = unit:mhz, low = 300, high = 600, qualifier = 'range', prov = $p;
UPDATE observation:c02_0010_118_00 SET unit = unit:mhz, value = 4500, prov = $p;
UPDATE observation:c02_0010_42_00 SET unit = unit:mhz, low = 850, high = 930, qualifier = 'range', prov = $p;
UPDATE observation:c02_0010_43_00 SET unit = unit:mhz, low = 720, high = 1050, qualifier = 'range', prov = $p;
UPDATE observation:c02_0010_44_00 SET unit = unit:mhz, low = 720, high = 1020, qualifier = 'range', prov = $p;
UPDATE observation:c02_0010_85_00 SET unit = unit:mhz, low = 400, high = 490, qualifier = 'range', prov = $p;
UPDATE observation:c02_0010_85_01 SET unit = unit:mhz, low = 720, high = 1020, qualifier = 'range', prov = $p;
UPDATE observation:c02_0010_85_02 SET unit = unit:mhz, low = 2100, high = 2300, qualifier = 'range', prov = $p;
UPDATE observation:c02_0023_04_00 SET unit = unit:mhz, low = 850, high = 930, qualifier = 'range', prov = $p;
UPDATE observation:c02_0023_04_01 SET unit = unit:mhz, low = 720, high = 1020, qualifier = 'range', prov = $p;
UPDATE observation:c02_0027_06_00 SET unit = unit:mhz, low = 300, high = 1100, qualifier = 'range', prov = $p;
UPDATE observation:c02_0029_24_00 SET unit = unit:mhz, low = 14000, high = 14500, qualifier = 'range', prov = $p;
UPDATE observation:c02_0029_52_00 SET unit = unit:mhz, value = 4500, prov = $p;
UPDATE observation:c02_0031_15_00 SET unit = unit:mhz, low = 850, high = 930, qualifier = 'range', prov = $p;
UPDATE observation:c02_0031_15_01 SET unit = unit:mhz, low = 720, high = 1020, qualifier = 'range', prov = $p;
UPDATE observation:c02_0031_17_00 SET unit = unit:mhz, low = 300, high = 1100, qualifier = 'range', prov = $p;
UPDATE observation:c02_0031_67_00 SET unit = unit:mhz, low = 1080, high = 2200, qualifier = 'range', prov = $p;
UPDATE observation:c02_0031_67_01 SET unit = unit:mhz, low = 2860, high = 4500, qualifier = 'range', prov = $p;
UPDATE observation:c02_0031_67_02 SET unit = unit:mhz, low = 4860, high = 6040, qualifier = 'range', prov = $p;
UPDATE observation:c02_0031_67_03 SET unit = unit:mhz, low = 6040, high = 8800, qualifier = 'range', prov = $p;
UPDATE observation:c02_0034_11_00 SET unit = unit:mhz, low = 14000, high = 14500, qualifier = 'range', prov = $p;
UPDATE observation:c05_0034_32_03 SET unit = unit:mhz, low = 14000, high = 14500, qualifier = 'range', prov = $p;

-- temperatures
UPDATE observation:c06_0029_10_00 SET unit = unit:celsius, prov = $p;
UPDATE observation:c06_0038_50_00 SET unit = unit:celsius, prov = $p;

-- 'Dnipro' meaning the city, not the river
DELETE claim:c03_0020_23->about WHERE out = place:dnipro_river;
IF count(SELECT id FROM about WHERE in = claim:c03_0020_23 AND out = place:dnipro_city) = 0 { RELATE claim:c03_0020_23->about->place:dnipro_city SET role = 'location', prov = $p; };
DELETE claim:c03_0027_066->about WHERE out = place:dnipro_river;
IF count(SELECT id FROM about WHERE in = claim:c03_0027_066 AND out = place:dnipro_city) = 0 { RELATE claim:c03_0027_066->about->place:dnipro_city SET role = 'location', prov = $p; };
DELETE claim:c05_0034_24->about WHERE out = place:dnipro_river;
IF count(SELECT id FROM about WHERE in = claim:c05_0034_24 AND out = place:dnipro_city) = 0 { RELATE claim:c05_0034_24->about->place:dnipro_city SET role = 'location', prov = $p; };
DELETE claim:c07_0035_15->about WHERE out = place:dnipro_river;
IF count(SELECT id FROM about WHERE in = claim:c07_0035_15 AND out = place:dnipro_city) = 0 { RELATE claim:c07_0035_15->about->place:dnipro_city SET role = 'location', prov = $p; };
