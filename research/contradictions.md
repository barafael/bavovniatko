<!-- Generated from the knowledge base by db/tools/render.py. Edit the data, not this file (see db/README.md). -->

# Contradictions

Pairs of claims that cannot both be true as stated, judged from rule-based candidates (db/tools/relate.py) and stored as `contradicts` edges. *Explained* means the difference has a stated cause (method, scope, date); *open* means it is unresolved. Claim ids resolve in the knowledge base; source ids in [sources.yaml](sources.yaml).

138 distinct contradictions (duplicate claims collapsed): 138 explained, 0 open.

## 00 Timeline & eras

- **explained** · strength 0.8 · 2 judged pairs — Russia's claimed 6,000 km² of 2025 gains conflicts with DeepState's 4,336 km² and CSIS's 4,831 km².
  - `00-0058-03` Russian 2025 gains were 4,336 km² (DeepState) or 4,831 km² (CSIS), under 1% of Ukraine. *(claimant: DeepState; CSIS; `kyivindependent-2026-basmat-deepstate-2025`, `csis-2026-jones-10-charts`)*
  - `07-0030-17` Russia claimed 6,000 km² of territorial gains in 2025. *(claimant: Government of Russia; `bbc-2022-ukraine-in-maps`)*
  - *Resolution:* Russian official claims include territory not confirmed by independent mappers.
- **explained** · strength 0.8 — Russia's claimed 6,000 km² of 2025 gains conflicts with trackers' 4,300-4,800 km².
  - `00-0005-09` Russia gained about 3,600 km² in 2024 and about 4,300–4,800 km² in 2025, depending on the tracker, buying slow gains at huge cost. *(claimant: —; `csis-2026-jones-10-charts`, `kyivindependent-2026-basmat-deepstate-2025`)*
  - `07-0030-17` Russia claimed 6,000 km² of territorial gains in 2025. *(claimant: Government of Russia; `bbc-2022-ukraine-in-maps`)*
  - *Resolution:* Russian official claims include territory not confirmed by independent mappers.
- **explained** · strength 0.7 · 3 judged pairs — The corps claims about 125 km² for Vivaldi phases 1–2, while outside mappers put the gains at over 200–240 km².
  - `00-0074-53` Key figure: Vivaldi area claims are 75 km² cleared plus 10 km² liberated in phase 1 and 125 km² including 50 km² previously occupied in phases 1–2 (3rd Corps). *(claimant: 3rd Army Corps; `euromaidanpress-2026-thomas-vivaldi`, `hromadske-2026-lisova-vivaldi-phase1`)*
  - `05-0025-34` Outside mappers put Operation Vivaldi's gains higher than the corps claims: Molin at over 200 km², others at up to about 240 km²; the "240 km²" is a mappers' estimate, not an ISW confirmation. *(claimant: Clément Molin; `hromadske-2026-lisova-vivaldi-phase1`, `euromaidanpress-2026-isw-vivaldi-fortress-belt`, `kyivindependent-2026-york-vivaldi-gains`)*
  - *Resolution:* The corps counts only formally cleared or liberated ground; outside OSINT mappers count all area where Russian control ended, including grey zone.
- **explained** · strength 0.6 · 3 judged pairs — CSIS's 275,000-325,000 Russian killed conflicts with Mediazona/BBC's estimate of 438,000-535,000 dead.
  - `00-0009-01` CSIS estimates Russian casualties to December 2025 at about 1.2 million, of whom 275,000–325,000 were killed. *(claimant: CSIS; `csis-2026-jones-10-charts`)*
  - `04-0033-81` Mediazona/BBC estimate Russian dead at about 438,000–535,000. *(claimant: BBC Russian Service; Mediazona; `wikipedia-casualties`)*
  - *Resolution:* Different methods (CSIS assessment vs Mediazona's inheritance-register extrapolation) and probably different cut-off dates.
- **explained** · strength 0.6 · 3 judged pairs — CSIS's 100,000-140,000 Ukrainian dead to December 2025 is well above Zelensky's 55,000 killed stated in February 2026.
  - `00-0074-62` Key figure: CSIS puts Ukrainian casualties to December 2025 at 500,000–600,000, with 100,000–140,000 dead. *(claimant: CSIS; `csis-2026-jones-10-charts`)*
  - `04-0027-77` Zelensky said in February 2026 that 55,000 Ukrainian soldiers had been killed, with a "large number" missing. *(claimant: Volodymyr Zelensky; `euromaidan-2026-thomas-zelensky-55k`)*
  - *Resolution:* Zelensky counts confirmed killed and lists the missing separately; CSIS estimates likely dead including many of the missing.
- **explained** · strength 0.6 · 4 judged pairs — DeepState's published 2023 totals (540 km² lost, 430 regained) differ from year-on-year differencing of DeepState versions (699 and 539 km²).
  - `00-0074-18` Key figure: in 2023 Ukraine lost 540 km² and regained about 430 km² (DeepState). *(claimant: DeepState; `euromaidan-2025-hrudka-2024-3600`)*
  - `07-0030-10` Monthly gross sums of gains and recaptures exceed year-on-year differencing because of back-and-forth churn; for 2023, differencing gives 699 km² gained by Russia and 539 km² recaptured by Ukraine. *(claimant: —; `deepstate-api-history`)*
  - *Resolution:* Published figures vs computed differencing of map versions; different method and possibly different year boundaries.
- **explained** · strength 0.6 — Different totals for Russian gains in 2025: 4,336 km² (DeepState) and 4,831 km² (CSIS) against about 4,700 km² (ISW).
  - `00-0074-39` Key figure: Russian territorial gains in 2025 were 4,336 km² (DeepState) or 4,831 km² (CSIS). *(claimant: DeepState; CSIS; `kyivindependent-2026-basmat-deepstate-2025`, `csis-2026-jones-10-charts`)*
  - `07-0030-16` ISW put Russia's 2025 territorial gains at about 4,700 km². *(claimant: Institute for the Study of War; `bbc-2022-ukraine-in-maps`)*
  - *Resolution:* Different mappers' methods and area accounting; all fall within about 10% of each other.
- **explained** · strength 0.6 · 3 judged pairs — CSIS puts Ukrainian dead at 100,000–140,000 to December 2025, far beyond Zelensky's 46,000 killed in February 2025, a gap that ten months cannot plausibly explain.
  - `00-0009-06` CSIS estimates Ukrainian casualties to December 2025 at 500,000–600,000, of whom 100,000–140,000 were killed. *(claimant: CSIS; `csis-2026-jones-10-charts`)*
  - `04-0027-79` Zelensky's figure for Ukrainian soldiers killed was 46,000 in February 2025. *(claimant: Volodymyr Zelensky; `euromaidan-2026-thomas-zelensky-55k`)*
  - *Resolution:* Zelensky counts confirmed killed only, excluding missing; CSIS estimates likely dead including many of the missing. The periods also differ by ten months.
- **explained** · strength 0.5 · 3 judged pairs — The 3rd Corps claims 125 km² for Vivaldi, analysts and mappers 200-240 km².
  - `00-0062-07` The 3rd Army Corps claims 125 km² in Vivaldi, including 50 km² previously occupied, and 3,000+ Russian losses (unverified). *(claimant: 3rd Army Corps; `euromaidanpress-2026-thomas-vivaldi`)*
  - `00-0074-54` Key figure: analysts and mappers put the Vivaldi area at 200–240 km². *(claimant: Analysts (unnamed); `kyivindependent-2026-farrell-vivaldi`)*
  - *Resolution:* The corps counts ground it confirms as cleared/liberated; mappers measure the wider area of changed control.
- **explained** · strength 0.5 · 2 judged pairs — ISW's net Russian change for March-September 2026 (-195 or +361 km²) is hard to reconcile with DeepState-derived +620 km² for February-September 2026.
  - `00-0074-52` Key figure: ISW's Russian net change from 8 March to 8 September 2026 was −195.19 km² for advanced areas and +361.47 km² including infiltration. *(claimant: Institute for the Study of War; `isw-2026-09-08-roca`)*
  - `07-0029-08` Computed from DeepStateMap versions, the net Russian territorial change in era e9 (Feb–Sep 2026) was +620 km², about 78 km² a month, against 444 km² recaptured by Ukraine. *(claimant: —; `deepstate-api-history`)*
  - *Resolution:* Different mappers and methods (ISW advanced/infiltration categories vs DeepState line), and the DeepState window includes February.
- **explained** · strength 0.5 — The SBU flew 117 FPVs in Spiderweb by the official account, while Butusov says 116 were launched.
  - `00-0055-09` In Operation Spiderweb on 1 June 2025 the SBU flew 117 FPVs from trucks against five Russian airbases. *(claimant: —; `euromaidanpress-2025-tril-spiderweb`)*
  - `c16-0026-13` Butusov says 150 drones were smuggled into Russia for Operation Spiderweb and 116 launched. *(claimant: Yurii Butusov; `texty-2025-butusov-pavutyna-preparation`)*
  - *Resolution:* Minor count difference; the official 117 may include a drone that did not take off.
- **explained** · strength 0.5 · 9 judged pairs — Ukrainian troops at Avdiivka reported 5:1 in early 2024 while another report gives 1:10 at Avdiivka in February 2024.
  - `00-0074-21` Key figure: the shell ratio reported by Ukrainian troops in early 2024 was 5:1 in Russia's favour, 8–9:1 at worst. *(claimant: Ukrainian forces (unnamed); `wikipedia-battle-of-avdiivka`)*
  - `c11-0032-20` The artillery shell ratio at Avdiivka reached 1:10 in February 2024. *(claimant: —; `liga-2025-shell-ratio`)*
  - *Resolution:* Anecdotal ratios from different sectors and days; 8–9:1 at worst is close to 10:1.
- **explained** · strength 0.5 · 3 judged pairs — RUSI's 70,000 UMPK kits for 2025 conflicts with HUR's claim of more than 100,000 a year.
  - `00-0057-01` Russia ordered 70,000 UMPK glide-bomb kits for 2025. *(claimant: RUSI; `rusi-2025-watling-third-year`)*
  - `03-0031-06` Russian UMPK production went from tens of thousands to more than 100,000 a year. *(claimant: Defence Intelligence of Ukraine (HUR); `pravda-2025-hur-glide-bomb-production`)*
  - *Resolution:* RUSI counts kits ordered; HUR cites planned production (120,000 for 2025).
- **explained** · strength 0.5 · 3 judged pairs — British intelligence's about 500,000 Russian killed (to September 2026) is hard to reconcile with CSIS's 275,000–325,000 killed to December 2025, since nine months cannot add about 200,000 dead.
  - `00-0009-02` British intelligence estimates about 1.5 million Russian casualties and about 500,000 killed. *(claimant: British intelligence; `euromaidan-2026-tril-cost-per-km`, `kyivpost-2026-korshak-confirmed-dead`)*
  - `04-0033-85` CSIS estimates Russian casualties to December 2025 at about 1.2M in total, with 275,000–325,000 killed. *(claimant: CSIS; `csis-2026-jones-grinding-war`)*
  - *Resolution:* The methods differ in the killed-to-wounded ratio, and the periods differ (CSIS to December 2025, UK to September 2026); the total casualty figures (1.2M vs 1.5M) are closer.
- **explained** · strength 0.5 — ISW's about 305 km² retaken by Ukraine at Kupiansk in late December 2025 conflicts with only 21 km² of Ukrainian recapture computed from DeepState for December 2025.
  - `00-0055-07` Ukraine retook much of Kupiansk and its surroundings, about 305 km², in late December 2025. *(claimant: Institute for the Study of War; `isw-2026-02-15-roca`)*
  - `07-0028-27` Computed from DeepStateMap versions, in December 2025 (e8) Russia gained 463 km² and Ukraine recaptured 21 km² (gross), a net Russian change of +445 km²; the grey zone was 1,370 km² at the end of the period. *(claimant: —; `deepstate-api-history`)*
  - *Resolution:* The computed series is dated by publication and DeepState withholds Ukrainian gains for OPSEC; ISW's assessment also differs in method.
- **explained** · strength 0.5 · 2 judged pairs — RUSI gives 19,547 Wagner dead at Bakhmut for December 2022-May 2023, while Kirby put all Russian dead since December at about 20,000, nearly half of them Wagner (about 10,000).
  - `00-0074-09` Key figure: 19,547 Wagner fighters were killed at Bakhmut, 88% of them convicts. *(claimant: RUSI; `rusi-2024-watling-offensive-lessons`)*
  - `c07-0007-22` Kirby said on 1 May 2023 that Russia had suffered about 20,000 killed and 80,000 wounded on all fronts since December, nearly half the dead Wagner. *(claimant: John Kirby; `pravda-2023-wp-carnage-kirby`)*
  - *Resolution:* RUSI's figure likely covers the whole Bakhmut battle from mid-2022 and draws on Wagner's own records; Kirby's US estimate covers only December-April.
- **explained** · strength 0.4 — A's 70,000 UMPK kits ordered for 2025 (RUSI) conflicts with HUR's claim of more than 100,000 a year.
  - `00-0077-20` In e8 Russia fought drone-centric, infantry-infiltration warfare with fibre-optic FPVs, 50,000+ Shaheds a year and 70k UMPK ordered. *(claimant: —; `csis-2026-jones-10-charts`, `rusi-2025-watling-third-year`)*
  - `03-0031-06` Russian UMPK production went from tens of thousands to more than 100,000 a year. *(claimant: Defence Intelligence of Ukraine (HUR); `pravda-2025-hur-glide-bomb-production`)*
  - *Resolution:* RUSI counts kits ordered; HUR cites planned production (120,000 for 2025).

## 01 Drones & uncrewed systems

- **explained** · strength 0.5 · 3 judged pairs — SBU says 117 drones were used in Spiderweb while Butusov says 116 were launched.
  - `01-0029-46` Ukraine claims Operation Spiderweb used 117 drones, hit 41 aircraft and caused about $7bn in damage. *(claimant: Security Service of Ukraine (SBU); `kyivindependent-2025-york-spiderweb`)*
  - `c16-0026-13` Butusov says 150 drones were smuggled into Russia for Operation Spiderweb and 116 launched. *(claimant: Yurii Butusov; `texty-2025-butusov-pavutyna-preparation`)*
  - *Resolution:* Minor count difference; one drone may have been used but not launched, or sources count differently.

## 02 EW & countermeasures

- **explained** · strength 0.4 · 2 judged pairs — The Atlantic Council's more than 200 km² retaken within five days of the Starlink cut conflicts with only 48 km² of Ukrainian recapture computed from DeepState for all of February 2026.
  - `02-0031-49` More than 200 km² was retaken within 5 days of the Starlink cut; the figure is contested. *(claimant: Atlantic Council; `atlanticcouncil-2026-spencer-starlink-crisis`)*
  - `07-0028-29` Computed from DeepStateMap versions, in February 2026 (e9) Russia gained 171 km² and Ukraine recaptured 48 km² (gross), a net Russian change of +126 km²; the grey zone was 1,567 km² at the end of the period. Context: after the Ukrainian counteroffensive from 29 January to 9 February. *(claimant: —; `deepstate-api-history`)*
  - *Resolution:* Different mappers; the counteroffensive began on 29 January, and DeepState publishes Ukrainian gains late.

## 03 Fires & air

- **explained** · strength 0.7 — The Air Force's 5,171 and the MoD's more than 5,700 KABs for January 2026 are incompatible counts of the same quantity.
  - `03-0024-12` Russia dropped 5,171 KABs on Ukraine in January 2026, according to Ukrainian Air Force data reported by Kyiv Post. *(claimant: Ukrainian Air Force; `kyivpost-2026-korshak-glide-bombs`)*
  - `03-0024-13` The Ukrainian MoD counted more than 5,700 Russian KABs in January 2026, with a record 316 on 18 January. *(claimant: Ministry of Defence of Ukraine; `united24-2026-mykhailenko-january-kabs`)*
  - *Resolution:* Two Ukrainian official sources; likely different counting rules (e.g. which guided bomb types are included) or reporting cut-offs.
- **explained** · strength 0.6 — About 130 KABs a day in 2025 (44,000 in January-November) conflicts with HUR's 200-250 launched a day.
  - `03-0022-21` Russia launched about 3,500 KABs in November 2025, for about 44,000 in January–November 2025 (130 a day). *(claimant: —; `euromaidanpress-2025-mukhina-84000-kabs`)*
  - `03-0022-22` HUR says Russia planned 120,000 glide bombs for 2025, with 200–250 launched a day. *(claimant: Defence Intelligence of Ukraine (HUR); `pravda-2025-hur-glide-bomb-production`)*
  - *Resolution:* HUR's claim sits alongside planned production of 120,000 and may be a projection or count all glide bombs; the monthly Air Force totals give about 130.

## 04 Ground tactics & manpower

- **explained** · strength 0.8 · 9 judged pairs — DeepState's 36 km² and Radio Svoboda's 27 km² of Russian gains for July 2026 conflict.
  - `04-0027-30` DeepState reported 36 km² of Russian gains for July 2026. *(claimant: DeepState; `kyivpost-2026-zavadska-deepstate-july`)*
  - `07-0029-12` Radio Svoboda figures, routed through Euromaidan Press, put Russian gains at 450–550 km² a month at the 2025 peak and 27 km² in July 2026. *(claimant: Radio Svoboda; `euromaidan-2026-tril-cost-per-km`)*
  - *Resolution:* Radio Svoboda's figure is likely an error or a different source; DeepState's own report and the computed series give 36 km².
- **explained** · strength 0.8 · 3 judged pairs — Radio Svoboda's 27 km² and the computed DeepState net of 36 km² for July 2026 conflict.
  - `04-0033-57` Radio Svoboda gives 27 km² of Russian territorial gains for July 2026. *(claimant: Radio Svoboda; `euromaidan-2026-tril-cost-per-km`)*
  - `07-0026-21` The net Russian territorial gain computed from DeepStateMap versions for July 2026 is 36 km². *(claimant: —; `deepstate-api-history`)*
  - *Resolution:* Radio Svoboda's figure is likely an error or a different source; DeepState's own report also gives 36 km².
- **explained** · strength 0.7 · 4 judged pairs — OSINT puts Vivaldi phase 1 at over 200 km², while the corps claims 75 km² cleared plus 10 km² liberated.
  - `04-0027-10` OSINT estimates of the area retaken in Operation Vivaldi's first phase exceed 200 km². *(claimant: Analysts (unnamed); `kyivindependent-2026-farrell-vivaldi`)*
  - `04-0033-60` Operation Vivaldi phase 1 cleared 75 km² and liberated 10 km², according to the corps. *(claimant: 3rd Army Corps; `kyivindependent-2026-farrell-vivaldi`)*
  - *Resolution:* The corps counts only formally cleared or liberated ground; outside OSINT mappers count all area where Russian control ended, including grey zone.
- **explained** · strength 0.7 · 3 judged pairs — The corps claims 125 km² retaken in Vivaldi, while mappers put the area at 200–240 km².
  - `04-0027-20` The 3rd Army Corps claims 125 km² retaken over the two phases of Operation Vivaldi. *(claimant: 3rd Army Corps; `euromaidan-2026-vivdych-robot-demining`)*
  - `04-0033-63` Mappers estimate the Operation Vivaldi area at 200–240 km². *(claimant: Analysts (unnamed); `kyivindependent-2026-york-vivaldi-gains`)*
  - *Resolution:* The corps counts only formally cleared or liberated ground; outside OSINT mappers count all area where Russian control ended, including grey zone.
- **explained** · strength 0.6 — UALosses has named 96,821 Ukrainian dead by June 2026, far above Zelensky's 55,000 killed stated in February 2026; a four-month gap cannot explain the difference.
  - `04-0027-77` Zelensky said in February 2026 that 55,000 Ukrainian soldiers had been killed, with a "large number" missing. *(claimant: Volodymyr Zelensky; `euromaidan-2026-thomas-zelensky-55k`)*
  - `04-0027-80` UALosses has named 96,821 Ukrainian dead and 97,938 missing to 21 June 2026. *(claimant: UALosses; `ualosses-2026-soldiers`)*
  - *Resolution:* Zelensky's figure counts officially confirmed killed in action and excludes the missing; UALosses names dead from open sources, possibly including deaths not officially confirmed.
- **explained** · strength 0.6 · 2 judged pairs — Radio Svoboda, citing DeepState, gives 160 km² of Russian gains for April 2026, while the computed DeepState series gives 141 km² net (151 gross).
  - `04-0033-55` DeepState's Russian monthly territorial gains were 450–550 km² at the 2025 peak and 160 km² in April 2026. *(claimant: DeepState; `euromaidan-2026-tril-cost-per-km`)*
  - `07-0028-31` Computed from DeepStateMap versions, in April 2026 (e9) Russia gained 151 km² and Ukraine recaptured 13 km² (gross), a net Russian change of +141 km²; the grey zone was 1,550 km² at the end of the period. *(claimant: —; `deepstate-api-history`)*
  - *Resolution:* Relayed figure vs computed differencing; possibly different cut-off dates or gross vs net.
- **explained** · strength 0.6 · 2 judged pairs — DeepState's 2025 peak of 450–550 km² a month is below Slivochny Kapriz's 550–600 km² a month for 2025.
  - `04-0033-55` DeepState's Russian monthly territorial gains were 450–550 km² at the 2025 peak and 160 km² in April 2026. *(claimant: DeepState; `euromaidan-2026-tril-cost-per-km`)*
  - `07-0030-13` The Russian project Slivochny Kapriz, which credits any area where Russian troops were seen, gives 550–600 km² a month of Russian gains for 2025. *(claimant: Slivochny Kapriz; `euromaidan-2026-tril-cost-per-km`)*
  - *Resolution:* Slivochny Kapriz credits any area where Russian troops were seen; DeepState counts confirmed control.
- **explained** · strength 0.5 · 6 judged pairs — HUR puts North Korean casualties at more than 7,000 in June 2026, while the NIS and UK put them at about 6,000 over the same period.
  - `04-0033-35` HUR put North Korean casualties at more than 7,000 in June 2026. *(claimant: Defence Intelligence of Ukraine (HUR); `kyivindependent-2026-terajima-nk-hur`)*
  - `c14-0027-09` In February–June 2026 the NIS and UK put North Korean casualties at about 6,000. *(claimant: National Intelligence Service (South Korea); UK Government; `kyivindependent-2026-terajima-nk-hur`, `osw-2026-wilk-zaporizhzhia`)*
  - *Resolution:* Different intelligence services with different methods; HUR's figure is the higher and less conservative one.
- **explained** · strength 0.4 — 17 detachments of about 474 people imply roughly 8,000 Rubikon personnel, well above the ~5,000 in A.
  - `04-0030-37` Tactic "Dedicated deep drone-interdiction unit (Rubikon)" (RU, e8, e9): an elite drone centre of about 5,000 cuts main supply roads and hunts drone crews, free of any duty to support infantry. *(claimant: —; `euromaidan-2025-mukhina-rubikon-izium`)*
  - `05-0035-03` By 2026 Rubikon had 17 detachments of about 474 people each, with FPV, fixed-wing FPV, Lancet/Supercam, Orlan and counter-UAS/EW teams. *(claimant: Foreign Policy Research Institute; `fpri-2026-lee-putiata-rubicon`)*
  - *Resolution:* 474 is probably the upper detachment size (detachments grew from 149 to 474), so multiplying overstates the total; FPRI itself gives about 5,000 in spring 2026.

## 05 ISR, C2 & logistics

- **explained** · strength 0.4 — ISW data show 201 km² of Ukrainian gains in one week after the Starlink cutoff, while DeepState versions give only 48 km² of Ukrainian recapture for all of February 2026.
  - `05-0025-22` Ukraine gained 201 km² in one week after the Starlink cutoff, per ISW data. *(claimant: Institute for the Study of War; `united24-2026-litnarovych-starlink-gains`)*
  - `07-0028-29` Computed from DeepStateMap versions, in February 2026 (e9) Russia gained 171 km² and Ukraine recaptured 48 km² (gross), a net Russian change of +126 km²; the grey zone was 1,567 km² at the end of the period. Context: after the Ukrainian counteroffensive from 29 January to 9 February. *(claimant: —; `deepstate-api-history`)*
  - *Resolution:* Different mappers; the week straddles late January and early February, the source has a date error, and DeepState publishes Ukrainian gains late.
- **explained** · strength 0.4 — 200-201 km² of Ukrainian gains in the first week after the Starlink cutoff conflicts with only 48 km² of Ukrainian recapture computed from DeepState for all of February 2026.
  - `05-0031-22` Ukrainian gains in the first week after the Starlink cutoff were 200–201 km². *(claimant: —; `atlanticcouncil-2026-spencer-starlink-crisis`, `united24-2026-litnarovych-starlink-gains`)*
  - `07-0028-29` Computed from DeepStateMap versions, in February 2026 (e9) Russia gained 171 km² and Ukraine recaptured 48 km² (gross), a net Russian change of +126 km²; the grey zone was 1,567 km² at the end of the period. Context: after the Ukrainian counteroffensive from 29 January to 9 February. *(claimant: —; `deepstate-api-history`)*
  - *Resolution:* Different mappers; the week straddles late January and early February, and DeepState publishes Ukrainian gains late.

## 06 Terrain & geodata

- **explained** · strength 0.6 — In the same Robotyne sample box, one computation gives a main-belt spacing of about 1,000–1,154 m while the other gives 0.5–0.6 km.
  - `06-0025-12` In the Robotyne sample box, main-belt spacing is 1,154 m by OSM median (p10–p90 271–1,790 m) and about 1,000 m by Sentinel-2. *(claimant: —; `eox-2021-s2cloudless`, `osm-overpass-2026-landscape-sample`)*
  - `c09-0016-04` In the OSM and Sentinel-2 sample box around Robotyne, tree lines run roughly E–W and N–S; main belts are about 0.5–0.6 km apart and cross-belts about 1.0–1.2 km apart, at 1.34 km of mapped belt per km². *(claimant: —; `eox-2021-s2cloudless`, `osm-overpass-2026-landscape-sample`)*
  - *Resolution:* The two computations probably label main and cross belts differently; 1.0–1.2 km matches the other's cross-belt spacing.
- **explained** · strength 0.5 — A gives Robotyne cross-belt spacing of about 610 m by Sentinel-2, while B gives cross-belts 1.0-1.2 km apart from the same imagery.
  - `06-0025-16` In the Robotyne sample box, cross-belt spacing is 1,134 m by OSM (OSM misses belts) and about 610 m by Sentinel-2. *(claimant: —; `eox-2021-s2cloudless`, `osm-overpass-2026-landscape-sample`)*
  - `c09-0016-04` In the OSM and Sentinel-2 sample box around Robotyne, tree lines run roughly E–W and N–S; main belts are about 0.5–0.6 km apart and cross-belts about 1.0–1.2 km apart, at 1.34 km of mapped belt per km². *(claimant: —; `eox-2021-s2cloudless`, `osm-overpass-2026-landscape-sample`)*
  - *Resolution:* A's OSM 1,134 m agrees with B; its ~610 m Sentinel-2 figure matches B's main-belt spacing, which suggests main and cross belts were mixed up.
- **explained** · strength 0.4 · 4 judged pairs — Length of the Kupiansk pipeline route: about 10 km vs 15 km.
  - `06-0017-08` Russian troops entered Kupiansk through the disused Shebelynka–Ostrohozhsk gas pipeline, "a walk of about 10 km in darkness" that exits in a forest. *(claimant: —; `euromaidan-2026-zoria-kupiansk-pipeline`)*
  - `c17-0012-06` The Kupiansk pipeline runs 15 km from the Russian rear to Ukrainian lines. *(claimant: —; `kyivindependent-2026-terajima-kupiansk-pipelines`)*
  - *Resolution:* Possibly different measures: the walk inside the pipe vs the full run from the Russian rear.
- **explained** · strength 0.4 · 2 judged pairs — 709,900 ha flooded is incompatible with a 2,155 km² reservoir surface.
  - `06-0017-20` The Kakhovka reservoir was created by flooding 709,900 ha, including the Velykyi Luh ("Great Meadow") of up to 80,000 ha. *(claimant: —; `uncg-2022-shamina-stalin-plan`)*
  - `c10-0025-02` The Kakhovka reservoir shrank from 2,155 km² to 509 km² by 20 June 2023. *(claimant: —; `wikipedia-kakhovka-dam-destruction`)*
  - *Resolution:* The 709,900 ha figure likely counts all land taken or affected by the project, not the water surface.

## 07 Maps & UI conventions

- **explained** · strength 0.8 · 3 judged pairs — The computed DeepState net of +36 km² and Radio Svoboda's 27 km² for July 2026 conflict.
  - `07-0028-34` Computed from DeepStateMap versions, in July 2026 (e9) Russia gained 86 km² and Ukraine recaptured 52 km² (gross), a net Russian change of +36 km²; the grey zone was 1,796 km² at the end of the period. Context: with Ukrainian gains withheld for OPSEC. *(claimant: —; `deepstate-api-history`)*
  - `07-0029-12` Radio Svoboda figures, routed through Euromaidan Press, put Russian gains at 450–550 km² a month at the 2025 peak and 27 km² in July 2026. *(claimant: Radio Svoboda; `euromaidan-2026-tril-cost-per-km`)*
  - *Resolution:* Radio Svoboda's figure is likely an error or a different source; DeepState's own report also gives 36 km².
- **explained** · strength 0.8 — UK MoD (143 km²) and ISW (about 203 km²) give different Russian gains for March 2025.
  - `07-0028-41` The UK Ministry of Defence put Russia's territorial gain in March 2025 at 143 km². *(claimant: UK Ministry of Defence; `euromaidan-2025-shandra-gains-plummet`)*
  - `07-0028-42` ISW put Russia's territorial gain in March 2025 at about 203 km². *(claimant: Institute for the Study of War; `euromaidan-2025-kravchuk-assaults-may`)*
  - *Resolution:* Different assessors with different mapping methods.
- **explained** · strength 0.8 — ISW assesses about 4,700 km² of Russian gains in 2025, while Russia claims 6,000 km².
  - `07-0030-16` ISW put Russia's 2025 territorial gains at about 4,700 km². *(claimant: Institute for the Study of War; `bbc-2022-ukraine-in-maps`)*
  - `07-0030-17` Russia claimed 6,000 km² of territorial gains in 2025. *(claimant: Government of Russia; `bbc-2022-ukraine-in-maps`)*
  - *Resolution:* Russian official claims routinely overstate captures; ISW counts only geolocated, assessed control.
- **explained** · strength 0.7 — DeepState's published 3,600+ km² for 2024 conflicts with 3,301 km² computed from DeepState's own versions.
  - `07-0026-24` DeepState's published net Russian territorial gain for the year 2024 was 3,600+ km² (per a Mil.in.ua summary). *(claimant: DeepState; `euromaidan-2025-hrudka-2024-3600`)*
  - `07-0026-25` The net Russian territorial gain computed from DeepStateMap versions for the year 2024 is 3,301 km², against DeepState's 3,600+ as summarised by Mil.in.ua. *(claimant: —; `deepstate-api-history`)*
  - *Resolution:* Published summary vs computed year-on-year differencing; the gap is unexplained.
- **explained** · strength 0.7 — DeepState's quoted 'some 630 km²' net Russian gain for November 2025 conflicts with the 503 km² computed from DeepStateMap versions for the same month.
  - `07-0026-09` DeepState's published net Russian territorial gain for November 2025 was some 630 km². *(claimant: DeepState; `euromaidan-2026-zoria-february-126`)*
  - `07-0026-10` The net Russian territorial gain computed from DeepStateMap versions for November 2025 is 503 km², an unexplained gap against DeepState's quoted "some 630". *(claimant: —; `deepstate-api-history`)*
  - *Resolution:* The gap is unexplained; it is likely a different cutoff window, or gross rather than net, in DeepState's published summary.
- **explained** · strength 0.7 — DeepState's quoted November 2025 net Russian gain of some 630 km² conflicts with 503 km² computed from DeepState's own map versions.
  - `07-0026-09` DeepState's published net Russian territorial gain for November 2025 was some 630 km². *(claimant: DeepState; `euromaidan-2026-zoria-february-126`)*
  - `07-0028-26` Computed from DeepStateMap versions, in November 2025 (e8) Russia gained 509 km² and Ukraine recaptured 9 km² (gross), a net Russian change of +503 km²; the grey zone was 1,350 km² at the end of the period. *(claimant: —; `deepstate-api-history`)*
  - *Resolution:* Unexplained gap; possibly a gross figure, grey-zone inclusion, or a misquotation of DeepState's monthly post.
- **explained** · strength 0.7 · 2 judged pairs — The DeepState-derived +729 km² and ISW's about 627 km² for Russian gains in November 2024 conflict.
  - `07-0028-14` Computed from DeepStateMap versions, in November 2024 (e7) Russia gained 728 km² and Ukraine recaptured 1 km² (gross), a net Russian change of +729 km²; the grey zone was 740 km² at the end of the period. Context: the peak month after 2022. *(claimant: —; `deepstate-api-history`)*
  - `07-0028-40` ISW assessed Russia's net territorial gain in November 2024 at about 627 km². *(claimant: Institute for the Study of War; `euromaidan-2025-kravchuk-assaults-may`)*
  - *Resolution:* Different mappers and methods; DeepState counts grey-zone transitions differently from ISW's assessed advances.
- **explained** · strength 0.7 — The DeepState-derived +134 km² and ISW's about 203 km² for Russian gains in March 2025 conflict.
  - `07-0028-18` Computed from DeepStateMap versions, in March 2025 (e7/e8) Russia gained 142 km² and Ukraine recaptured 11 km² (gross), a net Russian change of +134 km²; the grey zone was 775 km² at the end of the period. *(claimant: —; `deepstate-api-history`)*
  - `07-0028-42` ISW put Russia's territorial gain in March 2025 at about 203 km². *(claimant: Institute for the Study of War; `euromaidan-2025-kravchuk-assaults-may`)*
  - *Resolution:* Different mappers and methods; ISW's assessed advances are usually larger than DeepState's.
- **explained** · strength 0.7 — DeepState's 133 km² and ISW's about 203 km² for Russian gains in March 2025 conflict.
  - `07-0026-05` DeepState's published net Russian territorial gain for March 2025 was 133 km². *(claimant: DeepState; `euromaidan-2025-shandra-gains-plummet`)*
  - `07-0028-42` ISW put Russia's territorial gain in March 2025 at about 203 km². *(claimant: Institute for the Study of War; `euromaidan-2025-kravchuk-assaults-may`)*
  - *Resolution:* Different mappers and methods; ISW's assessed advances are usually larger than DeepState's.
- **explained** · strength 0.7 — Net Russian gain for November 2024 is 700+ km² (DeepState) vs about 627 km² (ISW).
  - `07-0026-03` DeepState's published net Russian territorial gain for November 2024 was 700+ km². *(claimant: DeepState; `euromaidan-2025-shandra-gains-plummet`)*
  - `07-0028-40` ISW assessed Russia's net territorial gain in November 2024 at about 627 km². *(claimant: Institute for the Study of War; `euromaidan-2025-kravchuk-assaults-may`)*
  - *Resolution:* Different trackers: DeepState and ISW map control differently and use different cut-off dates.
- **explained** · strength 0.7 — The DeepState-derived +134 km² and ISW's about 203 km² for Russian gains in March 2025 conflict.
  - `07-0026-06` The net Russian territorial gain computed from DeepStateMap versions for March 2025 is 134 km². *(claimant: —; `deepstate-api-history`)*
  - `07-0028-42` ISW put Russia's territorial gain in March 2025 at about 203 km². *(claimant: Institute for the Study of War; `euromaidan-2025-kravchuk-assaults-may`)*
  - *Resolution:* Different mappers and methods; ISW's assessed advances are usually larger than DeepState's.
- **explained** · strength 0.6 · 2 judged pairs — Ukrainian gains in Dec 2025: 21 km² (DeepState) vs 305 km² (ISW).
  - `07-0028-27` Computed from DeepStateMap versions, in December 2025 (e8) Russia gained 463 km² and Ukraine recaptured 21 km² (gross), a net Russian change of +445 km²; the grey zone was 1,370 km² at the end of the period. *(claimant: —; `deepstate-api-history`)*
  - `c17-0005-04` ISW counts 305 km² lost to Russian control in late December 2025 as Ukraine liberated much of Kupiansk and its surroundings. *(claimant: Institute for the Study of War; `isw-2026-02-15-roca`)*
  - *Resolution:* ISW's control-of-terrain assessment vs DeepState map versions; timing and grey-zone treatment differ.
- **explained** · strength 0.6 — DeepState-derived 4,336 km² for 2025 conflicts with Slivochny Kapriz's 550-600 km² a month (about 6,600-7,200 km² a year).
  - `07-0026-23` The net Russian territorial gain computed from DeepStateMap versions for the year 2025 is 4,336 km². *(claimant: —; `deepstate-api-history`)*
  - `07-0030-13` The Russian project Slivochny Kapriz, which credits any area where Russian troops were seen, gives 550–600 km² a month of Russian gains for 2025. *(claimant: Slivochny Kapriz; `euromaidan-2026-tril-cost-per-km`)*
  - *Resolution:* Slivochny Kapriz credits any area where Russian troops were seen, including infiltration; DeepState marks control.
- **explained** · strength 0.6 — The computed May 2026 gross changes (78 km² Russian, 66 km² Ukrainian) conflict with Kyiv Post's quoted DeepState figures of 130 km² lost and 250 km² regained.
  - `07-0028-32` Computed from DeepStateMap versions, in May 2026 (e9) Russia gained 78 km² and Ukraine recaptured 66 km² (gross), a net Russian change of +14 km²; the grey zone was 1,589 km² at the end of the period. *(claimant: —; `deepstate-api-history`)*
  - `07-0028-44` Kyiv Post quotes DeepState as reporting 130 km² lost and 250 km² regained by Ukraine in May 2026, conflicting with DeepState's net 14 km² Russian gain per RFE/RL. *(claimant: DeepState; Kyiv Post; `kyivpost-2026-zavadska-deepstate-july`)*
  - *Resolution:* The computed net of +14 km² matches DeepState's net per RFE/RL; the Kyiv Post quote may be a misquotation or cover a different window (e.g. OPSEC releases).
- **explained** · strength 0.6 — DeepStateMap versions give Russia 85 km² gained in June 2026, while a report says front-wide Russian gains fell to about 30 km².
  - `07-0028-33` Computed from DeepStateMap versions, in June 2026 (e9) Russia gained 85 km² and Ukraine recaptured 2 km² (gross), a net Russian change of +84 km²; the grey zone was 1,635 km² at the end of the period. *(claimant: —; `deepstate-api-history`)*
  - `c19-0009-46` Russia's front-wide gains fell to about 30 km² in June 2026. *(claimant: —; `euromaidanpress-2026-axe-potemkin-flags`)*
  - *Resolution:* The report may use another mapping source or a narrower definition of gains.
- **explained** · strength 0.6 — DeepState's 4,336 km² for 2025 conflicts with Slivochny Kapriz's 550-600 km² a month (about 6,600-7,200 km² a year).
  - `07-0026-22` DeepState's published net Russian territorial gain for the year 2025 was 4,336 km². *(claimant: DeepState; `euromaidan-2026-tril-2025-4336`)*
  - `07-0030-13` The Russian project Slivochny Kapriz, which credits any area where Russian troops were seen, gives 550–600 km² a month of Russian gains for 2025. *(claimant: Slivochny Kapriz; `euromaidan-2026-tril-cost-per-km`)*
  - *Resolution:* Slivochny Kapriz credits any area where Russian troops were seen, including infiltration; DeepState marks control.
- **explained** · strength 0.5 — Monthly Russian gains in 2025 given as 450–550 km² (Radio Svoboda/DeepState) vs 550–600 km² (Slivochny Kapriz).
  - `07-0029-12` Radio Svoboda figures, routed through Euromaidan Press, put Russian gains at 450–550 km² a month at the 2025 peak and 27 km² in July 2026. *(claimant: Radio Svoboda; `euromaidan-2026-tril-cost-per-km`)*
  - `07-0030-13` The Russian project Slivochny Kapriz, which credits any area where Russian troops were seen, gives 550–600 km² a month of Russian gains for 2025. *(claimant: Slivochny Kapriz; `euromaidan-2026-tril-cost-per-km`)*
  - *Resolution:* Slivochny Kapriz credits any area where Russian troops were seen, inflating gains relative to control-based mapping.
- **explained** · strength 0.5 — DeepState's 133 km² and the UK MoD's 143 km² for Russian gains in March 2025 differ.
  - `07-0026-05` DeepState's published net Russian territorial gain for March 2025 was 133 km². *(claimant: DeepState; `euromaidan-2025-shandra-gains-plummet`)*
  - `07-0028-41` The UK Ministry of Defence put Russia's territorial gain in March 2025 at 143 km². *(claimant: UK Ministry of Defence; `euromaidan-2025-shandra-gains-plummet`)*
  - *Resolution:* Different assessors with different mapping methods; the gap is small.
- **explained** · strength 0.5 — The DeepState-derived 134 km² and the UK MoD's 143 km² for Russian gains in March 2025 differ.
  - `07-0026-06` The net Russian territorial gain computed from DeepStateMap versions for March 2025 is 134 km². *(claimant: —; `deepstate-api-history`)*
  - `07-0028-41` The UK Ministry of Defence put Russia's territorial gain in March 2025 at 143 km². *(claimant: UK Ministry of Defence; `euromaidan-2025-shandra-gains-plummet`)*
  - *Resolution:* Different assessors with different mapping methods; the gap is small.
- **explained** · strength 0.5 — The DeepState-derived 134 km² and the UK MoD's 143 km² for Russian gains in March 2025 differ.
  - `07-0028-18` Computed from DeepStateMap versions, in March 2025 (e7/e8) Russia gained 142 km² and Ukraine recaptured 11 km² (gross), a net Russian change of +134 km²; the grey zone was 775 km² at the end of the period. *(claimant: —; `deepstate-api-history`)*
  - `07-0028-41` The UK Ministry of Defence put Russia's territorial gain in March 2025 at 143 km². *(claimant: UK Ministry of Defence; `euromaidan-2025-shandra-gains-plummet`)*
  - *Resolution:* Different assessors with different mapping methods; the gap is small.

## c01 Chapter: The march on Kyiv

- **explained** · strength 0.6 — MWI puts the Northern Front from Belarus at about 30,000 troops; Pavliuk claims about 70,000 came from Belarus.
  - `c01-0025-02` The Russian Northern Front had about 30,000 troops from the 29th, 35th and 36th Combined Arms Armies of the Eastern Military District, staged in Belarus on the west bank of the Dnipro. *(claimant: Modern War Institute at West Point; `mwi-2025-collins-spencer-battle-of-kyiv`)*
  - `c01-0025-04` Pavliuk claims about 70,000 Russian men and 7,000 armoured vehicles came from Belarus alone. *(claimant: Oleksandr Pavliuk; `ukrinform-2023-pavliuk-30-sof`)*
  - *Resolution:* MWI counts the three EMD combined-arms armies; Pavliuk's figure likely includes VDV, Rosgvardia and support troops, or is inflated.
- **explained** · strength 0.5 — MWI puts the whole Northeastern Front (41st and 2nd CAA) at about 20,000, while Wikipedia gives the 41st CAA alone about 30,000 at Chernihiv.
  - `c01-0025-03` The Russian Northeastern Front had about 20,000 troops from the 41st and 2nd Combined Arms Armies of the Central Military District, on the east bank of the Dnipro. *(claimant: Modern War Institute at West Point; `mwi-2025-collins-spencer-battle-of-kyiv`)*
  - `c01-0025-24` The 41st Combined Arms Army had about 30,000 troops at Chernihiv. *(claimant: —; `wikipedia-2026-siege-of-chernihiv`)*
  - *Resolution:* Different counting; the Wikipedia figure likely includes attached units or is inflated.

## c02 Chapter: Snake Island → the Moskva

- **explained** · strength 0.9 · 2 judged pairs — Moskva crew rescued: 396 vs 58.
  - `c02-0009-08` The Russian MoD said in April 2022 that the Moskva sinking left 1 dead, 27 missing and 396 rescued. *(claimant: Russian Ministry of Defence; `wikipedia-sinking-of-the-moskva`)*
  - `c02-0009-13` Danilov (NSDC) claimed only 58 of the Moskva's 510 crew were rescued. *(claimant: Oleksiy Danilov; `ukranews-2022-danilov-58-rescued`)*
  - *Resolution:* Russian MoD vs Ukrainian NSDC claims; each side has motives.
- **explained** · strength 0.9 — Moskva crew rescued: 396 vs 58.
  - `c02-0011-43` On 22 Apr 2022 the Russian MoD gave the Moskva losses as 1 dead, 27 missing and 396 rescued. *(claimant: Russian Ministry of Defence; `wikipedia-sinking-of-the-moskva`)*
  - `c02-0011-44` On 22 Apr 2022 Danilov said only 58 of the Moskva's 510 crew were rescued. *(claimant: Oleksiy Danilov; `ukranews-2022-danilov-58-rescued`)*
  - *Resolution:* Russian MoD vs Ukrainian NSDC claims; each side has motives.
- **explained** · strength 0.8 · 2 judged pairs — Moskva dead: 1 vs about 40.
  - `c02-0009-08` The Russian MoD said in April 2022 that the Moskva sinking left 1 dead, 27 missing and 396 rescued. *(claimant: Russian Ministry of Defence; `wikipedia-sinking-of-the-moskva`)*
  - `c02-0009-12` A Moskva conscript's mother told Novaya Gazeta Europe that about 40 of the crew died. *(claimant: Relatives of Russian servicemen (unnamed); `novayagazetaeu-2022-moskva-injured`)*
  - *Resolution:* The MoD lists many as missing; the higher figure is hearsay from a relative.
- **explained** · strength 0.8 · 2 judged pairs — Moskva dead: 37 vs 1.
  - `c02-0009-11` A Meduza source put the Moskva losses at 37 dead and about 100 injured. *(claimant: Russian sources (unnamed); `meduza-2022-moskva-37-dead`)*
  - `c02-0034-07` On 22 Apr 2022 the Russian MoD gave the Moskva losses as 1 dead, 27 missing and 396 rescued. *(claimant: Russian Ministry of Defence; `wikipedia-sinking-of-the-moskva`)*
  - *Resolution:* An unnamed Russian source vs the MoD's official figure, which likely understates by listing many as missing.
- **explained** · strength 0.7 — Ukrainian dead in the May 2022 Zmiinyi assault: about 10 vs more than 50.
  - `c02-0009-22` Ukraine lost about 10 men and a boat in the May 2022 assault on Zmiinyi Island. *(claimant: —; `pravda-2022-romaniuk-zmiinyi-battle`)*
  - `c02-0034-12` The Russian MoD claimed it repelled the May 2022 assault on Zmiinyi Island and killed "more than 50" paratroopers. *(claimant: Russian Ministry of Defence; `pravda-2022-romaniuk-zmiinyi-battle`)*
  - *Resolution:* Ukrainian reporting vs a Russian MoD claim that is likely inflated.
- **explained** · strength 0.6 — Moskva casualties: 20 dead and 24 injured vs 37 dead and about 100 injured.
  - `c02-0009-10` A Moscow military court in Jan 2026 found that the Moskva sinking left 20 dead, 24 injured and 8 missing. *(claimant: Moscow military court; `rbcukraine-2026-court-moskva`)*
  - `c02-0009-11` A Meduza source put the Moskva losses at 37 dead and about 100 injured. *(claimant: Russian sources (unnamed); `meduza-2022-moskva-37-dead`)*
  - *Resolution:* A Russian court finding vs an unnamed Russian source; the official tally likely understates.
- **explained** · strength 0.6 — Moskva crew size: 485 vs 510.
  - `c02-0023-04` Anušauskas put the Moskva's crew at 485, including 66 officers. *(claimant: Arvydas Anušauskas; `wikipedia-sinking-of-the-moskva`)*
  - `c02-0023-05` The US put the Moskva's crew at 510. *(claimant: US officials (unnamed); `nbc-2022-dilanian-us-intel-moskva`)*
  - *Resolution:* Different sources (Lithuanian vs US); nominal complement vs crew embarked.
- **explained** · strength 0.5 — Moskva dead: 17 declared vs 37.
  - `c02-0009-09` A Sevastopol court in Nov 2022 declared 17 Moskva sailors dead. *(claimant: Sevastopol court; `meduza-2022-moskva-17-declared-dead`)*
  - `c02-0009-11` A Meduza source put the Moskva losses at 37 dead and about 100 injured. *(claimant: Russian sources (unnamed); `meduza-2022-moskva-37-dead`)*
  - *Resolution:* The court declared dead only part of the toll; Meduza's source may include the missing.
- **explained** · strength 0.5 — Moskva dead: 20 (court) vs about 40 (relative).
  - `c02-0009-10` A Moscow military court in Jan 2026 found that the Moskva sinking left 20 dead, 24 injured and 8 missing. *(claimant: Moscow military court; `rbcukraine-2026-court-moskva`)*
  - `c02-0009-12` A Moskva conscript's mother told Novaya Gazeta Europe that about 40 of the crew died. *(claimant: Relatives of Russian servicemen (unnamed); `novayagazetaeu-2022-moskva-injured`)*
  - *Resolution:* An official court finding vs hearsay from a relative.
- **explained** · strength 0.5 — Moskva dead: 17 declared vs about 40.
  - `c02-0009-09` A Sevastopol court in Nov 2022 declared 17 Moskva sailors dead. *(claimant: Sevastopol court; `meduza-2022-moskva-17-declared-dead`)*
  - `c02-0009-12` A Moskva conscript's mother told Novaya Gazeta Europe that about 40 of the crew died. *(claimant: Relatives of Russian servicemen (unnamed); `novayagazetaeu-2022-moskva-injured`)*
  - *Resolution:* The court declared dead only some of the missing; the higher figure is hearsay from a relative.

## c03 Chapter: Azovstal

- **explained** · strength 0.6 · 2 judged pairs — Mariupol garrison size: up to 4,400 vs more than 8,000.
  - `c03-0022-02` Wikipedia extrapolates the Mariupol garrison to up to 4,400 in March 2022. *(claimant: Wikipedia editors; `wikipedia-siege-of-mariupol`)*
  - `c03-0035-03` Shoigu claimed more than 8,000 Ukrainian troops were in Mariupol at the time of encirclement. *(claimant: Sergei Shoigu; `wikipedia-siege-of-mariupol`)*
  - *Resolution:* Wikipedia's extrapolation vs a Shoigu claim that is likely inflated.

## c04 Chapter: Kherson and the Antonivskyi bridge

- **explained** · strength 0.5 — Kherson civilian movement in 2022: 50,000–60,000 vs at least 70,000.
  - `c04-0041-11` Occupation officials put the October–November 2022 civilian movement out of Kherson at 50,000–60,000 people. *(claimant: Russian occupation officials; `rferl-2022-krutov-kherson-ferries`)*
  - `c04-0041-12` Later reports say at least 70,000 civilians were moved from the Kherson right bank in 2022. *(claimant: —; `wikipedia-liberation-of-kherson`)*
  - *Resolution:* The occupation officials' interim figure vs a later, fuller count; the scope may differ slightly (city vs whole right bank).

## c05 Chapter: The Kerch bridge

- **explained** · strength 0.8 · 4 judged pairs — Rival accounts of the Act 1 charge: 21 t TNT-equivalent RDX vs about 10 t of rocket fuel.
  - `c05-0023-02` The SBU's Act 1 means were one truck carrying RDX cylinders of 21 t TNT equivalent hidden in film rolls, and a way around the Crimean Bridge's GPS jammers; Malyuk says no foreign partners were involved. *(claimant: Vasyl Malyuk; `militarnyi-2023-kushnikov-malyuk-details`)*
  - `c05-0041-03` In the Russian version of Act 1, the charge was about 10 t TNT equivalent of solid rocket fuel in 22.7 t of film reels, triggered by GPS at km 156. *(claimant: Russian investigators; `pravda-2024-kommersant-rocket-fuel`)*
  - *Resolution:* SBU account vs Russian investigators' case file.

## c08 Chapter: Vuhledar

- **explained** · strength 0.6 · 2 judged pairs — Size of the 155th Brigade: about 5,000 vs about 2,000 at full strength.
  - `c08-0007-19` Ukraine claimed the 155th lost "almost the entire brigade" of about 5,000 men at Vuhledar, at 150–300 marines a day. *(claimant: Ukrainian forces (unnamed); `politico-2023-melkozerova-155th-brigade`)*
  - `c08-0023-04` Kyiv Post put the 155th Naval Infantry Brigade at about 2,000 at full strength. *(claimant: Kyiv Post; `kyivpost-2023-korshak-vuhledar-marines`)*
  - *Resolution:* A Ukrainian claim that may count reinforcements over time vs Kyiv Post's full-strength estimate.

## c09 Chapter: Robotyne breach

- **explained** · strength 0.7 — Ukrainian and Western sources said the Surovikin line was cut near Verbove; a Kremlin-affiliated milblogger insisted it was unbroken.
  - `c09-0006-16` Al Jazeera reported Ukrainian and Western sources saying the "Surovikin Line" had been cut near Verbove in September 2023. *(claimant: —; `aljazeera-2023-psaropoulos-surovikin-line`)*
  - `c09-0050-18` In September 2023 a Kremlin-affiliated milblogger insisted the "Surovikin" line west of Verbove was unbroken. *(claimant: Russian milbloggers; `isw-2023-09-22-roca`)*
  - *Resolution:* Opposing partisan claimants; ISW's 23 September assessment sided with the breach.
- **explained** · strength 0.5 — One Oryx report as of 9 June 2023 logs 3 Leopard 2A6 lost near Mala Tokmachka, another logs 2 Leopards lost.
  - `c09-0007-13` Oryx, as reported on 9 June 2023, logged 16 Bradleys and 3 Leopard 2A6 lost in the Mala Tokmachka area, of which 5 Bradleys and 1 Leopard were destroyed. *(claimant: Oryx; `popularmechanics-2023-roblin-minefield-losses`)*
  - `c09-0010-26` Oryx logged 2 Leopards lost (1 destroyed, 1 abandoned) as of 9 June 2023. *(claimant: Oryx; `euromaidan-2023-shandra-leopards-mala-tokmachka`)*
  - *Resolution:* Oryx's list was updated during 9 June; the two reports likely use different snapshots.

## c10 Chapter: Krynky bridgehead

- **explained** · strength 0.4 — Russian assault rate at Krynky in July 2024: 7–8 a day vs 2–4 (sometimes a dozen).
  - `c10-0045-18` A Ukrainian platoon commander said Russia attacked at Krynky seven to eight times a day, with groups that shrank from 6–7 men to 3–4, including VDV and Spetsnaz. *(claimant: Ukrainian commanders (unnamed); `isw-2024-07-18-roca`)*
  - `c10-0045-19` Lykhovii said that by late July 2024 Russia made 2–4 assaults a day near Krynky, sometimes a dozen. *(claimant: Dmytro Lykhovii; `pravda-2024-lykhovii-krynky-destroyed`)*
  - *Resolution:* Different Ukrainian observers, sectors and weeks within July 2024.

## c11 Chapter: Avdiivka

- **explained** · strength 0.5 · 2 judged pairs — For Avdiivka in February 2024, Foka gives 1:10 odds in artillery and ammunition while Ukrainian troops report a 5:1 shell advantage.
  - `c11-0024-04` Foka of the 3rd Assault Brigade said four Russian brigades rotated at Avdiivka in the final weeks with two in reserve, at odds of 1:12 in infantry and 1:10 in artillery and ammunition. *(claimant: Foka; `pravda-2024-kyrylenko-last-days-avdiivka`)*
  - `c11-0032-19` Ukrainian troops at Avdiivka reported a 5:1 Russian shell advantage in February 2024 and 8–9:1 at worst in late November 2023. *(claimant: Ukrainian forces (unnamed); `wikipedia-battle-of-avdiivka`)*
  - *Resolution:* Foka describes the final weeks before withdrawal; the 5:1 figure is from early February, and different units and sources report.
- **explained** · strength 0.7 — Tarnavskyi claimed 364 tanks, 748 AFVs and 248 artillery lost at Avdiivka, while Morozov put Russian lost equipment at 300 units.
  - `c11-0007-11` Tarnavskyi claimed Russian losses at Avdiivka of over 47,000 personnel, 364 tanks, 748 AFVs, 248 artillery systems and 5 aircraft. *(claimant: Oleksandr Tarnavskyi; `isw-2024-02-18-roca`)*
  - `c11-0007-14` The Russian milblogger Andrei 'Murz' Morozov put Russian irrecoverable losses at Avdiivka at 16,000 and lost equipment at 300 units. *(claimant: Andrei Morozov; `theinsider-2024-meat-grinder-avdiivka`, `isw-2024-02-21-roca`)*
  - *Resolution:* Ukrainian command claim versus a Russian milblogger's count of irrecoverable losses; both have an incentive to skew.
- **explained** · strength 0.7 · 2 judged pairs — The same Avdiivka pipe is described as a mostly empty 1.3–1.4 m service-water main versus a flooded 0.5 m drainage pipe.
  - `c11-0016-13` The Avdiivka pipe is the main service-water pipe that fed the Donetsk filtration station, about 1.3–1.4 m high, and was mostly empty because the city's water had been cut since late 2021. *(claimant: —; `pravda-2024-kyrylenko-last-days-avdiivka`)*
  - `c11-0016-14` Russian sources describe the Avdiivka pipe as a flooded 0.5 m drainage pipe and a 2 km infiltration. *(claimant: Russian sources (unnamed); `wikipedia-battle-of-avdiivka`)*
  - *Resolution:* The Russian accounts are unverified and may dramatise the infiltration or describe a different conduit; the technical description is better sourced.
- **explained** · strength 0.7 · 2 judged pairs — The same Avdiivka pipe is described as an empty 1.3–1.4 m service-water main versus a flooded 0.5 m drainage pipe.
  - `c11-0010-60` About 15 January 2024 Russian assault groups used the empty 1.3–1.4 m service-water pipe to reach the rear of Avdiivka's southern positions. *(claimant: —; `pravda-2024-kyrylenko-last-days-avdiivka`)*
  - `c11-0016-14` Russian sources describe the Avdiivka pipe as a flooded 0.5 m drainage pipe and a 2 km infiltration. *(claimant: Russian sources (unnamed); `wikipedia-battle-of-avdiivka`)*
  - *Resolution:* The Russian accounts are unverified and may dramatise the infiltration or describe a different conduit; the Ukrainian-side technical description is better sourced.
- **explained** · strength 0.6 · 2 judged pairs — Ukrainian officials privately estimated about 100 Ukrainians captured at Avdiivka, while Mordvichev claimed about 200 surrendered.
  - `c11-0007-22` US officials told the Washington Post that Ukrainian officials privately estimated about 100 Ukrainians captured at Avdiivka (via LWJ). *(claimant: US officials (unnamed); `lwj-2024-hardie-avdiivka-analysis`)*
  - `c11-0007-23` Mordvichev claimed about 200 Ukrainians surrendered at Avdiivka, with 100 more expected. *(claimant: Andrei Mordvichev; `lwj-2024-hardie-avdiivka-analysis`)*
  - *Resolution:* A Russian commander's claim versus a private Ukrainian estimate; each side has an incentive to skew.
- **explained** · strength 0.6 — Morozov's 300 units of lost equipment is less than half of Naalsio's 690 visually confirmed Russian vehicles at Avdiivka.
  - `c11-0007-14` The Russian milblogger Andrei 'Murz' Morozov put Russian irrecoverable losses at Avdiivka at 16,000 and lost equipment at 300 units. *(claimant: Andrei Morozov; `theinsider-2024-meat-grinder-avdiivka`, `isw-2024-02-21-roca`)*
  - `c11-0007-18` Naalsio visually confirmed 690 Russian vehicles lost at Avdiivka by 23 February 2024: 465 destroyed, 197 abandoned and 28 damaged. *(claimant: Naalsio; `theinsider-2024-meat-grinder-avdiivka`)*
  - *Resolution:* Morozov may count only irrecoverable armour or use an earlier cut-off; Naalsio includes abandoned and damaged vehicles.
- **explained** · strength 0.5 — Ukrainian sources give different daily glide-bomb counts for Avdiivka in February 2024: about 60 versus 80–110.
  - `c11-0006-15` Ukrainian units reported about 60 Russian glide bombs a day on Avdiivka by mid-February 2024. *(claimant: Ukrainian forces (unnamed); `isw-2024-02-17-assessment`)*
  - `c11-0032-12` Foka put Russian glide-bomb use at Avdiivka at 80–110 a day. *(claimant: Foka; `pravda-2024-kyrylenko-last-days-avdiivka`)*
  - *Resolution:* Different observers and days: unit reports of a typical daily rate versus Foka's figure, possibly a peak or a wider sector.
- **explained** · strength 0.5 — Murz put lost Russian equipment at Avdiivka at 300 units, while Naalsio had visually counted 574 Russian vehicles lost by 27 January 2024.
  - `c11-0007-14` The Russian milblogger Andrei 'Murz' Morozov put Russian irrecoverable losses at Avdiivka at 16,000 and lost equipment at 300 units. *(claimant: Andrei Morozov; `theinsider-2024-meat-grinder-avdiivka`, `isw-2024-02-21-roca`)*
  - `c11-0010-67` On 27 January 2024 Naalsio counted 574 Russian vehicles lost at Avdiivka against 44 Ukrainian. *(claimant: Naalsio; `kyivpost-2024-york-graveyard-armour`)*
  - *Resolution:* Murz's is an informal milblogger figure, likely of irrecoverable armour only; Naalsio counts every visually confirmed vehicle loss, damaged ones included.
- **explained** · strength 0.5 — A US estimate gives 13,000 Russian killed and wounded at Avdiivka in October–December 2023, while Shtupun implies about 20,000 on the Avdiivka axis in two months.
  - `c11-0007-16` A US estimate put Russian killed and wounded at Avdiivka at 13,000 for October–December 2023 alone. *(claimant: US intelligence; `theinsider-2024-meat-grinder-avdiivka`)*
  - `c11-0010-57` On 20 December 2023 Shtupun said Russia had gained 1.5–2 km in two months for about 25,000 casualties, 80% of them on the Avdiivka axis. *(claimant: Oleksandr Shtupun; `kyivindependent-2023-fornusek-2km-20000`)*
  - *Resolution:* Ukrainian command claim versus a US intelligence estimate; slightly different periods.
- **explained** · strength 0.5 — The same brigade gives Russian odds at Avdiivka of 1:12 and 7:1 for the same final weeks.
  - `c11-0024-04` Foka of the 3rd Assault Brigade said four Russian brigades rotated at Avdiivka in the final weeks with two in reserve, at odds of 1:12 in infantry and 1:10 in artillery and ammunition. *(claimant: Foka; `pravda-2024-kyrylenko-last-days-avdiivka`)*
  - `c11-0024-05` 3rd Assault Brigade statements put the odds at Avdiivka at 7:1. *(claimant: 3rd Assault Brigade; `kyivpost-2024-korshak-zenit-withdrawal`)*
  - *Resolution:* Foka's 1:12 refers to infantry in the final weeks; the 7:1 may be an overall or earlier force ratio.

## c12 Chapter: Hunting the A-50

- **explained** · strength 0.7 — Dead in the 14 Jan 2024 A-50U: 11 vs about 15.
  - `c12-0007-02` The Aviation Safety Network (ASN) records 11 dead in the A-50U shot down on 14 January 2024. *(claimant: ASN; `asn-2024-a50u-rf93966`)*
  - `c12-0007-04` A US officer, as reported, gave about 15 dead in the A-50U shot down on 14 January 2024. *(claimant: US officials (unnamed); `twz-2024-newdick-patriot-a50-confirmed`)*
  - *Resolution:* ASN record vs an unnamed US officer; the crew size is uncertain.
- **explained** · strength 0.6 — Operational A-50s in Jan 2024: 3 vs 8.
  - `c12-0007-11` Ukraine's Southern Command said Russia had 3 A-50s in service out of 6 before the January 2024 strike. *(claimant: Operational Command South; `isw-2024-01-15-assessment`)*
  - `c12-0007-12` UK Defence Intelligence said Russia had 8 operational A-50s in January 2024. *(claimant: UK Defence Intelligence; `pravda-2024-ukdi-a50-significance`)*
  - *Resolution:* Ukrainian Southern Command vs UK Defence Intelligence; they may differ on what counts as in service.
- **explained** · strength 0.6 — Dead in the 14 Jan 2024 A-50U: 11–12 vs about 15.
  - `c12-0007-03` The Russian blogger FighterBomber gives 11–12 dead in the A-50U shot down on 14 January 2024. *(claimant: FighterBomber; `wikipedia-2024-a50-il22-shootdowns`)*
  - `c12-0007-04` A US officer, as reported, gave about 15 dead in the A-50U shot down on 14 January 2024. *(claimant: US officials (unnamed); `twz-2024-newdick-patriot-a50-confirmed`)*
  - *Resolution:* A Russian blogger vs an unnamed US officer; the crew size is uncertain.
- **explained** · strength 0.5 — Serviceable A-50s in Jan 2024: 3 vs 8.
  - `c12-0007-11` Ukraine's Southern Command said Russia had 3 A-50s in service out of 6 before the January 2024 strike. *(claimant: Operational Command South; `isw-2024-01-15-assessment`)*
  - `c12-0007-13` Budanov said Russia had "just eight A-50s in good condition". *(claimant: Kyrylo Budanov; `euromaidanpress-2024-zoria-a50-azov`)*
  - *Resolution:* Two Ukrainian sources with different definitions (in service vs in good condition).
- **explained** · strength 0.5 — A-50 crew size: 15–16 vs 19.
  - `c12-0020-03` Militarnyi gives the A-50's crew as 15–16. *(claimant: Militarnyi; `militarnyi-2024-a50-lost-eyes`)*
  - `c12-0020-05` Ukrainska Pravda gives the A-50's crew as 19: 5 pilots, 11 radio engineers and 3 technical engineers. *(claimant: —; `pravda-2024-balachuk-a50-azov`)*
  - *Resolution:* Outlets differ on the nominal crew; mission crews vary with configuration.
- **explained** · strength 0.5 — Distance of the 23 Feb 2024 A-50 kill: about 220 km vs about 300 km.
  - `c12-0015-17` The War Zone put the 23 February 2024 A-50 kill about 220 km from the nearest Ukrainian positions. *(claimant: —; `twz-2024-newdick-second-a50`)*
  - `c12-0015-18` The Russian court, as reported, put the 23 February 2024 A-50 about 300 km from Ukraine. *(claimant: Moscow court; `kyivpost-2024-dzyaman-court`)*
  - *Resolution:* TWZ measures from the nearest Ukrainian positions; the Russian court may have an interest in exaggerating.
- **explained** · strength 0.5 — Distance of the 23 Feb 2024 A-50 kill: about 170 km from the front vs about 300 km from Ukraine.
  - `c12-0015-15` The Times, as summarised, put the 23 February 2024 A-50 kill about 170 km from the front. *(claimant: —; `babel-2024-perepechko-times-a50`)*
  - `c12-0015-18` The Russian court, as reported, put the 23 February 2024 A-50 about 300 km from Ukraine. *(claimant: Moscow court; `kyivpost-2024-dzyaman-court`)*
  - *Resolution:* Different reference points (front line vs border) and a Russian court's interest in exaggerating the range.
- **explained** · strength 0.5 — A-50 fleet and availability: 3 of 6 vs about 5 of 10.
  - `c12-0007-11` Ukraine's Southern Command said Russia had 3 A-50s in service out of 6 before the January 2024 strike. *(claimant: Operational Command South; `isw-2024-01-15-assessment`)*
  - `c12-0007-16` The War Zone estimates Russia's A-50 fleet at about 10, with about 5 operational at a time. *(claimant: The War Zone; `twz-2024-altman-rogoway-a50-claims`)*
  - *Resolution:* Southern Command may count only modernised active airframes; TWZ estimates the whole fleet.
- **explained** · strength 0.5 — Size of Russia's A-50 fleet: 6 aircraft vs 10.
  - `c12-0007-11` Ukraine's Southern Command said Russia had 3 A-50s in service out of 6 before the January 2024 strike. *(claimant: Operational Command South; `isw-2024-01-15-assessment`)*
  - `c12-0007-15` Militarnyi gives Russia's A-50 fleet as 10 aircraft. *(claimant: Militarnyi; `militarnyi-2024-a50-lost-eyes`)*
  - *Resolution:* Southern Command may count only active modernised airframes; Militarnyi counts all airframes, including stored ones.
- **explained** · strength 0.5 — Operational A-50s in Jan 2024: 8 vs about 5.
  - `c12-0007-12` UK Defence Intelligence said Russia had 8 operational A-50s in January 2024. *(claimant: UK Defence Intelligence; `pravda-2024-ukdi-a50-significance`)*
  - `c12-0007-16` The War Zone estimates Russia's A-50 fleet at about 10, with about 5 operational at a time. *(claimant: The War Zone; `twz-2024-altman-rogoway-a50-claims`)*
  - *Resolution:* UKDI may count airworthy airframes, while TWZ counts those available at any one time.
- **explained** · strength 0.4 — Distance of the 23 Feb 2024 A-50 kill from the front: about 200 km or more vs about 170 km.
  - `c12-0006-02` On 23 February 2024 a second Russian A-50U was brought down over Krasnodar Krai, about 200 km or more from the front. *(claimant: —; `pravda-2024-kravets-a50-s200`, `twz-2024-newdick-second-a50`)*
  - `c12-0015-15` The Times, as summarised, put the 23 February 2024 A-50 kill about 170 km from the front. *(claimant: —; `babel-2024-perepechko-times-a50`)*
  - *Resolution:* Rough estimates with different reference points.
- **explained** · strength 0.4 — Distance of the 23 Feb 2024 A-50 kill: about 170 km from the front vs about 220 km from the nearest Ukrainian positions.
  - `c12-0015-15` The Times, as summarised, put the 23 February 2024 A-50 kill about 170 km from the front. *(claimant: —; `babel-2024-perepechko-times-a50`)*
  - `c12-0015-17` The War Zone put the 23 February 2024 A-50 kill about 220 km from the nearest Ukrainian positions. *(claimant: —; `twz-2024-newdick-second-a50`)*
  - *Resolution:* Rough estimates with slightly different reference points.

## c13 Chapter: The Black Sea drone war

- **explained** · strength 0.5 — 31 Dec 2024 Magura engagement: two Mi-8s destroyed vs one downed and one hit that reached its airfield.
  - `c13-0009-12` On 2 Jan 2025, HUR revised its 31 Dec 2024 claim to two Mi-8s destroyed and a third helicopter damaged. *(claimant: Defence Intelligence of Ukraine (HUR); `eurosd-2025-magura-mi8-kills`)*
  - `c13-0013-77` On 31 Dec 2024 off Cape Tarkhankut, Group 13 Maguras armed with R-73 "SeeDragon" missiles downed an Mi-8 under fire from the helicopters; a second Mi-8 was hit and reached its airfield. *(claimant: —; `isw-2024-12-31-roca`, `aviationist-2024-durso-magura-mi8`, `gur-2024-mi8`)*
  - *Resolution:* HUR's revised claim vs the initial reporting; the second helicopter's fate is uncertain.
- **explained** · strength 0.5 — Magura hits on the Ivanovets: six vs three.
  - `c13-0013-44` The Ivanovets took six Magura hits through AK-630M fire, rolled astern and sank. *(claimant: Kyrylo Budanov; `twz-2024-altman-ivanovets`, `euromaidan-2024-zoria-six-maguras`)*
  - `c13-0044-08` The Russian milblogger VoenkorKotenok confirmed the Ivanovets loss ("three hits from naval drones"). *(claimant: VoenkorKotenok; `twz-2024-altman-ivanovets`)*
  - *Resolution:* Budanov's claim vs a Russian milblogger who may count only the decisive hits; both agree it sank.
- **explained** · strength 0.5 · 2 judged pairs — The same 21-target Magura tally broken down as 16 warships, 3 helicopters and 2 jets vs 9 vessels, 2 Su-30s and 2 Mi-8s.
  - `c13-0010-07` HUR spokesman Yusov said on 15 May 2025 that Maguras had hit 21 targets, including "16 warships at the bottom", 3 helicopters and 2 jets. *(claimant: Andrii Yusov; `ukrinform-2025-magura-v7`)*
  - `c13-0013-86` On 15 May 2025, HUR unveiled the Magura V7 in Kyiv and gave its tallies: 9 vessels, 2 Su-30s and 2 Mi-8s, or 21 targets. *(claimant: Defence Intelligence of Ukraine (HUR); `defenseexpress-2025-magura-v7`, `ukrinform-2025-magura-v7`)*
  - *Resolution:* Two HUR breakdowns on the same day; "16 warships" likely counts small craft, or the tallies were mixed up in reporting.
- **explained** · strength 0.5 — 31 Dec 2024 Magura engagement: two Mi-8s downed vs one downed and one hit that reached its airfield.
  - `c13-0008-04` HUR later claimed that two Mi-8s were downed by its Magura V5s on 31 Dec 2024. *(claimant: Defence Intelligence of Ukraine (HUR); `eurosd-2025-magura-mi8-kills`)*
  - `c13-0013-77` On 31 Dec 2024 off Cape Tarkhankut, Group 13 Maguras armed with R-73 "SeeDragon" missiles downed an Mi-8 under fire from the helicopters; a second Mi-8 was hit and reached its airfield. *(claimant: —; `isw-2024-12-31-roca`, `aviationist-2024-durso-magura-mi8`, `gur-2024-mi8`)*
  - *Resolution:* HUR's later claim vs the initial reporting; the second helicopter's fate is uncertain.
- **explained** · strength 0.4 — Magura V5 top speed: 42 kn vs a 54 kn burst.
  - `c13-0030-11` The Magura V5 cruises at 22 kn with a 42 kn maximum. *(claimant: —; `twz-2024-altman-ivanovets`)*
  - `c13-0030-12` A low-reliability source gives the Magura V5 a "54 kn burst" speed. *(claimant: Georgetown Security Studies Review; `gssr-2026-swarm-at-sea`)*
  - *Resolution:* The 54 kn figure comes from a low-reliability source; it may describe a burst mode or a variant.

## c14 Chapter: The Kursk incursion and the North Korean defence

- **explained** · strength 0.6 · 2 judged pairs — ISW's about 800 km² and Syrskyi's about 1,000 km² for the same date and area are incompatible as stated.
  - `c14-0011-02` As of 12 August 2024 ISW assessed about 800 km² of claimed Ukrainian advance in Kursk Oblast, not all of it controlled. *(claimant: Institute for the Study of War; `isw-2024-08-12-roca`, `aljazeera-2024-duggal-mapping-kursk`)*
  - `c14-0014-030` On 12 August 2024 Syrskyi claimed about 1,000 km² taken in Kursk Oblast. *(claimant: Oleksandr Syrskyi; `guardian-2024-syrskyi-1000-sq-km`)*
  - *Resolution:* Syrskyi gives an official claim; ISW assesses only what it can verify, not all of it controlled.
- **explained** · strength 0.5 · 2 judged pairs — The NIS put North Korean dead at about 600 in April 2025, while a BBC analysis counted 2,304 killed.
  - `c14-0014-129` On 30 April 2025 the NIS estimated about 4,700 North Korean casualties, including 600 dead. *(claimant: National Intelligence Service (South Korea); `euromaidan-2025-tril-nis-4700`)*
  - `c14-0027-08` A BBC analysis, via Wikipedia, counted 2,304 North Koreans killed. *(claimant: BBC News; `wikipedia-2024-kursk-offensive`)*
  - *Resolution:* Different methods and probably different dates; the NIS figure is an early official estimate.
- **explained** · strength 0.4 — Observers saw about a hundred Russian troops emerge from the Sudzha pipeline on 8 March 2025, while Gerasimov claimed over 600 took part.
  - `c14-0007-03` On 8 March 2025 about a hundred Russian troops came out of a disused gas pipeline inside Sudzha. *(claimant: —; `isw-2025-03-09-roca`, `twz-2025-newdick-kursk-pipeline`)*
  - `c14-0014-101` Gerasimov claimed "over 600" Russian troops took part in the 8 March 2025 Sudzha pipeline raid. *(claimant: Valery Gerasimov; `twz-2025-newdick-kursk-pipeline`)*
  - *Resolution:* Gerasimov's figure may count all participants and follow-on forces, and is a self-serving Russian claim; the hundred is those seen exiting.
- **explained** · strength 0.4 — The NIS put North Korean dead at 300 in January 2025, while a BBC analysis counted 2,304 killed.
  - `c14-0014-087` On 13 January 2025 the NIS said 300 North Koreans had been killed and 2,700 wounded. *(claimant: National Intelligence Service (South Korea); `aljazeera-2025-nis-300-killed`)*
  - `c14-0027-08` A BBC analysis, via Wikipedia, counted 2,304 North Koreans killed. *(claimant: BBC News; `wikipedia-2024-kursk-offensive`)*
  - *Resolution:* Probably different dates; the NIS figure is an early official estimate made before later fighting.

## c15 Chapter: Pokrovsk–Myrnohrad

- **explained** · strength 0.7 · 2 judged pairs — One briefing is read as 15,000 total casualties incl. 7,000 dead vs 7,000 dead plus 15,000 wounded or missing (22,000).
  - `c15-0005-28` Syrskyi said about 7,000 Russians were killed and 15,000 became casualties in total on the Pokrovsk axis in January 2025. *(claimant: Oleksandr Syrskyi; `euromaidan-2025-mukhina-chechen-war-losses`)*
  - `c15-0005-29` The Guardian read the same January 2025 briefing as 7,000 Russians killed plus 15,000 wounded or missing on the Pokrovsk axis. *(claimant: Oleksandr Syrskyi; `guardian-2025-harding-pishchane`)*
  - *Resolution:* Different readings of the same Syrskyi January 2025 briefing by Euromaidan and the Guardian; the primary statement decides which is right.

## c16 Chapter: Operation Spiderweb

- **explained** · strength 0.8 — AviVector imagery shows 3 Tu-95MS destroyed at Olenya, while Military Chronicle claims only one lost and one damaged.
  - `c16-0011-76` AviVector imagery of Olenya at 09:55 UTC on 3 June 2025 showed 3 Tu-95MS and 1 An-12 completely destroyed. *(claimant: AviVector; `twz-2025-trevithick-loss-assessments`)*
  - `c16-0047-11` Military Chronicle claimed only one Tu-95MS was lost and one badly damaged at Olenya because the aircraft were empty for maintenance. *(claimant: Military Chronicle; `twz-2025-altman-what-we-know-spiderweb`)*
  - *Resolution:* Military Chronicle is a pro-Russian channel minimising losses; AviVector counts from post-strike satellite imagery, which should be preferred.
- **explained** · strength 0.7 — OSINT's 8 aircraft confirmed destroyed, including 5 Tu-95MS, is incompatible with Military Chronicle's claim that only one Tu-95MS was lost.
  - `c16-0011-56` By 18:00 Kyiv time on 1 June 2025 OSINT confirmed 8 aircraft destroyed: 5 Tu-95MS, 2 Tu-22M3 and 1 An-12. *(claimant: —; `euromaidanpress-2025-zoria-trojan-truck`)*
  - `c16-0047-11` Military Chronicle claimed only one Tu-95MS was lost and one badly damaged at Olenya because the aircraft were empty for maintenance. *(claimant: Military Chronicle; `twz-2025-altman-what-we-know-spiderweb`)*
  - *Resolution:* Military Chronicle is a Russian milblogger channel minimising losses; the OSINT count rests on imagery.
- **explained** · strength 0.7 — Four Tu-95MS burning on the Olenya apron is incompatible with Military Chronicle's claim that only one Tu-95MS was lost there.
  - `c16-0040-08` On the southern apron at Olenya four Tu-95MS and a propeller transport burned in the Spiderweb strike. *(claimant: —; `barentsobserver-2025-chentemirov-olenya-attack`)*
  - `c16-0047-11` Military Chronicle claimed only one Tu-95MS was lost and one badly damaged at Olenya because the aircraft were empty for maintenance. *(claimant: Military Chronicle; `twz-2025-altman-what-we-know-spiderweb`)*
  - *Resolution:* Military Chronicle is a Russian milblogger channel minimising losses; the four burned airframes rest on imagery reporting.
- **explained** · strength 0.7 — For the same aircraft hit in Spiderweb, the NATO official's 40+ (15 Tu-95, 20 Tu-22) is incompatible with the NYT's 6 Tu-95, 4 Tu-22M and 1 A-50 (up to 20).
  - `c16-0007-06` A senior NATO official said at least 40 Russian aircraft were damaged in Operation Spiderweb: 15 Tu-95, 20 Tu-22 and at least one A-50. *(claimant: Western officials (unnamed); `eurointegration-2025-sydorenko-nato-count`, `moscowtimes-2025-kozlov-nato-retaliate`)*
  - `c16-0011-71` On 3 June 2025 the NYT reported 6 Tu-95, 4 Tu-22M and 1 A-50 hit in Operation Spiderweb, and possibly up to 20. *(claimant: The New York Times; `isw-2025-06-03-roca`)*
  - *Resolution:* An unnamed NATO official counted 'damaged' aircraft early; the NYT figure rests on US officials' count of hit aircraft. Later imagery-based counts favour the lower figure.
- **explained** · strength 0.7 — A NATO official said at least 40 aircraft were damaged, while two US officials put those hit at up to about 20.
  - `c16-0007-06` A senior NATO official said at least 40 Russian aircraft were damaged in Operation Spiderweb: 15 Tu-95, 20 Tu-22 and at least one A-50. *(claimant: Western officials (unnamed); `eurointegration-2025-sydorenko-nato-count`, `moscowtimes-2025-kozlov-nato-retaliate`)*
  - `c16-0007-08` Two US officials estimated that up to about 20 Russian aircraft were hit in Operation Spiderweb and about 10 destroyed. *(claimant: US officials (unnamed); `reuters-2025-stewart-ali-us-estimate`)*
  - *Resolution:* Different Western officials' estimates; the NATO count may include lightly damaged aircraft, the US count only significant hits.
- **explained** · strength 0.7 — Ukraine reports drones hitting Ivanovo and Dyagilevo (Ryazan); the Russian MoD said all attacks in Ivanovo and Ryazan were repelled.
  - `c16-0005-03` On 1 June 2025, the eve of the second Istanbul talks, 117 drones hit four Russian bases: Olenya, Belaya, Dyagilevo and Ivanovo. *(claimant: —; `bbc-2025-adams-spiders-web-strike`)*
  - `c16-0047-02` Russia's MoD said all attacks in the Ivanovo, Ryazan and Amur regions were repelled. *(claimant: Russian Ministry of Defence; `twz-2025-altman-rogoway-bombers-destroyed`)*
  - *Resolution:* Russian MoD denial of damage versus Ukrainian and imagery-backed reports of hits; the MoD statement is self-serving.
- **explained** · strength 0.7 — BBC Verify's imagery count of 5 aircraft hit at Olenya conflicts with Military Chronicle's claim of only one lost and one damaged there.
  - `c16-0007-12` BBC Verify counted 12 Russian aircraft damaged or destroyed in Operation Spiderweb: 5 at Olenya and 7 at Belaya. *(claimant: BBC Verify; `bbc-2025-brown-spencer-satellite`)*
  - `c16-0047-11` Military Chronicle claimed only one Tu-95MS was lost and one badly damaged at Olenya because the aircraft were empty for maintenance. *(claimant: Military Chronicle; `twz-2025-altman-what-we-know-spiderweb`)*
  - *Resolution:* Satellite-imagery count versus a pro-Russian blog minimising losses.
- **explained** · strength 0.7 — US officials estimated up to about 20 aircraft hit in Spiderweb, while the SBU claimed 41 bombers hit.
  - `c16-0007-08` Two US officials estimated that up to about 20 Russian aircraft were hit in Operation Spiderweb and about 10 destroyed. *(claimant: US officials (unnamed); `reuters-2025-stewart-ali-us-estimate`)*
  - `c16-0011-48` At 10:52 UTC (13:52 Kyiv) on 1 June 2025 Kyiv Independent published an SBU source saying 41 bombers were hit at four airfields. *(claimant: Security Service of Ukraine (SBU); `kyivindependent-2025-york-sbu-bombers-burning`)*
  - *Resolution:* US officials' estimate from imagery versus the SBU's own claim, which counts every aircraft it says it struck.
- **explained** · strength 0.7 — The SBU claims 41 aircraft hit, while US officials estimated up to about 20 hit.
  - `c16-0007-01` The SBU and the Ukrainian General Staff claim 41 Russian aircraft were hit in Operation Spiderweb. *(claimant: Security Service of Ukraine (SBU); General Staff of the Armed Forces of Ukraine; `kyivindependent-2025-york-spiderweb`, `kyivpost-2025-spiderweb-a50-ivanovo`)*
  - `c16-0007-08` Two US officials estimated that up to about 20 Russian aircraft were hit in Operation Spiderweb and about 10 destroyed. *(claimant: US officials (unnamed); `reuters-2025-stewart-ali-us-estimate`)*
  - *Resolution:* The SBU counts every aircraft it says it struck; US officials' estimate is independent and more conservative.
- **explained** · strength 0.7 — The SBU claims 41 aircraft hit while BBC Verify counted 12 damaged or destroyed.
  - `c16-0007-01` The SBU and the Ukrainian General Staff claim 41 Russian aircraft were hit in Operation Spiderweb. *(claimant: Security Service of Ukraine (SBU); General Staff of the Armed Forces of Ukraine; `kyivindependent-2025-york-spiderweb`, `kyivpost-2025-spiderweb-a50-ivanovo`)*
  - `c16-0007-12` BBC Verify counted 12 Russian aircraft damaged or destroyed in Operation Spiderweb: 5 at Olenya and 7 at Belaya. *(claimant: BBC Verify; `bbc-2025-brown-spencer-satellite`)*
  - *Resolution:* The SBU counts every aircraft it says it hit, including light damage; BBC counts only damage visible in satellite imagery.
- **explained** · strength 0.6 — The War Zone counts with certainty at least 6 Tu-95MS and 4 Tu-22M3 destroyed, while Military Chronicle claims only one Tu-95MS was lost.
  - `c16-0007-10` The War Zone says "with certainty" that at least 6, probably 7, Tu-95MS and 4 Tu-22M3 were destroyed in Operation Spiderweb. *(claimant: The War Zone; `twz-2025-newdick-confirmed-losses`)*
  - `c16-0047-11` Military Chronicle claimed only one Tu-95MS was lost and one badly damaged at Olenya because the aircraft were empty for maintenance. *(claimant: Military Chronicle; `twz-2025-altman-what-we-know-spiderweb`)*
  - *Resolution:* A pro-Russian blog minimising losses, against an imagery-based count.
- **explained** · strength 0.6 — A NATO official's at least 40 aircraft damaged in Spiderweb is incompatible with BBC Verify's count of 12 damaged or destroyed.
  - `c16-0007-06` A senior NATO official said at least 40 Russian aircraft were damaged in Operation Spiderweb: 15 Tu-95, 20 Tu-22 and at least one A-50. *(claimant: Western officials (unnamed); `eurointegration-2025-sydorenko-nato-count`, `moscowtimes-2025-kozlov-nato-retaliate`)*
  - `c16-0007-12` BBC Verify counted 12 Russian aircraft damaged or destroyed in Operation Spiderweb: 5 at Olenya and 7 at Belaya. *(claimant: BBC Verify; `bbc-2025-brown-spencer-satellite`)*
  - *Resolution:* BBC Verify counts only aircraft confirmed on imagery at Olenya and Belaya; the NATO estimate covers all airfields and likely includes lighter damage not visible from above.
- **explained** · strength 0.6 — Frontelligence's 11 bombers and 1 An-12 hit and the General Staff's re-affirmed 41 aircraft hit on the same day cannot both be the Spiderweb total.
  - `c16-0011-72` On 3 June 2025 Frontelligence counted 11 bombers and 1 An-12 hit in Operation Spiderweb. *(claimant: Frontelligence Insight; `isw-2025-06-03-roca`)*
  - `c16-0011-74` At 16:00 on 3 June 2025 the Ukrainian General Staff re-affirmed 41 aircraft hit in Operation Spiderweb. *(claimant: General Staff of the Armed Forces of Ukraine; `kyivpost-2025-spiderweb-a50-ivanovo`)*
  - *Resolution:* Frontelligence counts only aircraft confirmed on satellite imagery; the General Staff claim includes damage that imagery cannot confirm.
- **explained** · strength 0.6 — On 4 June 2025 a NATO official said at least 40 aircraft were damaged, while two US officials said about 20 were hit.
  - `c16-0011-82` On 4 June 2025 a NATO official said at least 40 Russian aircraft were damaged and 10–13 destroyed in Operation Spiderweb. *(claimant: Western officials (unnamed); `eurointegration-2025-sydorenko-nato-count`)*
  - `c16-0011-83` On 4 June 2025 two US officials said about 20 Russian aircraft were hit and about 10 destroyed in Operation Spiderweb. *(claimant: US officials (unnamed); `reuters-2025-stewart-ali-us-estimate`)*
  - *Resolution:* The two agree on about 10 destroyed; the NATO count probably includes lightly damaged aircraft.
- **explained** · strength 0.6 — The NYT's 11 aircraft hit, possibly up to 20, and the General Staff's re-affirmed 41 hit on the same day cannot both be the Spiderweb total.
  - `c16-0011-71` On 3 June 2025 the NYT reported 6 Tu-95, 4 Tu-22M and 1 A-50 hit in Operation Spiderweb, and possibly up to 20. *(claimant: The New York Times; `isw-2025-06-03-roca`)*
  - `c16-0011-74` At 16:00 on 3 June 2025 the Ukrainian General Staff re-affirmed 41 aircraft hit in Operation Spiderweb. *(claimant: General Staff of the Armed Forces of Ukraine; `kyivpost-2025-spiderweb-a50-ivanovo`)*
  - *Resolution:* The NYT relies on US officials and imagery assessments; the General Staff claim includes damage that cannot be verified.
- **explained** · strength 0.6 — Military Chronicle claimed only one Tu-95MS lost at Olenya, while the "other sources" it relayed gave 8 Tu-95MS lost.
  - `c16-0047-11` Military Chronicle claimed only one Tu-95MS was lost and one badly damaged at Olenya because the aircraft were empty for maintenance. *(claimant: Military Chronicle; `twz-2025-altman-what-we-know-spiderweb`)*
  - `c16-0047-12` Military Chronicle also relayed "other sources" giving 8 Tu-95MS lost. *(claimant: Military Chronicle; `twz-2025-altman-what-we-know-spiderweb`)*
  - *Resolution:* The one-loss figure is a minimising Russian milblogger line about Olenya; the eight comes from unnamed other sources and may cover all bases.
- **explained** · strength 0.6 — The historical floor of about 10 heavy bombers destroyed conflicts with Military Chronicle's claim that only one Tu-95MS was lost.
  - `c16-0008-01` The historical floor for Operation Spiderweb is at least about 10 heavy bombers (Tu-95MS and Tu-22M3) destroyed at Olenya and Belaya, plus one An-12. *(claimant: —; `twz-2025-newdick-confirmed-losses`, `newsweek-2025-cook-tu160-anadyr`)*
  - `c16-0047-11` Military Chronicle claimed only one Tu-95MS was lost and one badly damaged at Olenya because the aircraft were empty for maintenance. *(claimant: Military Chronicle; `twz-2025-altman-what-we-know-spiderweb`)*
  - *Resolution:* A pro-Russian blog minimising losses, against imagery-based counts by several analysts.
- **explained** · strength 0.5 — Bronk counts at least 6 Tu-95MS destroyed, while Military Chronicle claims only one Tu-95MS was lost at Olenya, where most were based.
  - `c16-0007-11` RUSI's Justin Bronk counts at least 6 Tu-95MS and 4 Tu-22M3 destroyed in Operation Spiderweb. *(claimant: Justin Bronk; `newsweek-2025-cook-tu160-anadyr`)*
  - `c16-0047-11` Military Chronicle claimed only one Tu-95MS was lost and one badly damaged at Olenya because the aircraft were empty for maintenance. *(claimant: Military Chronicle; `twz-2025-altman-what-we-know-spiderweb`)*
  - *Resolution:* Pro-Russian blog minimising losses; its claim covers only Olenya, not Belaya.
- **explained** · strength 0.5 — BBC says 117 drones hit the bases while Butusov says 116 became airborne.
  - `c16-0005-03` On 1 June 2025, the eve of the second Istanbul talks, 117 drones hit four Russian bases: Olenya, Belaya, Dyagilevo and Ivanovo. *(claimant: —; `bbc-2025-adams-spiders-web-strike`)*
  - `c16-0006-02` Journalist Yurii Butusov says 150 drones and 300 munitions were smuggled into Russia for Operation Spiderweb and 116 drones became airborne. *(claimant: Yurii Butusov; `texty-2025-butusov-pavutyna-preparation`)*
  - *Resolution:* Minor count difference; the official 117 may include a drone that did not take off.
- **explained** · strength 0.5 — AviVector counted three Tu-95MS destroyed at Olenya; the report says four burned.
  - `c16-0011-76` AviVector imagery of Olenya at 09:55 UTC on 3 June 2025 showed 3 Tu-95MS and 1 An-12 completely destroyed. *(claimant: AviVector; `twz-2025-trevithick-loss-assessments`)*
  - `c16-0040-08` On the southern apron at Olenya four Tu-95MS and a propeller transport burned in the Spiderweb strike. *(claimant: —; `barentsobserver-2025-chentemirov-olenya-attack`)*
  - *Resolution:* Imagery counts only completely destroyed airframes; the press report counts all aircraft seen burning, some only damaged.
- **explained** · strength 0.5 — Butusov says 116 drones were airborne while Zelensky says 117 were used.
  - `c16-0006-02` Journalist Yurii Butusov says 150 drones and 300 munitions were smuggled into Russia for Operation Spiderweb and 116 drones became airborne. *(claimant: Yurii Butusov; `texty-2025-butusov-pavutyna-preparation`)*
  - `c16-0011-59` On the evening of 1 June 2025 Zelensky said 117 drones were used in Operation Spiderweb, each with its own pilot. *(claimant: Volodymyr Zelensky; `bbc-2025-adams-spiders-web-strike`)*
  - *Resolution:* Minor count difference; 117 may include a drone that did not take off.
- **explained** · strength 0.5 — BBC reports 117 drones launched in Spiderweb while Butusov says 116 became airborne.
  - `c16-0006-01` 117 FPV drones were launched in Operation Spiderweb on 1 June 2025. *(claimant: —; `bbc-2025-adams-spiders-web-strike`)*
  - `c16-0026-13` Butusov says 150 drones were smuggled into Russia for Operation Spiderweb and 116 launched. *(claimant: Yurii Butusov; `texty-2025-butusov-pavutyna-preparation`)*
  - *Resolution:* Minor count difference; one drone may have failed to launch, or sources count differently.
- **explained** · strength 0.4 — OSINT confirmed 5 Tu-95MS destroyed by 18:00 on 1 June, while Military Chronicle relayed other sources claiming 8 Tu-95MS lost.
  - `c16-0011-56` By 18:00 Kyiv time on 1 June 2025 OSINT confirmed 8 aircraft destroyed: 5 Tu-95MS, 2 Tu-22M3 and 1 An-12. *(claimant: —; `euromaidanpress-2025-zoria-trojan-truck`)*
  - `c16-0047-12` Military Chronicle also relayed "other sources" giving 8 Tu-95MS lost. *(claimant: Military Chronicle; `twz-2025-altman-what-we-know-spiderweb`)*
  - *Resolution:* Unverified relayed claim versus imagery-confirmed count; possibly 8 aircraft of all types conflated with Tu-95MS.
- **explained** · strength 0.4 — Bronk counts at least 6 Tu-95MS destroyed, while Military Chronicle relayed other sources giving 8 Tu-95MS lost.
  - `c16-0007-11` RUSI's Justin Bronk counts at least 6 Tu-95MS and 4 Tu-22M3 destroyed in Operation Spiderweb. *(claimant: Justin Bronk; `newsweek-2025-cook-tu160-anadyr`)*
  - `c16-0047-12` Military Chronicle also relayed "other sources" giving 8 Tu-95MS lost. *(claimant: Military Chronicle; `twz-2025-altman-what-we-know-spiderweb`)*
  - *Resolution:* An unverified relayed claim, possibly conflating 8 aircraft of all types with Tu-95MS; Bronk's figure is a floor.

## c17 Chapter: The Kupiansk gas-pipeline infiltration

- **explained** · strength 0.8 — Same Sudzha-pipe force size, about 100 vs over 600.
  - `c17-0022-10` Ukrainska Pravda, via TWZ, put the Russian force through the Sudzha pipe at about 100. *(claimant: —; `twz-2025-newdick-kursk-pipeline`)*
  - `c17-0022-11` Gerasimov claimed 'over 600' Russian troops went through the Sudzha pipe. *(claimant: Valery Gerasimov; `twz-2025-newdick-kursk-pipeline`)*
  - *Resolution:* Ukrainian reporting vs Gerasimov's claim; the Russian figure is likely inflated for propaganda.
- **explained** · strength 0.6 — Avdiivka infiltration pipe length: nearly 2 km vs 3.7 km.
  - `c17-0022-02` Pro-Russian claims say the Avdiivka drainage and sewer pipe was 0.8 m wide and nearly 2 km long. *(claimant: Russian sources (unnamed); `kyivpost-2024-korshak-avdiivka-pipe`)*
  - `c17-0022-03` United24 says the Avdiivka infiltration pipe was 3.7 km long and up to 1.4 m wide. *(claimant: —; `united24-2026-kabachynskyi-pipelines`)*
  - *Resolution:* Pro-Russian claims vs United24; possibly different end points or sections.
- **explained** · strength 0.4 — Russians in Kupiansk in Dec 2025: about 500 with 100 trapped vs 300 falling to about 50.
  - `c17-0005-15` The Achilles regiment said about 500 Russians had been in Kupiansk earlier, with about 100 trapped. *(claimant: Achilles; `defenseexpress-2025-achilles-pipeline-destroyed`)*
  - `c17-0005-16` A December 2025 frontline report put the Russians in Kupiansk at 300, falling to about 50. *(claimant: —; `euromaidanpress-2025-frontline-kupiansk-pipeline-killzone`)*
  - *Resolution:* Different units' estimates at different points in December.

## c18 Chapter: One night over Kyiv

- **explained** · strength 0.5 — Ihnat's 28 ballistic missiles at Kyiv exceeds the Air Force's total of 24 Iskander-M/S-400 ballistic missiles for the whole attack.
  - `c18-0004-03` 28 ballistic missiles flew at Kyiv on 1–2 July 2026, the most ever fired in one attack on the capital, according to Air Force spokesman Ihnat. *(claimant: Yurii Ihnat; `kyivindependent-2026-fenbert-kyiv-attack-31-killed`)*
  - `c18-0008-03` On 1–2 July 2026 Russia launched 24 Iskander-M/S-400 ballistic missiles from Bryansk and Kursk oblasts, according to the Ukrainian Air Force. *(claimant: Ukrainian Air Force; `pravda-2026-romanenko-570-targets`, `isw-2026-07-02-roca`)*
  - *Resolution:* Ihnat likely counted the 4 Zircon or other aeroballistic missiles as ballistic.
- **explained** · strength 0.5 — For the same night, the Air Force counts 74 missiles launched and the Defence Ministry 77.
  - `c18-0008-11` In total Russia launched 74 missiles on 1–2 July 2026, of which the Ukrainian Air Force says 48 were downed or suppressed (65%, our arithmetic). *(claimant: Ukrainian Air Force; `isw-2026-07-02-roca`, `pravda-2026-romanenko-570-targets`)*
  - `c18-0010-05` Ukraine's Defence Ministry gave rounder figures for 1–2 July 2026: "almost 500" drones and 77 missiles, 25 of them ballistic or hypersonic. *(claimant: Ministry of Defence of Ukraine; `pravda-2026-bondarieva-fedorov-patriot-appeal`)*
  - *Resolution:* The Defence Ministry gave rounded figures and may count missile categories differently, e.g. 25 rather than 28 ballistic or hypersonic.
- **explained** · strength 0.5 — For the same night Ihnat gives 28 ballistic missiles and the Defence Ministry 25 ballistic or hypersonic missiles.
  - `c18-0004-03` 28 ballistic missiles flew at Kyiv on 1–2 July 2026, the most ever fired in one attack on the capital, according to Air Force spokesman Ihnat. *(claimant: Yurii Ihnat; `kyivindependent-2026-fenbert-kyiv-attack-31-killed`)*
  - `c18-0010-05` Ukraine's Defence Ministry gave rounder figures for 1–2 July 2026: "almost 500" drones and 77 missiles, 25 of them ballistic or hypersonic. *(claimant: Ministry of Defence of Ukraine; `pravda-2026-bondarieva-fedorov-patriot-appeal`)*
  - *Resolution:* The Defence Ministry gave rounded figures and may classify the Zircons or S-400 missiles differently from the Air Force tally of 24 Iskander/S-400 plus 4 Zircon.

## c19 Chapter: Operation Vivaldi

- **explained** · strength 0.6 — The 3rd Army Corps claimed Nove and Ridkodub liberated on 28 Sept 2026; the Russian MoD claimed them the next day.
  - `c19-0005-08` The 3rd Army Corps announced on 28 September 2026 (season 3 of Operation Vivaldi) that Nove, Ridkodub and Katerynivka had been liberated. *(claimant: 3rd Army Corps; `isw-2026-09-28-roca`)*
  - `c19-0009-102` On 29 September 2026 the Russian Ministry of Defence posted older footage to claim Ridkodub, Novomykhailivka and Nove. *(claimant: —; `isw-2026-09-29-roca`)*
  - *Resolution:* The Russian MoD used older footage; DeepState and ISW geolocations back the Ukrainian claim.
