-- 0012_sim_dist_table: a "table" distribution may be a list of rows or a keyed mapping.
DEFINE FIELD OVERWRITE dist ON sim_param TYPE { type: "point" | "uniform" | "triangular" | "normal" | "lognormal" | "table",
                                                value: option<float>, min: option<float>, max: option<float>,
                                                mode: option<float>, mean: option<float>, sd: option<float>,
                                                table: option<array<object> | object> };
