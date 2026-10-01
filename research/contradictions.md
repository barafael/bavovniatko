<!-- Generated from the knowledge base by db/tools/render.py. Edit the data, not this file (see db/README.md). -->

# Contradictions

Pairs of claims that cannot both be true as stated, judged from rule-based candidates (db/tools/relate.py) and stored as `contradicts` edges. *Explained* means the difference has a stated cause (method, scope, date); *open* means it is unresolved. Claim ids resolve in the knowledge base; source ids in [sources.yaml](sources.yaml).

47 distinct contradictions (duplicate claims collapsed): 47 explained, 0 open.

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
- **explained** · strength 0.4 · 6 judged pairs — Ukrainian troops' 5:1 (8-9:1 at worst) shell ratio in early 2024 conflicts with officials' 1:10 for February 2024.
  - `00-0074-21` Key figure: the shell ratio reported by Ukrainian troops in early 2024 was 5:1 in Russia's favour, 8–9:1 at worst. *(claimant: Ukrainian forces (unnamed); `wikipedia-battle-of-avdiivka`)*
  - `03-0018-01` The Ukraine:Russia shell ratio reached 1:10 in February 2024 while US aid was stalled. *(claimant: Ukrainian officials (unnamed); `liga-2025-shell-ratio`)*
  - *Resolution:* Frontline anecdotes from particular sectors vs an aggregate official figure; the worst-case local ratio is close to 1:10.
- **explained** · strength 0.4 — A's 70,000 UMPK kits ordered for 2025 (RUSI) conflicts with HUR's claim of more than 100,000 a year.
  - `00-0077-20` In e8 Russia fought drone-centric, infantry-infiltration warfare with fibre-optic FPVs, 50,000+ Shaheds a year and 70k UMPK ordered. *(claimant: —; `csis-2026-jones-10-charts`, `rusi-2025-watling-third-year`)*
  - `03-0031-06` Russian UMPK production went from tens of thousands to more than 100,000 a year. *(claimant: Defence Intelligence of Ukraine (HUR); `pravda-2025-hur-glide-bomb-production`)*
  - *Resolution:* RUSI counts kits ordered; HUR cites planned production (120,000 for 2025).

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
- **explained** · strength 0.4 · 2 judged pairs — South Korean and British estimates of about 6,000 North Korean casualties differ from HUR's more than 7,000.
  - `04-0027-82` South Korea's NIS and British estimates put North Korean casualties at about 6,000. *(claimant: UK Government; National Intelligence Service (South Korea); `kyivindependent-2026-terajima-nk-hur`)*
  - `04-0033-35` HUR put North Korean casualties at more than 7,000 in June 2026. *(claimant: Defence Intelligence of Ukraine (HUR); `kyivindependent-2026-terajima-nk-hur`)*
  - *Resolution:* Different agencies and probably different dates; HUR's June 2026 figure may be later.
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
- **explained** · strength 0.6 — DeepState-derived 4,336 km² for 2025 conflicts with Slivochny Kapriz's 550-600 km² a month (about 6,600-7,200 km² a year).
  - `07-0026-23` The net Russian territorial gain computed from DeepStateMap versions for the year 2025 is 4,336 km². *(claimant: —; `deepstate-api-history`)*
  - `07-0030-13` The Russian project Slivochny Kapriz, which credits any area where Russian troops were seen, gives 550–600 km² a month of Russian gains for 2025. *(claimant: Slivochny Kapriz; `euromaidan-2026-tril-cost-per-km`)*
  - *Resolution:* Slivochny Kapriz credits any area where Russian troops were seen, including infiltration; DeepState marks control.
- **explained** · strength 0.6 — The computed May 2026 gross changes (78 km² Russian, 66 km² Ukrainian) conflict with Kyiv Post's quoted DeepState figures of 130 km² lost and 250 km² regained.
  - `07-0028-32` Computed from DeepStateMap versions, in May 2026 (e9) Russia gained 78 km² and Ukraine recaptured 66 km² (gross), a net Russian change of +14 km²; the grey zone was 1,589 km² at the end of the period. *(claimant: —; `deepstate-api-history`)*
  - `07-0028-44` Kyiv Post quotes DeepState as reporting 130 km² lost and 250 km² regained by Ukraine in May 2026, conflicting with DeepState's net 14 km² Russian gain per RFE/RL. *(claimant: DeepState; Kyiv Post; `kyivpost-2026-zavadska-deepstate-july`)*
  - *Resolution:* The computed net of +14 km² matches DeepState's net per RFE/RL; the Kyiv Post quote may be a misquotation or cover a different window (e.g. OPSEC releases).
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
