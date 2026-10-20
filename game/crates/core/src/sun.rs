//! The sun over a diorama whose file gives its day (`sun`): where it is, and how much light there is to see by
//! (design/hostomel.md, phase 6). A place without it keeps its own hand-made light, and the battle ignores the dark.

/// The sun at the anchor's latitude and longitude on the diorama's day (`sun` in its file), at this many minutes
/// after local midnight: elevation and azimuth from north, clockwise, in degrees. NOAA's low-precision formulae:
/// good to a fraction of a degree, which is all a battlefield needs.
pub fn position(minutes: f32) -> (f32, f32) {
    let d = crate::data::data();
    let (doy, utc) = d.sun.unwrap_or((172, 2.0));
    let (lat, lon) = (d.anchor.lat as f32, d.anchor.lon as f32);
    let g = std::f32::consts::TAU / 365.0 * (doy as f32 - 1.0 + (minutes / 60.0 - utc - 12.0) / 24.0);
    let eqtime = 229.18 * (0.000075 + 0.001868 * g.cos() - 0.032077 * g.sin() - 0.014615 * (2.0 * g).cos() - 0.040849 * (2.0 * g).sin());
    let decl = 0.006918 - 0.399912 * g.cos() + 0.070257 * g.sin() - 0.006758 * (2.0 * g).cos() + 0.000907 * (2.0 * g).sin() - 0.002697 * (3.0 * g).cos() + 0.00148 * (3.0 * g).sin();
    let true_solar = minutes + eqtime + 4.0 * lon - 60.0 * utc;
    let ha = (true_solar / 4.0 - 180.0).to_radians();
    let phi = lat.to_radians();
    let cos_zen = (phi.sin() * decl.sin() + phi.cos() * decl.cos() * ha.cos()).clamp(-1.0, 1.0);
    let zen = cos_zen.acos();
    let elev = 90.0 - zen.to_degrees();
    let az = ((phi.sin() * cos_zen - decl.sin()) / (phi.cos() * zen.sin()).max(1e-6)).clamp(-1.0, 1.0).acos().to_degrees();
    let azim = if ha > 0.0 { (az + 180.0) % 360.0 } else { (540.0 - az) % 360.0 };
    (elev, azim)
}

/// How far men see, as a share of the day's: full in daylight, falling through dusk to about a third in the night
/// (flares, fires and the glow of the town). Always 1 where the file gives no sun.
pub fn sight(minutes: f32) -> f32 {
    if crate::data::data().sun.is_none() {
        return 1.0;
    }
    let (elev, _) = position(minutes);
    0.35 + 0.65 * crate::geo::smoothstep(-12.0, 2.0, elev)
}
