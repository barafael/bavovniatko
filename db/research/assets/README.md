# Equipment 3D-asset catalogue

**Hand-written 2026-10-10** (six research sweeps + a licence/provenance re-verification pass against
Sketchfab's public API); **T-64/T-72-focused sweep added 2026-10-11** — licence labels *and* GLB
export re-verified for every new row via an authenticated API pass. The war's most common tanks
are no longer missing, see §1. It is deliberately **outside the knowledge base**: nothing here is in `db/`,
`sources.yaml` or the topic files. Per repo policy this file is **links only**; if we ever mirror
downloads, they go to `db/research/assets/files/` (gitignored).

What's inside: online, downloadable, freely-licensed 3D models of the equipment of this war — tanks,
IFVs, artillery, MLRS, air defence, glide weapons, aircraft, drones, ships, trucks, cars, boats — for
the Bevy dioramas and as modelling reference.

**Downloading:** [fetch.py](fetch.py) parses these tables as its source of truth and mirrors the
assets into `files/<category>/<id>/` — each with a `meta.yaml` (author, licence + URL, provenance
tier, per-file sha256, ready-made attribution line), plus `files/CREDITS.md` and `files/REPORT.md`.
It re-verifies every licence at download time and refuses models whose description admits a game
rip. Sketchfab needs a free account's API token (`--token`, `SKETCHFAB_TOKEN`, or
`~/.config/sketchfab/token`); run `python3 fetch.py --list` to preview, `--only-extras` for the
token-free packs. Resumable: re-run and it skips what's already `status: ok`.

## How to read it

- **Licences.** Everything is **CC BY 4.0** unless marked otherwise. That fits our CC BY 4.0 content
  licence, but **attribution is mandatory** (see the appendix). `⚠SA` marks share-alike (CC BY-SA /
  GPL): usable, but that asset and its derivatives must stay under the same licence. Anything
  NonCommercial, NoDerivs, or "royalty-free EULA" (TurboSquid/CGTrader/Free3D/Fab) was rejected.
- **Provenance column `P`:**
  - `✓` — licence label verified via API on 2026-10-10 *and* the description presents it as the
    uploader's own work (or a retexture chain with a named, independently-available base model).
  - `?` — licence label fine, but the model credits an untraceable third party, or the author's
    record is mixed (see warnings below). **Confirm before shipping.**
- **Game-rip trap (read this first).** Huge amounts of "CC BY" military content on Sketchfab are
  rips of *War Thunder*, *WARNO*, *Squad*, *Combat Mission*, *DCS*, *Wargame: Red Dragon*,
  *Battlefield*… The uploader cannot grant a licence for those. We checked every recommendation's
  description and rejected anything that admits a rip — including several that look perfect. See
  [Known-unusable accounts and items](#known-unusable-accounts-and-items).
- **Sketchfab mechanics.** Downloads need a free account. Every downloadable model can be exported
  as **glTF/GLB** (plus USdz/FBX and usually the original .blend) by Sketchfab's converter — ideal
  for Bevy. Face counts below are Sketchfab-reported: **15–60k ≈ game-ready**, ~100–300k = decimate
  first, 0.5M+ = reference/bake source only. `STL` rows are 3D-print files: reference only.
- **Links** are `sketchfab.com/3d-models/<uid>`; they redirect to the full slug.

### Provenance warnings

- **`42manako`** — the most prolific "Ukraine war markings" uploader, and mostly a *re-texturer*:
  itemised checks found admitted rips (T-64BV ← Combat Mission, T-72B3 ← Squad, T-62MD ← WARNO,
  GAZ Tigr ← Squad, 2S3/PzH 2000 ← Call to Arms, Pantsir ← War Thunder, KrAZ-6322/BM-27 ← DCS,
  Ural KUNG ← Spintires, Ropucha ← Cold Waters, Bushmaster ← Battlefield, AMX-10RC ← War Thunder)
  and untraceable "not my model" credits (HIMARS, Bohdana, Ural-4320, KamAZ-5350/63968, T-90A cage,
  BMD-4M, Mirage 2000-5F…). **Only his retextures of independently-listed originals are used below**
  (Tu-95MS, Il-76MD, TB2, T-90M — dual attribution).
- **`unhappy_nation`** — supplied many of the rips above; their own T-62M/T-62MV descriptions are
  clean, so those stay with `?`.
- **`Jeyhun1985`**, **`KojfDiscord`**, **`FlashPoin`**, **`Mr_Chiko`** — verified or strongly
  suspected rip re-uploads; excluded everywhere (they cover several gaps below with poison).
- **`MirzaArrafiERV_45`** — models check out as originals, but note the account handle reads
  `nazidefenseforceofficial`; prefer the alternatives where they exist.
- **`scout.`** "High-Quality model" series — no rip admission found, but zero provenance info;
  kept with `?`.
- Cults3D/MyMiniFactory "war game" STLs of modern Russian equipment are frequently *WARNO/War
  Thunder* rips too — check before paying/printing.

---

## 1. Tanks

| equipment | asset | author | ~faces | P | note |
|---|---|---|---|---|---|
| T-90M Proryv | [T-90M](https://sketchfab.com/3d-models/2e31a3cf16b04f0180b9387df5198c9a) | minehffd | 206k | ✓ | correct Relikt wrap; the Stepove-vignette antagonist |
| T-90M | [T-90M (With Interior) [FREE]](https://sketchfab.com/3d-models/f465e873d8d0468abd5d84b1ecc22f19) | mansoorsajidp | 233k | ✓ | interior + 1 animation; base of 42manako's UA-marked retexture |
| T-90A | [T-90A Vladimir](https://sketchfab.com/3d-models/66e1ac73b0d940edb232335248bb3a2b) | MirzaArrafiERV_45 | 262k | ✓ | welded turret + ESSA; decimate |
| T-80 (base) | [T-80 MBT](https://sketchfab.com/3d-models/2daa7c41afbb4778af59d74acb26356c) | MirzaArrafiERV_45 | 45k | ✓ | gas-turbine T-64 derivative; base shape for the BV (add Kontakt-1 wrap) |
| T-72B3 | [T-72B3 Main Battle Tank](https://sketchfab.com/3d-models/dfe130dfce724c30958cec51f2bd86b7) | MirzaArrafiERV_45 | 83k | ✓ | the 2022-vintage Russian workhorse (Brovary column); upload is tagged "createdwithai" — sanity-check textures |
| T-72A-shape | [T72M1](https://sketchfab.com/3d-models/e5c1edee1c27446c874bde41fa8c7911) | DustyMojito | 155k | ✓ | GHPC-workflow study with own ArtStation renders; M1 ≈ export T-72A, nearest stand-in for B/B1; decimate |
| T-72 LOD | [Low Poly T-72](https://sketchfab.com/3d-models/a116335b48504edb867eb70ede62b1d9) | S1Priv | 24k | ✓ | generic T-72 shape; mass-spawn filler |
| T-64 | [T-64 MBT](https://sketchfab.com/3d-models/58321766e5b64399aa2679ca6f6da297) | MirzaArrafiERV_45 | 42k | ✓ | the only verified-free T-64 mesh anywhere; game-ready — confirm exact A/B configuration from the preview |
| T-64BM2 | [T-64BM2](https://sketchfab.com/3d-models/c93ddbf6cec9484a94190e560670b8ee) | Hamster6465 | 132k | ✓ | UA modernization with Nizh ERA; the Kyiv-oblast defence tank |
| T-64BV | [T-64BV](https://sketchfab.com/3d-models/ca3fc6daf73b49f5bd13f0573c9c6401) | unhappy_nation | 243k | ? | description is clean, but the same author's 26k T-64BV admits a WARNO rip — confirm before shipping |
| T-64 LOD | [T64 (LOW POLY)](https://sketchfab.com/3d-models/28296493f1a34b89a2dc3abe45bde841) | lm9241221 | 23k | ✓ | |
| T-80BVM | [T-80BVM Shakar](https://sketchfab.com/3d-models/bef5b0ff5275492789acaadde8bcde42) | fuzion_z3 | 558k | ✓ | only free BVM anywhere; heavy |
| Leopard 2A4 | [Leopard 2A4](https://sketchfab.com/3d-models/99122ebcf2154a74ad6556b6f20ccb9c) | MirzaArrafiERV_45 | 26k | ✓ | game-ready; as delivered to UA |
| Leopard 2A4 | [Leopard 2a4](https://sketchfab.com/3d-models/0f192d77d90f4a378f837f7f6bb750b5) | XxRxX | 121k | ✓ | detailed alt |
| Leopard 2A4M | [Leopard 2A4M](https://sketchfab.com/3d-models/80b589fc9c0b4720888b40d01d6e5153) | MirzaArrafiERV_45 | 28k | ✓ | CAN winter-kit look for UA |
| Leopard 2A6 | [Leopard 2A6](https://sketchfab.com/3d-models/3440ba1ba8ce48469ae489a99eaca45c) | MirzaArrafiERV_45 | 65k | ✓ | L/55; Mala Tokmachka vignette |
| Leopard 2A6 | [Leopard 2 A6](https://sketchfab.com/3d-models/7cb23d5322df4b409a880de635826067) | buh-late | 120k | ✓ | well-textured alt |
| Leopard 2A6 | [Leopard 2A6 [Free]](https://sketchfab.com/3d-models/b62ba39ae5584b768b53d58803c3cc4b) | mansoorsajidp | 426k | ✓ | bake/reference source |
| Challenger 2 | [Challenger 2 Mk.3](https://sketchfab.com/3d-models/f1e0bd03ab9d4409bb708f7c0a84b507) | MirzaArrafiERV_45 | 169k | ✓ | as delivered to UA |
| Challenger 2 | [Challenger II](https://sketchfab.com/3d-models/f7869d0d85ff4a65b2e1333ab00b25cf) | buh-late | 142k | ✓ | alt |
| Challenger 2 | [Challenger 2 (low-poly)](https://sketchfab.com/3d-models/34dc550c4e024649bdf96b8a6a8f4810) | ScurvyWoof | 30k | ✓ | LOD fallback |
| M1A1 Abrams | [M1A1 Abrams](https://sketchfab.com/3d-models/1529f5d842cb4f578aa8cb672524f044) | MirzaArrafiERV_45 | 70k | ✓ | add SA-era roof details for AU config |
| M1A1 Abrams | [Abrams M1A1 [Free]](https://sketchfab.com/3d-models/1b4a0b196df14e67bce38ed6fef7d944) | mansoorsajidp | 29k | ✓ | lighter pick |
| BMPT Terminator | [BMPT Terminator](https://sketchfab.com/3d-models/1ba3651408e2417e89827d678e2a8042) | maks_modeller | 93k | ✓ | as spotted in Ukraine 2022+ |
| BMPT-T | [BMPT Terminator 2](https://sketchfab.com/3d-models/8d023ae2bccb473ca0af973397b58e6d) | SanderWolf_ | 29k | ✓ | unmanned-turret variant |
| AMX-10RC | [AMX-10RC](https://sketchfab.com/3d-models/c88849a3c6bb4011bdabfc8ce91ae412) | cloudhub | 175k | ✓ | French donation (42manako's is a WT retexture) |
| T-62M/MV | [T-62M](https://sketchfab.com/3d-models/8eb0a82d5e56467f9a46b65ace54c592), [T-62MV](https://sketchfab.com/3d-models/ed5df572d71c4674975432f1e9251aa9) | unhappy_nation | 223k/229k | ? | clean descriptions, mixed author record |
| mine plough ref | [M1150 ABV](https://sketchfab.com/3d-models/26eaeb6a7dbf498db16e02b1c232de33) | Kalibrovshik | 495k | ? | Abrams-chassis breacher; plough geometry reusable |
| mine roller ref | [PT-34 mine roller 1/50](https://www.thingiverse.com/thing:3533248) | Alainkif | STL | ? | WWII-lineage; print reference |
| T-72 ref | [T-72 Raw Scan from Fort Hood Museum](https://sketchfab.com/3d-models/81ae81f0dbf24a89804eebd007074ac0) | pervonharke | 319k | ✓ | photogrammetry (495 frames from phone video); modelling reference |
| T-72 ref | [T-72 (outdated)](https://sketchfab.com/3d-models/fe83405ad7c441ff851b2760b3370b1c) | buh-late | 1.36M | ✓ | 8K textures, weathered/rusty; bake/reference source only |
| T-72 ref | [T-72 Tank](https://blendswap.com/blend/19929) | SpaceC | — | ✓ | BlendSwap CC-BY .blend built from own photographs; untextured, Blender 2.7x; turret shape queried in comments |
| T-72B wreck | [Destroyed Russian tank T-72b](https://sketchfab.com/3d-models/246100389e1f49fc9d325473abc04723) | 3dtomchuk | 1.48M | ✓ | photogrammetry of the Buzova wreck, battle of Kyiv 2022; wreck set-pieces |
| T-72B wreck LOD | [Destroyed Russian tank T-72b Low Poly](https://sketchfab.com/3d-models/6245b762214a4ee5a25b499169f384b5) | lil_bentley | 73k | ✓ | decimated derivative of the scan above; dual attribution |

**Still missing:** T-72B/B1 exact shape (the T72M1 stand-in above is closest; B3 is now covered), a
✓-grade T-64BV (build from the base T-64 + Kontakt-1, or clear the ?-row), T-80BV (base T-80 above
+ procedural ERA). The Buzova T-72B scan gives real 2022 geometry for the honest route.

## 2. IFVs, APCs, light armour

| equipment | asset | author | ~faces | P | note |
|---|---|---|---|---|---|
| BMP-1 | [BMP-1 (outdated)](https://sketchfab.com/3d-models/9b00c6b1da094bb3802af5ca6a3081bf) | buh-late | 550k | ✓ | 51×4K maps; decimate |
| BMP-1 | [BMP-1 (low-poly)](https://sketchfab.com/3d-models/8f0a883414d1488aaf43030782ce3288) | ScurvyWoof | 18k | ✓ | untextured; add materials |
| BMP-2 | [BMP 2](https://sketchfab.com/3d-models/6ffd38bd956241ab9524d9d9da573c1f) | usedm | 47k | ✓ | best game-ready pick |
| BMP-2 ref | [BMP-2 photogrammetry](https://sketchfab.com/3d-models/fe516c430c3849fd995c9c8b89d587d1) | pervonharke | 489k | ✓ | real-vehicle scan, modelling reference |
| BMP-3 | [BMP-3](https://sketchfab.com/3d-models/a45b0e7355634b8c8b7e30af416ace73) | lampilamp | 335k | ✓ | 2A70+2A72 correct; decimate (Kurdyumivka) |
| BTR-80 | [BTR-80 RKHM](https://sketchfab.com/3d-models/dfd734a9ae4343fc8f572b11fe09ead3) | perceval-66 | 51k | ✓ | recon variant, base-80 hull/turret |
| BTR-80 | [Abandoned Soviet BTR-80](https://sketchfab.com/3d-models/32145d6303e5487e9d92097b9845ef02) | GameDevNick | 108k | ✓ | derelict styling; retexture for service |
| BTR-82A | [Low poly BTR 82](https://sketchfab.com/3d-models/6226acae68424d99bf03882b245ff7c5) | S1Priv | 21k | ✓ | check 2A72 turret fit; baked as `btr80`, standing in for the 4th NGU Brigade's BTR-3E at Hostomel (30 mm turret, closest free match) |
| BTR-4 Bucefal | [BTR-4 vehicle](https://sketchfab.com/3d-models/0653ced66d2f49cebdc4edfd0f0db04a) | Taher.Maknine | 102k | ✓ | UA-built, game-ready |
| BTR-4 | [Low poly BTR 4](https://sketchfab.com/3d-models/ecd845e80b1549dab7464b8f29c45cc9) | S1Priv | 28k | ✓ | budget alt |
| MT-LB | [MTLB](https://sketchfab.com/3d-models/369d8a7eb1ed4daeaa52843ba7563beb) | citizensnip | 89k | ✓ | the workhorse |
| MT-LB | [Mtlb](https://sketchfab.com/3d-models/d38c5f52c10d420094e8e8b49a7e3d05) | sacher | 76k | ✓ | uploader's own Arma asset, interior |
| MT-LB + MT-12 | [Mt-lb With Mt-12 Rapira](https://sketchfab.com/3d-models/3dd22a3b82fe4e20a7da7d3198b22fdf) | Hamster6465 | 21k | ✓ | improvised AT gun; very 2022 |
| BMD-2 | [Bmd-2 Free](https://sketchfab.com/3d-models/407cda86aaa74654a2b0574015e798fd) | 2w | 10k | ✓ | decimated scan |
| BMD-2 wreck | [Destroyed russian BMD-2](https://sketchfab.com/3d-models/775bfacbff8d4525aafc397f3f49e11d) | yaro.pro | 229k | ✓ | Kyiv siege wreck scan; Hostomel set piece |
| BMD-4 proxy | [BMD-4](https://sketchfab.com/3d-models/b3f97dc7bcf94674abcf25b9a48e3b33) | cloudhub | 260k | ? | base BMD-4 ≈ 4M; untextured proxy |
| Bradley | [M2A3 Bradley (low-poly)](https://sketchfab.com/3d-models/d691ec8d9a1a4981b4c246c59b8ef370) | ScurvyWoof | 33k | ✓ | M2A3 shape; kitbash ODS ERA for v02 vignette |
| Bradley | [Low poly M3a3 Bradley](https://sketchfab.com/3d-models/013cf36644314d1787fa55997729a08f) | S1Priv | 24k | ✓ | scout variant, common hull |
| Bradley ref | [M2 Bradley 1 (Raw Scan)](https://sketchfab.com/3d-models/a842a1ca579a4957be7d8e9704a341e7) | kryik1023 | 2.5M | ✓ | own photos, Fort Benning; reference |
| Marder 1A3 | [MARDER 1A3](https://sketchfab.com/3d-models/53238479325448fcb158e5ea58568e38) | MirzaArrafiERV_45 | 33k | ✓ | correct 1A3 kit |
| Marder proxy | [Marder 1A3 W.I.P](https://sketchfab.com/3d-models/42ec9739505f44ffae803aae121d1cc4) | MalleninjaJP | 14k | ✓ | WIP proxy/LOD |
| CV90 | [STRIDSFORDON (CV-90)](https://sketchfab.com/3d-models/1e3e8f5df84d47a69a3a74103dc62cb4) | MirzaArrafiERV_45 | 7k | ✓ | ≈CV9040C Swedish donation |
| Stryker | [M1126 ICV](https://sketchfab.com/3d-models/b0c559a02c1842b0976a241c9a998360) | cenobyte | 122k | ✓ | game-ready |
| Stryker | [M1126 (low-poly)](https://sketchfab.com/3d-models/66eef8e96a3f477eba57e656ce8ac7a0) | ScurvyWoof | 32k | ✓ | LOD |
| Stryker ⚠SA | [Stryker](https://sketchfab.com/3d-models/fe28759be4914e4a9ea52d73aa503a4e) | bunnyuwuj | 138k | ⚠SA | CC BY-SA; backup only |
| BTR-60 | [BTR60](https://sketchfab.com/3d-models/ffb08c13235c48ae9eb27532f68e628e) | hiamproyou1 | 56k | ✓ | |
| BRDM-2 | [BRDM 2](https://sketchfab.com/3d-models/065dc054914f40fdbfb25ad62d6696b7) | Protdj | 130k | ✓ | |
| M113 | [M113A1](https://sketchfab.com/3d-models/b544214a78b64b47999022f50701cd3c) | MenOfMena | 23k | ? | |
| M113 | [Low poly M113](https://sketchfab.com/3d-models/b9fb203a59ee453f83069cf7b2612886) | jesamabin | 5k | ? | filler |
| Bushmaster | [Bushmaster PMV (low-poly)](https://sketchfab.com/3d-models/90f5615410f7465ea811376066aa564f) | ScurvyWoof | 27k | ✓ | UA-operated (42manako's is a Battlefield rip) |

**Cope cages / ERA cages:** no verified-free add-on set exists; every "with cage" upload sits on a
rip or untraceable hull. Build procedurally — the geometry is trivial and provenance stays clean.

## 3. Artillery — towed and self-propelled

| equipment | asset | author | ~faces | P | note |
|---|---|---|---|---|---|
| D-30 122mm | [D-30 howitzer](https://sketchfab.com/3d-models/0f37641766054d9c9ff0666158b26426) | 42manako | 8k | ? | "Model by BUTTHE4D" — trace that author |
| 2A65 Msta-B | [howitzer 2a65 msta](https://sketchfab.com/3d-models/6bd63aac4fc241d0bdacc5e4639ffd0d) | fabertibler | 7k | ✓ | only usable Msta-B |
| MT-12 Rapira | (in MT-LB combo, §2) | Hamster6465 | 21k | ✓ | separates for emplacement |
| M777 155mm | [M777 howitzer](https://sketchfab.com/3d-models/2a2c381888ef4e4790ed883cddda0aca) | miloschnack | 183k | ✓ | trim for LODs |
| 2S3 Akatsiya | [2S3 Akatsiya](https://sketchfab.com/3d-models/3df6f262cb5a4aa39dd84e1f8e433948) | SanderWolf_ | 25k | ✓ | |
| 2S19 Msta-S | [2S19 MSTA-S](https://sketchfab.com/3d-models/c12432ada9a84859bc74c3d5ef7046f8) | MirzaArrafiERV_45 | 47k | ✓ | M2 diffs are minor |
| 2S19 LOD | [2S19 (LOW POLY)](https://sketchfab.com/3d-models/64f3ecce8fb747b08dc99d24b569f02d) | lm9241221 | 7k | ✓ | |
| 2S22 Bohdana | [2S22 Bohdana](https://sketchfab.com/3d-models/f98d6d99e9c6442cb94b65385e0af84c) | MirzaArrafiERV_45 | 420k | ✓ | "(Test)" heavy build |
| PzH 2000 | [PANZERHAUBITZE 2000](https://sketchfab.com/3d-models/da078be6bc66461e87034f29600a4982) | MirzaArrafiERV_45 | 53k | ✓ | (42manako's is a Call-to-Arms rip) |
| M109 | [M109 PALADIN](https://sketchfab.com/3d-models/3de0572475d54f7b9da18d6ed26b4a10) | MirzaArrafiERV_45 | 47k | ✓ | closest to UA's A3/A6 mix |
| CAESAR 6x6 | [CAESAR SPG](https://sketchfab.com/3d-models/35e840605e9c41f581e920c2eeaee6ca) | 42manako | 31k | ? | "not my original" — trace base |

**Missing:** 2S1 Gvozdika, 2S7 Pion, 2S4 Tyulpan, AHS Krab, AS-90, Archer — nothing verified-free
(rip-only or licence unverifiable).

## 4. MLRS

| equipment | asset | author | ~faces | P | note |
|---|---|---|---|---|---|
| BM-21 Grad | [БМ-21 "Град"](https://sketchfab.com/3d-models/c90559a30e6a414d993a0d3bdf6c5ff8) | Hsu.Pei.Ge | 115k | ✓ | Ural-375D + 40 tubes |
| BM-21 LOD | [BM-21 (LOW POLY)](https://sketchfab.com/3d-models/0a358a81e71140d7b0446c2fb25291cc) | lm9241221 | 16k | ✓ | |
| BM-27 Uragan | [BM-27 Uragan](https://sketchfab.com/3d-models/536cf77932ae4784ad5ad1d69937eaa2) | javanilga | 46k | ✓ | ZIL-135LM chassis |
| BM-30 Smerch | [BM-30 (LOW POLY)](https://sketchfab.com/3d-models/36fba6ae49724379bd847d3040737fe4) | lm9241221 | 7k | ✓ | cheapest; scout.'s 274k as detail ref (?) |
| TOS-1A proxy | [TOS-1A (Low-Poly)](https://sketchfab.com/3d-models/3d801e8cdc6a4f89bd1b5345d662fcc3) | Prisma3dModel4 | 2k | ✓ | crude; distance only |
| TOS-1 proxy | [TOS-1 BURATINO (LOW POLY)](https://sketchfab.com/3d-models/255fba6ba6c44b97aaea91d0225d0c75) | lm9241221 | 7k | ✓ | 30-tube base variant |
| M142 HIMARS | [M142 HIMARS \| FREE \| ANIMATION](https://sketchfab.com/3d-models/53c53a112c674d29a2afdbddbe3cecb5) | Denys.Cherkasov | 78k | ✓ | **animated launch sequence** — chapter 4 |
| M142 HIMARS | [MLRS - Low Himars](https://sketchfab.com/3d-models/1f6d5dd9a2d9432c87e626fa0a88c5d5) | degyuser | 77k | ? | alt build |
| M270 MLRS | [Free M270](https://sketchfab.com/3d-models/fc879301999a4cd0a09721b51affba9b) | DaHoon | 532k | ? | only non-rip; needs decimation |

## 5. Air defence

| equipment | asset | author | ~faces | P | note |
|---|---|---|---|---|---|
| S-300 TEL | [Lowpoly S300](https://sketchfab.com/3d-models/db81bd95b7a14ecf8e0be4e605681a7f) | S1Priv | 24k | ✓ | 5P85-style; no free radar exists |
| Buk | [BUK Missile System](https://sketchfab.com/3d-models/7663f9fca2ce485da8c0fafcf2d40d10) | azlan | 49k | ✓ | TELAR; variant unspecified |
| Tor-M2 | [Tor‑M2 (SAM‑15)](https://sketchfab.com/3d-models/f7df8c790fd14a3cba9e8b863715abcf) | Chenzoss | 120k | ✓ | |
| Pantsir-S1 | [Pantsir-S1](https://sketchfab.com/3d-models/3cb67b0bae10418190e7ce32142231e4) | SanderWolf_ | 88k | ✓ | (42manako's is a WT retexture) |
| Strela-10 | [MTLB Strela-10 (low-poly)](https://sketchfab.com/3d-models/bbc5ef74a6024739aa89d7e5bbd53f21) | ScurvyWoof | 18k | ✓ | |
| ZU-23-2 | [ЗУ-23](https://sketchfab.com/3d-models/fdae5a37901941fea3a02578506072a3) | LuckySkink9661 | 264k | ✓ | heavy scan; decimate; also truck/boat mount |
| Gepard | [Low poly Flakpanzer gepard](https://sketchfab.com/3d-models/43d276c929184ad6822d9b868e1dbd26) | S1Priv | 21k | ✓ | |
| Gepard detail | [Flakpanzer Gepard](https://sketchfab.com/3d-models/43746c9ec4a64f8d9a30db81b82843bd) | scout. | 142k | ? | |
| NASAMS | [Nasams 1 SAM](https://sketchfab.com/3d-models/5fb34fab137c430988fd5642a3b87db3) | MirzaArrafiERV_45 | 3k | ✓ | NASAMS 1 canisters; UA uses 2/3 |
| Patriot | [MIM-104 Patriot](https://sketchfab.com/3d-models/95e1e402426f4ddcb543af7a0455cf05) | MirzaArrafiERV_45 | 12k | ✓ | M901 launcher |
| Patriot | [MIM-104 Patriot](https://sketchfab.com/3d-models/163622db973542ffbf697490d1bc571f) | SnicksGameDevelopment | 21k | ✓ | alt launcher |
| PAC-2 missile | [MIM-104 SAM](https://sketchfab.com/3d-models/7a64d0af78514a159877edab1ab2bccb) | MirzaArrafiERV_45 | 14k | ✓ | close-up prop |
| Stinger | [FIM-92 Stinger (Low-Poly)](https://sketchfab.com/3d-models/c97e321bae1249a78ac4eed43a932ed7) | GoldbergR | 12k | ✓ | chapter-1 prop; missile-only variant same author |

**Missing:** S-300 radar (30N6/36D6), Patriot AN/MPQ-53/65 radar, IRIS-T SLM launcher, correct
NASAMS 2/3 TEL — rip-only or view-only.

## 6. Glide weapons and missiles

| equipment | asset | author | ~faces | P | note |
|---|---|---|---|---|---|
| FAB-500 body | [FAB-500 M-54 V5](https://sketchfab.com/3d-models/87e064b28f6f4d7ba1fde7e79d621b8d) | 4ikalka | 6k | ✓ | M-54 high-drag body — **not** the UMPK's M62 |
| Mk82+JDAM tail | [Mk82 (JDAM)](https://sketchfab.com/3d-models/b17b76a5e8be474d9cfc400efe27cf7b) | ferisecat | 9k | ✓ | GBU-38 config; kitbash JDAM-ER wings (none free) |
| R-360 Neptune | [Neptune missile](https://sketchfab.com/3d-models/a64c109936da417eb7ce4346a801a276) | MrZeuglodon | 22k | ✓ | sank the Moskva; no free TEL |
| Storm Shadow | [Storm Shadow Ukraine](https://sketchfab.com/3d-models/d1ad04ce964f46dd9cbac6a64cd19b1b) | Viktor.Zhuravlev | 12k | ✓ | correct box fuselage |
| ATACMS ref | [MGM140](https://sketchfab.com/3d-models/e370939d9c9c41d7a0bab8189ce125fa) | mahmadsaleem11111 | 540k | ? | reference-grade |
| Kalibr | [SS-N-27 Kalibr 3M-54](https://sketchfab.com/3d-models/63fc0ac8032c42e3bfd3e548192d5a9c) | mmickel | 7k | ✓ | anti-ship member; ≈3M-14 except nose |
| Iskander TEL | [ISKANDER](https://sketchfab.com/3d-models/244b511ee2ca4f3794d118b872ee229c) | dofash_76 | 22k | ✓ | |
| Iskander LOD | [Russian Iskander Low-poly](https://sketchfab.com/3d-models/934e70c449e7408bb54da06d8641d5dc) | Prisma3dModel4 | 6k | ✓ | |

**Missing: the UMPK glide-bomb kit itself.** No verified-free UMPK (or FAB-1500/3000-with-UMPK)
exists — the only candidates are game rips or NC. The wing kit is ~50 faces of work: kitbash
strakes + fold-wing box onto any FAB body. This matters: glide bombs are the war's signature weapon.

## 7. Helicopters

| equipment | asset | author | ~faces | P | note |
|---|---|---|---|---|---|
| Mi-8/17 | [Mi-8](https://sketchfab.com/3d-models/5e5e57ef66994c6196a5f5d6b9f50403) | tiunov.se | 17k | ✓ | Azovstal air-bridge; repaint, not remodel |
| Mi-8 LOD | [Mi8](https://sketchfab.com/3d-models/1bc2f674ad8345648710f16df847599a) | manilov.ap | 35k | ✓ | |
| Mi-24 Hind | [Mi24](https://sketchfab.com/3d-models/5d0b5d8dc985401685f217a775deadad) | manilov.ap | 71k | ✓ | closest to Mi-24P |
| Mi-24 LOD | [Mi-24 Hind](https://sketchfab.com/3d-models/004d68143e1a4df88e136dbc0a05f181) | duanesmind | 4k | ✓ | |
| Ka-52 | [Lowpoly Ka-52](https://sketchfab.com/3d-models/1b47fb709d634ea987a3bb1a30c1507b) | S1Priv | 19k | ✓ | coaxial layout correct; Robotyne lob fire |
| Ka-52 detail | [Ka-52](https://sketchfab.com/3d-models/a7f227659c1a42ec92ea7329ab1d90d6) | maddex88 | 86k | ? | cinematics |
| Mi-28N | [Mi-28N Havoc](https://sketchfab.com/3d-models/3e80c95bbadf46abbafba9d9a08afc68) | rukh3d | 25k | ✓ | bonus |

## 8. Fixed-wing aircraft

| equipment | asset | author | ~faces | P | note |
|---|---|---|---|---|---|
| Su-25 | [[PBR] Sukhoi Su-25](https://sketchfab.com/3d-models/956d0cb0bbe54fa8ad72e8c3e224fd2d) | Immersive3A | 16k | ✓ | both sides' CAS |
| Su-25 | [Su-25](https://sketchfab.com/3d-models/88b71eb848cf4418a95dff497c07cefc) | tnikita | 16k | ✓ | |
| Su-27 | [Sukhoi Su-27 "Flanker"](https://sketchfab.com/3d-models/2541c1dbc6c1499cb7b38b5bf1314750) | andertan | 29k | ✓ | UA interceptor force |
| Su-30 proxy | [[PBR] Sukhoi Su-30](https://sketchfab.com/3d-models/421bd56fd97c4c7196904e778f27aef7) | Immersive3A | 20k | ✓ | generic Flanker-C; check SM details |
| Su-30SM2 | [SU-30SM2](https://sketchfab.com/3d-models/3ff55f1cfa934aed89838db19ea4407f) | urastyzh | 11k | ? | rare exact SM2; verify textures |
| Su-34 | [Su34](https://sketchfab.com/3d-models/1c94f436be51453fa598e9eab50eb0a7) | manilov.ap | 58k | ✓ | the one clean Fullback |
| Su-24M | [Su24](https://sketchfab.com/3d-models/3439ae4000df438894a44127ebda538c) | manilov.ap | 12.6k | ✓ | Hostomel bomb runs; same author as the ✓ Mi8/Mi24/An225/Su34 rows, no rip admission |
| Su-35S | [Sukhoi Su-35](https://sketchfab.com/3d-models/c98cf9b3b3e04017a04732798a31888a) | andertan | 24k | ✓ | no canards + TVC |
| Su-35S | [Su-35 low poly](https://sketchfab.com/3d-models/830b103029184fdf853dce9bb1ea7336) | Chenzoss | 21k | ✓ | |
| MiG-29 | [MiG-29 (Ukrainian Air Force)](https://sketchfab.com/3d-models/34afffebbf494ff281b5c2b00d03ad65) | ADSKIE | 179k | ✓ | UA markings out of the box |
| MiG-31 | [Миг-31](https://sketchfab.com/3d-models/b168fbca1f6c4ad0ad4e45b7a22f52bc) | tnikita | 88k | ✓ | Kinzhal carrier |
| Tu-95MS | [Tupolev Tu-95 "Bear"](https://sketchfab.com/3d-models/c0f87ce4c461451a96e9d0a0990831b3) | helijah | 35k | ✓ | FlightGear dev's own mesh; Spiderweb |
| Tu-95MS UA livery | [Ukrainian Tu-95MS](https://sketchfab.com/3d-models/b550941898c848de95ffeed8c8d7834f) | 42manako | 65k | ✓ | legit retexture of helijah's CC BY mesh; dual attribution |
| Tu-22M3 | [Tu-22M3](https://sketchfab.com/3d-models/da59874e999a41f4a73da4c0bc5d1f43) | shermannwolf | 126k | ✓ | swing-wing |
| A-50U | [Beriev A50](https://sketchfab.com/3d-models/e0a7e6d5dfe54ce986870103fb9916c0) | MaX3Dd | 9k | ✓ | low-poly, 2048² diffuse; chapter 12 — only free A-50 anywhere |
| Il-76 | [Ilyushin Il-76](https://sketchfab.com/3d-models/ab71a5f790f940798cc755246a7a8a7f) | helijah | 56k | ✓ | Hostomel |
| Il-76 LOD | [Low poly Il-76](https://sketchfab.com/3d-models/f4213ecafa644cb287d5e6cd39c6e28e) | S1Priv | 14k | ✓ | airdrop stream |
| An-26 | [Antonov An-26](https://sketchfab.com/3d-models/df3d6b1359144a459d170abef262211d) | 42manako | 63k | ? | credits third-party base mesh (Otavio B.) |
| An-225 | [An225](https://sketchfab.com/3d-models/9d2af38b75bb4dd1a4aa58d026a5dc3c) | manilov.ap | 21k | ✓ | Mriya; destroyed at Hostomel |
| F-16C | [F-16 \| NATO \| GameReady](https://sketchfab.com/3d-models/1d74c97a06384d318b1d09e0e3811055) | rave.Ar4ik | 115k | ✓ | UA camo + stores; best in class |
| F-16 alt | [F-16e/f](https://sketchfab.com/3d-models/d54c72e055a647ad9173e01548e50db3) | andertan | 35k | ✓ | Block 60 — strip CFTs for UA MLU look |
| Mirage 2000 | [mirage 2000](https://sketchfab.com/3d-models/14bb51b36e4b470abbe15761a549deb8) | Chenzoss | 120k | ✓ | repaint for 5F (no exact free 5F) |
| L-39 | [L39](https://sketchfab.com/3d-models/4b37b2b4ae9e40ac9a6fb876e2142672) | manilov.ap | 59k | ✓ | UA attack use |

**GPL alternative ⚠SA:** FGMEMBERS/FlightGear hold GPL Su-27, MiG-29, Il-76, Tu-95, L-39 (e.g.
[github.com/FGMEMBERS](https://github.com/FGMEMBERS)) — accurate but share-alike.

## 9. Drones (air)

| equipment | asset | author | ~faces | P | note |
|---|---|---|---|---|---|
| Shahed-136/Geran-2 | [Iranian Shahed-136](https://sketchfab.com/3d-models/af8ac3d45ade494fb280c99922513ae2) | Chenzoss | 120k | ✓ | textured delta-wing; the night-raid staple |
| Shahed-136 | [suicide drone Shahed 136](https://sketchfab.com/3d-models/92a87f5f4e6d4d5a9d56a27d09c49faf) | ghafurian.mahdi | 77k | ✓ | covers Geran-2 designation |
| jet Geran ref | [Shahed 238](https://sketchfab.com/3d-models/10b7f80a149247748d8912bd0f59d517) | Rudy27 | 1.96M | ✓ | jet variant; reference/wreckage only |
| Lancet-3 | [Lancet](https://sketchfab.com/3d-models/9820cfa9d1084d918c85c6dfb6287737) | flightclubua | 38k | ✓ | double X-wing |
| Orlan-10 | [Orlan-10](https://sketchfab.com/3d-models/e2efb223c1814b7291818a9a7b4e80c5) | flightclubua | 38k | ✓ | RU recon workhorse |
| Supercam S350 | [Super Cam](https://sketchfab.com/3d-models/519052b76cc84036ae55bd567213a881) | Viktor.Zhuravlev | 11k | ✓ | 4K textures |
| ZALA 421-16E | [Zala_421_16E2](https://sketchfab.com/3d-models/25c8c177042340238af814ded16ca24b) | Viktor.Zhuravlev | 10k | ✓ | exact variant |
| FPV quad | [Combat FPV drone](https://sketchfab.com/3d-models/df640a20c2af49a6af7c371b2d49d51f) | Amaranthus_retroflexus | 40k | ✓ | RPG-7 warhead; most on-theme |
| FPV quad LOD | [Low-Poly FPV Drone](https://sketchfab.com/3d-models/d080d91ee43446d29fa6a446664bc4c4) | misha_xok | 8k | ✓ | mass-spawn |
| fibre FPV | [Tail FPV Optic Fibre Drone](https://sketchfab.com/3d-models/4c8c3dc239a8483c8053f8ce7515441b) | line-o | 217k | ✓ | AI-generated; only fibre-spool variant found |
| Baba Yaga proxy | [Octocopter (DJI S1000 parts)](https://sketchfab.com/3d-models/2b401b4488d649bcb05d1b2af3212471) | idmental | 82k | ✓ | heavy-lift airframe to dress as night bomber |
| Mavic 3 | [DJI Mavic 3](https://sketchfab.com/3d-models/c5a5abae1dea468ab73b1bdc7d616fa6) | llirikslon | 127k | ✓ | recon quad |
| Mavic 3 + pad | [Mavic 3 on landing pad](https://sketchfab.com/3d-models/e380b27d1c2d40c485f60a0c73bb874d) | eyesonukraine | 141k | ✓ | from a UA war-documentation account |
| Bayraktar TB2 | [Baykar Bayraktar TB2](https://sketchfab.com/3d-models/8e5b6972f7d049f19688096e03949487) | TheDevilsEye | 9k | ✓ | chapters 1–2 |
| TB2 alt | [Bayraktar TB2](https://sketchfab.com/3d-models/097dbe6815c24f31957c111cd922a7cd) | Kellot | 31k | ✓ | untextured detail |
| Shark proxy | [Shark M](https://sketchfab.com/3d-models/58d1ed0dbbd8412983f6daa921b2d0b5) | rb014s | 137k | ? | AI-generated; verify silhouette |
| UJ-32 Lastivka | [UJ-32 Lastvika UAV](https://sketchfab.com/3d-models/3f681c071708442c91f54407acdd33aa) | SpinoCactus | 15k | ✓ | closest free thing to UJ-26 Beaver |
| interceptor | [Piranha HUNTER](https://sketchfab.com/3d-models/21b3fa2d6e304444b56e9ffc96af9ec8) | flightclubua | 36k | ✓ | real interceptor product |
| interceptor | [One Punch — Tailsitter FPV](https://sketchfab.com/3d-models/1c0acf0db579411a94889aa5a4042f58) | flightclubua | 36k | ✓ | night-AD-vs-Shahed asset |
| jet interceptor | [Chaklun](https://sketchfab.com/3d-models/2d4cabfd99034863adece9b3835a782c) | flightclubua | 37k | ✓ | jet-powered |

`flightclubua` is the standout author here: textured, original, Ukraine-war-specific (also
[Magura](https://sketchfab.com/3d-models/b6da005a8d1e45e0872615f49110e08b),
[Sea Baby](https://sketchfab.com/3d-models/45596e0d0226454ab3c8df670fb8156e), ZALA, Harop, Skyfall
and more). **Missing:** UJ-26 Beaver, An-196 Liutyi, Tu-141 (NC-only) — nothing free.

## 10. Naval and sea drones

| equipment | asset | author | ~faces | P | note |
|---|---|---|---|---|---|
| Slava-class (Moskva) | [Project 1164](https://sketchfab.com/3d-models/b13773d65e7440bca803ec6485e5f1b6) | AKlyushnikov | 151k | ✓ | PBR; hero ship of chapter 2 |
| Slava LOD | [Russian Slava-class cruiser](https://sketchfab.com/3d-models/726f8cb290a346bfb066b773f76f5112) | 42manako | 17k | ? | claims own build; verify before use |
| Tarantul-III | [Tarantul-class corvette](https://sketchfab.com/3d-models/d1a31d73de034e298c80ac3e74f789dc) | 42manako | 968k | ? | untextured, heavy; Ivanovets |
| river assault boat | [Fast Assault Boat (Low Poly)](https://sketchfab.com/3d-models/02235e2c848e43b788d175be435232cf) | michael-dg | 8k | ✓ | CB90-style stand-in; Krynky |
| RIB | [rib boat](https://sketchfab.com/3d-models/844cf6413a614465b757d8b28a23cd22) | passas | 29k | ✓ | 13 m special-ops boat |
| civilian skiff | [Fishing boat](https://sketchfab.com/3d-models/f4b38ccf5ffb46018aa7931c0e106654) | Pabooklas | 7k | ✓ | the Dnipro workhorse |
| **Magura V5** | [MAGURA USV Drone boat](https://sketchfab.com/3d-models/b6da005a8d1e45e0872615f49110e08b) | flightclubua | 38k | ✓ | 3×4K textures; best find of the sweep |
| **Sea Baby** | [Seababy USV drone](https://sketchfab.com/3d-models/45596e0d0226454ab3c8df670fb8156e) | flightclubua | 38k | ✓ | with Starlink dome; Kerch chapter |
| USV variety | [Ukrainian USV "STALKER 5.0"](https://sketchfab.com/3d-models/e9e9cb1141964b7bb0b566f821bda2b6) | flightclubua | 37k | ✓ | swarm variety |
| USV LOD | [Ukrainian navy maritime drone](https://sketchfab.com/3d-models/2c347a181fea41319f75d9acac0c286e) | idmental.id | 11k | ✓ | first-gen USV |

**Missing:** Ropucha landing ship (only a Cold Waters rip), Karakurt and project 22160 (view-only
"Editorial"), Raptor patrol boat (paid only), Mamay/Marichka.

## 11. Trucks, engineering, UGVs

| equipment | asset | author | ~faces | P | note |
|---|---|---|---|---|---|
| Ural-4320 | [Ural-4320-31](https://sketchfab.com/3d-models/870869cec0a94d7788d4db3861184b1d) | kuwalol93 | 9k | ? | the convoy truck; both 42manako Urals are unusable |
| KamAZ-43118 ref | [Kamaz 43118 Flatbed 2017](https://sketchfab.com/3d-models/849b5345d8ae48bea1fe1a249d008359) | niev | 505k | ? | untextured; re-topology source |
| GAZ-66 | [Gaz 66 "Shishiga" #FreeLowPoly](https://sketchfab.com/3d-models/13a2a560bacf4df7bbba19e4e1905095) | vsevolod_smoki | 11k | ✓ | untextured low-poly |
| GAZ-66 detail | [Gaz 66](https://sketchfab.com/3d-models/4ae573e2fb084f63bd63a2bb44852c68) | tiunov.se | 54k | ✓ | |
| GAZ-66 scan | [GAZ-66 photogrammetry](https://sketchfab.com/3d-models/33e8892a55c84d8191fc199777808c3b) | trolleway | 141k | ✓ | real truck scan |
| ZIL-131 | [ZIL 131](https://sketchfab.com/3d-models/5c1c4ff26bee4754aa3288b13d5a6830) | danieljorge435 | 33k | ✓ | textured |
| KrAZ-255 | [low-poly KrAZ-255](https://sketchfab.com/3d-models/52e286e0d6274866a4a46ec975623806) | DU1701 | 12k | ✓ | untextured |
| UAZ-469 | [UAZ-469 (A-469)](https://sketchfab.com/3d-models/bb90dcb38f8e4a2d89a681320266e737) | strannik961 | 63k | ✓ | |
| UAZ Patriot | [UAZ Patriot](https://sketchfab.com/3d-models/35ba1f77bf7748b4ad2aa89f5dd75e9f) | tiunov.se | 28k | ✓ | RU command car |
| HMMWV | [Low Poly Humvee FREE](https://sketchfab.com/3d-models/c0e2ddcc4ccf4d49a9303303a6307bcf) | madtrollstudio | 8k | ? | weak category — no verified-free armoured Humvee |
| M113 family | see §2 (M113A1, low-poly) | — | — | ? | |
| Bushmaster | [Bushmaster PMV (low-poly)](https://sketchfab.com/3d-models/90f5615410f7465ea811376066aa564f) | ScurvyWoof | 27k | ✓ | |
| Novator | [Ukrainian Novator (Stugna)](https://sketchfab.com/3d-models/b0a76e62e84440eeb5fd18d48394fabb) | 42manako | 80k | ? | base by MirzaArrafiERV_45 — ask him for the original |
| Mitsubishi L200 | [Mitsubishi L200](https://sketchfab.com/3d-models/372e50454f3246239a5a1551f839f862) | David_Holiday | 26k | ✓ | the technical base of this war |
| Toyota Hilux | [Toyota Hilux](https://sketchfab.com/3d-models/417915f419f945c8a424b2a7943eb25a) | David_Holiday | 19k | ✓ | |
| DShKM (technical gun) | [dshk](https://sketchfab.com/3d-models/070de1ca5d664160b3fd489c412f871e) | hensyu_leader | 19k | ? | mount on any pickup — no ready-made free technical |
| NSV Utyos | [Low-Poly NSV "Utyos"](https://sketchfab.com/3d-models/74b32fa5addc40bea6376ef3ffdbb164) | wtksmb | 13k | ? | kitbash |
| river crossing | [M3 Amphibious Bridging Vehicle](https://sketchfab.com/3d-models/4790009738b6452da350663088881f64) | Owen.Barden | 538k | ✓ | western PMP analog; decimate |
| pontoon span | [Deep pontoon bridge](https://sketchfab.com/3d-models/976053b6002348229e5a2c00661c9007) | pruitt.jim | 321k | ✓ | Dnipro crossings |
| Starlink dish | [Starlink Satellite Dish](https://sketchfab.com/3d-models/84476761b87546d2b23fecfaaf849326) | malacodart | 344 | ✓ | mount on everything |
| UGV ref | [Ripsaw M5](https://sketchfab.com/3d-models/e530eddf0c354a189b9515825ed8a2de) | MirzaArrafiERV_45 | 1.9M | ✓ | generic tracked-UGV stand-in; very heavy |

**Missing:** KamAZ Typhoon-K, GAZ Tigr (both only as rips), KrAZ-6322/6446 (DCS rip), MaxxPro (NC),
UR-77 Meteorit (NC — the only model anywhere), MTU-72/TMM bridgelayers, PMP park, any real UA/RU
ground robot.

## 12. Civilian cars, vans, buses

| equipment | asset | author | ~faces | P | note |
|---|---|---|---|---|---|
| Lada Niva | [Lada Niva](https://sketchfab.com/3d-models/26ebf5872bce4ff4ad2baee41882d6b2) | mckechniegreg6 | 75k | ✓ | 40 maps |
| VAZ-2106 | [VAZ-2106](https://sketchfab.com/3d-models/a73b3cdeddd846209bf7c73d5f04d20f) | domhathair | 279k | ✓ | decimate |
| Lada 2107 | [LADA 2107](https://sketchfab.com/3d-models/31201f80892b404a8e49c272490a664c) | tiunov.se | 105k | ✓ | Riva sedan |
| Sprinter | [Mercedes Benz Sprinter 2006](https://sketchfab.com/3d-models/f69de1315bb049c8946d57f6006acd73) | Max-7215 | 38k | ✓ | drone-team/medevac van |
| Transit-class van | [Transit Van](https://sketchfab.com/3d-models/4d7e7de43c6149448d13881f06e903d5) | dani-satellite | 16k | ? | stylized filler |
| Bohdan bus | [Bohdan bus](https://sketchfab.com/3d-models/bb09481350b647219288ca7cfe4f5b1e) | vishchun | 8k | ✓ | the actual Ukrainian city bus |
| LAZ-695 | [LAZ-695](https://sketchfab.com/3d-models/2995773ef2bf4ef58bdb3f4c1f3f7178) | hans1240 | 59k | ✓ | Soviet-era Lviv bus |

**CC0 filler (civilian-traffic layer):** [Kenney Car Kit](https://kenney.nl/assets/car-kit) (CC0,
GLB-ready), [Quaternius Cars](https://quaternius.com/packs/cars.html) and
[Public Transport](https://quaternius.com/packs/publictransport.html) (CC0).

## Known-unusable accounts and items

Useful to know so nobody re-finds them and gets burned:

- **Game rips labelled CC BY:** 42manako's armour/truck/naval uploads (itemised above);
  KojfDiscord's "War Thunder/DCS" sets (would have covered 2S1, 2S4, AS-90, L118, IRIS-T, NASAMS 3,
  Patriot radar, FAB-1500, UMPK); Jeyhun1985 (Su-25/Su-34/Ka-52/Mirage verified WT geometry);
  FlashPoin's WoT T-62; Mr_Chiko's WARNO BMDs; the entire `scout.` "High-Quality" series is
  suspect. Also Cults3D "M2A2 Bradley WARTHUNDER WARNO" STL.
- **More T-64/T-72 traps (2026-10-11 sweep):** manbs999's "T-72B3 obr. 2011" admits a War Thunder
  import; DG2BOSSSSSS's "T-72" has a face count identical to MirzaArrafiERV_45's T-90AM — suspected
  reupload, skip; lm9241221's T64BV is view-only (the T64 LOW POLY row above is the downloadable
  one); the 42manako T-72 family (T-72 Ural/M1/A/B/B1/AV/B3/B3M, T-64A/B/BV/BM) is Combat Mission /
  Squad / WARNO / zavod_z material, as their own descriptions admit.
- **CC BY-NC (downloadable but not usable):** Karosio's BMP-3 (the best-looking one), 42manako's
  Russian technicals + MaxxPro + SA-13, jloiacono82's UR-77 Meteorit, aeck2142's Tu-141,
  Karosio's "Vampire UAV" Baba Yaga.
- **View-only ("Editorial"):** citizensnip's Karakurt/22160/Rubin; timonfreud's IRIS-T SLM;
  momsboxtv's 2S7; leondp's and svtoshi's Magura V5s.
- **Paid only:** Shark and UJ-26 (CGTrader/TurboSquid), Magura/Sea Baby commercial clones
  (RenderHub), Raptor (3dmodels.org), 2S4/FAB-UMPK/L118 (wargame STL shops).

## Attribution appendix (draft credits block)

Every CC BY asset needs: author — title — link — "CC BY 4.0". Sketchfab profiles:
`sketchfab.com/@<handle>`. Authors behind the ✓-rows above: 2w, 3dtomchuk, ADSKIE, AKlyushnikov,
Amaranthus_retroflexus, andertan, azlan, buh-late, cenobyte, Chenzoss, cloudhub, David_Holiday,
danieljorge435, Denys.Cherkasov, dofash_76, DU1701, duanesmind, DustyMojito, eyesonukraine, fabertibler,
ferisecat, flightclubua, fuzion_z3, GameDevNick, ghafurian.mahdi, GoldbergR, Hamster6465,
hans1240, helijah (Emmanuel Baranger), hiamproyou1, idmental.id, Immersive3A, lampilamp,
line-o, lil_bentley, llirikslon, LuckySkink9661, maddex88, mahmadsaleem11111, malacodart, manilov.ap,
maks_modeller, mansoorsajidp, MaX3Dd, mckechniegreg6, Max-7215, michael-dg,
miloschnack, minehffd, MirzaArrafiERV_45, mmickel, MrZeuglodon, Owen.Barden, Pabooklas,
passas, pervonharke, Prisma3dModel4, Protdj, pruitt.jim, rukh3d, rave.Ar4ik, Rudy27, sacher,
SanderWolf_, SavyTheCreator, scout., shermannwolf, S1Priv, SpaceC (BlendSwap), SpinoCactus, ScurvyWoof,
SnicksGameDevelopment,
strannik961, tiunov.se, tnikita, trolleway, urastyzh, usedm, Viktor.Zhuravlev, vishchun,
vsevolod_smoki, wtksmb, yaro.pro. Plus 42manako for the two legitimate retextures (Tu-95MS,
Il-76MD — credit helijah as base mesh, dual attribution).

**Rule: keep a `CREDITS.md` next to any downloaded files, one line per asset.**
Tier-`?` authors: confirm provenance first, then credit the same way. Kenney/Quaternius are CC0 —
no attribution required (but nice).

## Re-verification recipe

Licences move; re-check before shipping anything:

```
GET https://api.sketchfab.com/v3/search?type=models&q=<query>&downloadable=true&count=24
→ results[].license.label (keep "CC0…"/"CC Attribution…"; reject NonCommercial/NoDerivs;
   flag ShareAlike), .isDownloadable, .faceCount, .user.username, .uid
GET https://api.sketchfab.com/v3/models/<uid>   # fuller record (rate-limited anonymously)
GET https://api.sketchfab.com/v3/models/<uid>/download   # needs an API token; returns pre-signed
   glb/gltf/usdz/source URLs — confirms GLB export actually resolves, not just the licence label
```

Then read the description for rip admissions ("ripped from", "not my model", "provided by") — a
CC BY label from someone who doesn't hold the rights is worthless. Sweeps performed 2026-10-10 and
2026-10-11.
