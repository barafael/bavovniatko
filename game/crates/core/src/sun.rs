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

/// The moon over a night part whose file gives the date (`moon`), at this many minutes after that date's local
/// midnight: its elevation and azimuth (degrees, from north, clockwise) and how much of its disc is lit (0 new, 1
/// full). The Astronomical Almanac's low-precision series (a few tenths of a degree, parallax left out): enough to
/// know whether it is up and how bright. None where the file gives no date.
pub fn moon(minutes: f32) -> Option<(f32, f32, f32)> {
    let d = crate::data::data();
    Some(moon_at(d.moon?, minutes, d.sun.map_or(2.0, |s| s.1), (d.anchor.lat, d.anchor.lon)))
}

/// The moon on a date at a place: `moon`'s sums.
fn moon_at((year, month, day): (i32, u32, u32), minutes: f32, utc: f32, (lat, lon_e): (f64, f64)) -> (f32, f32, f32) {
    let utc = utc as f64;
    // Days from J2000.0 (2000-01-01 12:00 UT) to the moment.
    let (y, m) = if month <= 2 { (year as f64 - 1.0, month as f64 + 12.0) } else { (year as f64, month as f64) };
    let a = (y / 100.0).floor();
    let jd0 = (365.25 * (y + 4716.0)).floor() + (30.6001 * (m + 1.0)).floor() + day as f64 + 2.0 - a + (a / 4.0).floor() - 1524.5;
    let days = jd0 + (minutes as f64 / 60.0 - utc) / 24.0 - 2451545.0;
    let t = days / 36525.0;
    let s = |deg: f64| deg.to_radians().sin();
    // Ecliptic longitude and latitude.
    let ecl_lon = 218.32 + 481267.881 * t + 6.29 * s(134.9 + 477198.85 * t) - 1.27 * s(259.2 - 413335.38 * t) + 0.66 * s(235.7 + 890534.23 * t) + 0.21 * s(269.9 + 954397.70 * t)
        - 0.19 * s(357.5 + 35999.05 * t)
        - 0.11 * s(186.6 + 966404.05 * t);
    let ecl_lat = 5.13 * s(93.3 + 483202.03 * t) + 0.28 * s(228.2 + 960400.87 * t) - 0.28 * s(318.3 + 6003.18 * t) - 0.17 * s(217.6 - 407332.20 * t);
    let (l, b, eps) = (ecl_lon.to_radians(), ecl_lat.to_radians(), 23.44f64.to_radians());
    let (x, yy, z) = (b.cos() * l.cos(), eps.cos() * b.cos() * l.sin() - eps.sin() * b.sin(), eps.sin() * b.cos() * l.sin() + eps.cos() * b.sin());
    let (ra, dec) = (yy.atan2(x), z.asin());
    // The hour angle from the sidereal time at the place.
    let gmst = 280.46061837 + 360.98564736629 * days + lon_e;
    let ha = (gmst.to_radians() - ra).rem_euclid(std::f64::consts::TAU);
    let phi = lat.to_radians();
    let elev = (phi.sin() * dec.sin() + phi.cos() * dec.cos() * ha.cos()).asin();
    let az = (-ha.sin()).atan2(dec.tan() * phi.cos() - phi.sin() * ha.cos()).to_degrees().rem_euclid(360.0);
    // The lit share from the elongation from the sun.
    let g = (357.528 + 0.9856003 * days).to_radians();
    let sun_lon = (280.46 + 0.9856474 * days + 1.915 * g.sin() + 0.02 * (2.0 * g).sin()).to_radians();
    let lit = (1.0 - b.cos() * (l - sun_lon).cos()) * 0.5;
    (elev.to_degrees() as f32, az as f32, lit as f32)
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

#[cfg(test)]
mod tests {
    /// The moon over Myrnohrad: full on 5 November 2025 (13:19 UT) and high at local midnight; new on 18 January
    /// 2026 (19:52 UT), so on the night of 19-20 January a thin crescent, set before 22:00 and not up again by 04:00.
    #[test]
    fn the_moon_over_myrnohrad() {
        let at = (48.302, 37.248);
        let (elev, _, lit) = super::moon_at((2025, 11, 5), 24.0 * 60.0, 2.0, at);
        assert!(lit > 0.98 && elev > 40.0, "full moon: lit {lit}, elevation {elev}");
        for h in [22.0, 24.0, 26.0, 28.0] {
            let (elev, _, lit) = super::moon_at((2026, 1, 19), h * 60.0, 2.0, at);
            assert!(lit < 0.08 && elev < 0.0, "at {h} h: lit {lit}, elevation {elev}");
        }
        let (elev, _, _) = super::moon_at((2026, 1, 19), 16.0 * 60.0, 2.0, at);
        assert!(elev > 0.0, "the crescent is up in the afternoon: {elev}");
    }
}
