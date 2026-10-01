-- 0010_sim: the game feed. Derived from claims, never hand-typed; every row says what it rests on
-- (derived_from), how it was derived (method) and whether a human has reviewed it.

DEFINE TABLE IF NOT EXISTS sim_loadout SCHEMAFULL COMMENT "Which systems each side has in each era, and how plentiful";
DEFINE FIELD IF NOT EXISTS era          ON sim_loadout TYPE record<era> REFERENCE ON DELETE REJECT;
DEFINE FIELD IF NOT EXISTS side         ON sim_loadout TYPE "ua" | "ru";
DEFINE FIELD IF NOT EXISTS system       ON sim_loadout TYPE record<system> REFERENCE ON DELETE REJECT;
DEFINE FIELD IF NOT EXISTS availability ON sim_loadout TYPE "none" | "prototype" | "scarce" | "limited" | "common" | "abundant";
DEFINE FIELD IF NOT EXISTS first_available ON sim_loadout TYPE option<{ from: datetime, to: option<datetime>, precision: "day" | "month" | "quarter" | "year" }>;
DEFINE FIELD IF NOT EXISTS quantity_note ON sim_loadout TYPE option<string>;
DEFINE FIELD IF NOT EXISTS derived_from ON sim_loadout TYPE set<record<claim>> DEFAULT [];
DEFINE FIELD IF NOT EXISTS method       ON sim_loadout TYPE string;
DEFINE FIELD IF NOT EXISTS reviewed     ON sim_loadout TYPE bool DEFAULT false;
DEFINE FIELD IF NOT EXISTS notes        ON sim_loadout TYPE option<string>;
DEFINE FIELD IF NOT EXISTS prov         ON sim_loadout TYPE { by: string, method: "import" | "agent" | "rule" | "human" | "computed", run: option<string>, input: option<string>, at: datetime };
DEFINE FIELD IF NOT EXISTS ext          ON sim_loadout TYPE object FLEXIBLE DEFAULT {};
DEFINE INDEX IF NOT EXISTS sim_loadout_unique ON sim_loadout FIELDS era, side, system UNIQUE;

DEFINE TABLE IF NOT EXISTS sim_param SCHEMAFULL COMMENT "Scalar or distribution parameters per era/side (kill-zone depth, latency, rates)";
DEFINE FIELD IF NOT EXISTS key          ON sim_param TYPE string;        -- "kill_zone_depth"
DEFINE FIELD IF NOT EXISTS era          ON sim_param TYPE option<record<era>>;
DEFINE FIELD IF NOT EXISTS side         ON sim_param TYPE option<"ua" | "ru">;
DEFINE FIELD IF NOT EXISTS dist         ON sim_param TYPE { type: "point" | "uniform" | "triangular" | "normal" | "lognormal" | "table",
                                                           value: option<float>, min: option<float>, max: option<float>,
                                                           mode: option<float>, mean: option<float>, sd: option<float>,
                                                           table: option<array<object>> };
DEFINE FIELD IF NOT EXISTS unit         ON sim_param TYPE record<unit>;
DEFINE FIELD IF NOT EXISTS description  ON sim_param TYPE string;
DEFINE FIELD IF NOT EXISTS derived_from ON sim_param TYPE set<record<claim>> DEFAULT [];
DEFINE FIELD IF NOT EXISTS method       ON sim_param TYPE string;
DEFINE FIELD IF NOT EXISTS reviewed     ON sim_param TYPE bool DEFAULT false;
DEFINE FIELD IF NOT EXISTS prov         ON sim_param TYPE { by: string, method: "import" | "agent" | "rule" | "human" | "computed", run: option<string>, input: option<string>, at: datetime };
DEFINE FIELD IF NOT EXISTS ext          ON sim_param TYPE object FLEXIBLE DEFAULT {};
DEFINE INDEX IF NOT EXISTS sim_param_unique ON sim_param FIELDS key, era, side UNIQUE;

DEFINE TABLE IF NOT EXISTS sim_series SCHEMAFULL COMMENT "Time series for the game (KABs/month, Shaheds, territory change), one preferred claimant per series";
DEFINE FIELD IF NOT EXISTS key          ON sim_series TYPE string;
DEFINE FIELD IF NOT EXISTS metric       ON sim_series TYPE record<metric>;
DEFINE FIELD IF NOT EXISTS side         ON sim_series TYPE option<"ua" | "ru" | "western" | "other">;
DEFINE FIELD IF NOT EXISTS unit         ON sim_series TYPE record<unit>;
DEFINE FIELD IF NOT EXISTS points       ON sim_series TYPE array<{ t: datetime, precision: string, value: option<float>, low: option<float>, high: option<float>, claim: record<claim> }>;
DEFINE FIELD IF NOT EXISTS description  ON sim_series TYPE string;
DEFINE FIELD IF NOT EXISTS method       ON sim_series TYPE string;
DEFINE FIELD IF NOT EXISTS reviewed     ON sim_series TYPE bool DEFAULT false;
DEFINE FIELD IF NOT EXISTS prov         ON sim_series TYPE { by: string, method: "import" | "agent" | "rule" | "human" | "computed", run: option<string>, input: option<string>, at: datetime };
DEFINE FIELD IF NOT EXISTS ext          ON sim_series TYPE object FLEXIBLE DEFAULT {};
DEFINE INDEX IF NOT EXISTS sim_series_key ON sim_series FIELDS key UNIQUE;

DEFINE TABLE IF NOT EXISTS sim_terrain_profile SCHEMAFULL COMMENT "Procedural-generation parameters per landscape type (from measured sample boxes)";
DEFINE FIELD IF NOT EXISTS key          ON sim_terrain_profile TYPE string;
DEFINE FIELD IF NOT EXISTS place        ON sim_terrain_profile TYPE option<record<place>>;
DEFINE FIELD IF NOT EXISTS description  ON sim_terrain_profile TYPE string;
DEFINE FIELD IF NOT EXISTS params       ON sim_terrain_profile TYPE object FLEXIBLE;
DEFINE FIELD IF NOT EXISTS derived_from ON sim_terrain_profile TYPE set<record<claim>> DEFAULT [];
DEFINE FIELD IF NOT EXISTS method       ON sim_terrain_profile TYPE string;
DEFINE FIELD IF NOT EXISTS reviewed     ON sim_terrain_profile TYPE bool DEFAULT false;
DEFINE FIELD IF NOT EXISTS attribution  ON sim_terrain_profile TYPE option<string>;
DEFINE FIELD IF NOT EXISTS prov         ON sim_terrain_profile TYPE { by: string, method: "import" | "agent" | "rule" | "human" | "computed", run: option<string>, input: option<string>, at: datetime };
DEFINE FIELD IF NOT EXISTS ext          ON sim_terrain_profile TYPE object FLEXIBLE DEFAULT {};
DEFINE INDEX IF NOT EXISTS sim_terrain_key ON sim_terrain_profile FIELDS key UNIQUE;
