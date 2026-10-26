#!/usr/bin/env python3
"""OpenStreetMap to a diorama's ground: the roads, taxiways, railway, tree rows, land cover and buildings inside
the box, as RON fields for `bav_core::data` (design/hostomel.md, phase 1).

    tools/osm/fetch.sh hostomel              # the Overpass extract, into tools/osm/data/hostomel.json
    python3 tools/osm/osm2ron.py hostomel    # writes dioramas/hostomel/osm.ron, formatted with fmtron

The data is © OpenStreetMap contributors (ODbL); the extract and what is made from it carry that credit
(dioramas/hostomel/CREDITS.md). OSM is the map as it is now: what was built after the day (housing estates
put up since 2022) is dropped by name below, and the rest is checked against imagery of the time by hand.

The box is the game's fixed 4608 x 3584 m (`geo::HALF_X`, `geo::HALF_Z`), centred on the place's `centre`
and turned so that +z points along its `bearing` (180, south, keeps +x east; Avdiivka's 45 faces the attack
from the north-east, `Anchor::bearing`). Shapes are simplified to about a metre and clipped to the box.
"""
import json
import math
import shutil
import subprocess
import sys
from pathlib import Path

HALF_X, HALF_Z = 2304.0, 1792.0

PLACES = {
    # The runway's middle, and the box's centre from it: 500 m east and 700 m south, so the apron, the Antonov
    # hangar, the National Guard's base and the flats by the gate are inside, and only the runway's far north end
    # is cut.
    "hostomel": dict(lat0=50.6036, lon0=30.1912, east=500.0, south=700.0, bearing=180.0),
    # The northern shoulder of the Avdiivka salient (design/avdiivka.md): the box's centre between Stepove and the
    # spoil heap, turned so the Russian columns from Krasnohorivka and Kamianka come down +z.
    "avdiivka": dict(lat0=48.1898, lon0=37.6998, east=0.0, south=0.0, bearing=75.0),
    # The road into Myrnohrad (design/myrnohrad.md): the box's centre west of the town, turned to 150 so +z points
    # south-south-east at the railway and their side, the road from Rivne and Svitle coming in at -x.
    # Its buildings are mostly untagged, so they are given storeys by their footprint (`by_footprint`).
    "myrnohrad": dict(lat0=48.302, lon0=37.248, east=0.0, south=0.0, bearing=150.0, untagged="mining town"),
}

# Built after 24 February 2022 (the names say so): not on the day's ground.
AFTER = ("ЖК", "перспектива", "Brickwood", "Cherry House")

GAME = Path(__file__).resolve().parents[2]


def main(place: str) -> None:
    cfg = PLACES[place]
    lat0, lon0 = cfg["lat0"], cfg["lon0"]
    m_lat = 111320.0
    m_lon = 111320.0 * math.cos(math.radians(lat0))
    ox, oz = cfg["east"], cfg["south"]
    brg = math.radians(cfg["bearing"])
    # The box's centre, in metres east and north of (lat0, lon0): `east` and `south` are given in the map's
    # frame, so they turn with it.
    ce, cn = -ox * math.cos(brg) + oz * math.sin(brg), ox * math.sin(brg) + oz * math.cos(brg)

    def xz(p):
        # Metres east and north of the centre, then onto the map's axes (`geo::map_offset`).
        e = (p["lon"] - lon0) * m_lon - ce
        n = (p["lat"] - lat0) * m_lat - cn
        return (-e * math.cos(brg) + n * math.sin(brg), e * math.sin(brg) + n * math.cos(brg))

    elements = json.loads((GAME / "tools/osm/data" / f"{place}.json").read_text())["elements"]
    # The works' grounds, for the untagged buildings of a mining town: a long shed there is a workshop, not flats.
    works = [
        [xz(q) for q in e["geometry"]]
        for e in elements
        if e.get("geometry") and (e.get("tags", {}).get("landuse") == "industrial" or e.get("tags", {}).get("man_made") == "works")
    ]
    lines, roads, rails, belts, areas, buildings, runways = [], [], [], [], [], [], []
    for e in elements:
        t = e.get("tags", {})
        g = e.get("geometry")
        if not g:
            continue
        pts = [xz(q) for q in g]
        closed = len(g) > 3 and g[0] == g[-1]
        name = t.get("name", "")
        if any(a in name for a in AFTER):
            continue
        if "building" in t and closed:
            foot = clip_poly(simplify(pts[:-1], 0.5))
            if len(foot) >= 3:
                h, kind = height(t, area(foot)), building_kind(t)
                if cfg.get("untagged") == "mining town" and untagged(t):
                    h, kind = by_footprint(foot, any(point_in(c, w) for w in works for c in [centroid(foot)]))
                buildings.append((foot, h, kind))
            continue
        cover = cover_kind(t)
        if cover and closed:
            poly = clip_poly(simplify(pts[:-1], 1.0))
            if len(poly) >= 3 and area(poly) > 50.0:
                areas.append((cover, poly))
            continue
        way = t.get("aeroway")
        hw = t.get("highway")
        if way == "runway":
            # The strip itself as a polygon: the line widened to its width.
            width = number(t.get("width"), 50.0)
            areas.append(("Runway", clip_poly(widen(pts, width))))
            # Its whole frame, ends and width, unclipped: the markings are measured from its real ends.
            runways.append((pts[0], pts[-1], width))
            continue
        for part in clip_line(simplify(pts, 1.0)):
            if len(part) < 2:
                continue
            if way in ("taxiway", "taxilane"):
                lines.append(part)
                roads.append((len(lines) - 1, 22.0 if way == "taxiway" else 14.0, "Concrete"))
            elif hw in ROAD:
                lines.append(part)
                width, kind = ROAD[hw]
                roads.append((len(lines) - 1, width, kind))
            elif t.get("railway") in ("rail", "disused"):
                # (Disused on today's map: the war's; the sidings and the line past Stepove were there on the day.)
                lines.append(part)
                rails.append(len(lines) - 1)
            elif t.get("natural") == "tree_row":
                # (A closed row simplifies to a point: nothing to plant.)
                for a, b in zip(part, part[1:]):
                    if math.dist(a, b) > 1.0:
                        belts.append((a, b))

    out = [
        f"// Generated by tools/osm/osm2ron.py from tools/osm/data/{place}.json (© OpenStreetMap contributors, ODbL).",
        "// Do not edit: change the script, or the hand-written map.ron beside it.",
        "(",
        f"    anchor: (lat: {lat0 + cn / m_lat:.6f}, lon: {lon0 + ce / m_lon:.6f}, x: 0.0, z: 0.0, m_per_lon: {m_lon:.1f}, bearing: {cfg['bearing']:.1f}),",
        "    osm: (",
        "    lines: [",
    ]
    for i, l in enumerate(lines):
        out.append(f'        ("osm{i}", Points([{pts_ron(l)}])),')
    out.append("    ],")
    out.append("    roads: [")
    for i, w, k in roads:
        out.append(f'        (line: "osm{i}", width: {w:.1f}, kind: {k}),')
    out.append("    ],")
    out.append("    rails: [" + ", ".join(f'"osm{i}"' for i in rails) + "],")
    out.append("    runways: [" + ", ".join(f"({p(a)}, {p(b)}, {w:.1f})" for a, b, w in runways) + "],")
    out.append("    belts: [Lines(lines: [")
    for a, b in belts:
        out.append(f"        ({p(a)}, {p(b)}),")
    out.append("    ], width: 7.0)],")
    out.append("    areas: [")
    for k, poly in sorted(areas, key=lambda a: AREA_ORDER.index(a[0])):
        out.append(f"        ({k}, [{pts_ron(poly)}]),")
    out.append("    ],")
    out.append("    buildings: [")
    for foot, h, k in buildings:
        out.append(f"        (kind: {k}, height: {h:.1f}, foot: [{pts_ron(foot)}]),")
    out.append("    ],")
    out.append("    ),")
    out.append(")")
    dest = GAME / "dioramas" / place / "osm.ron"
    dest.parent.mkdir(parents=True, exist_ok=True)
    dest.write_text("\n".join(out) + "\n")
    # Laid out as every RON file of the game is (fmtron, with game/fmt.ron's width).
    fmtron = shutil.which("fmtron")
    if fmtron:
        subprocess.run([fmtron, str(dest)], check=True)
    else:
        print("fmtron not found (cargo install fmtron --version 1.0.0-rc.2): osm.ron left unformatted", file=sys.stderr)
    print(f"{dest}: {len(lines)} lines, {len(roads)} roads, {len(rails)} rails, {len(belts)} belt segments, {len(areas)} areas, {len(buildings)} buildings")


# highway=* -> (width in metres, RoadKind)
ROAD = {
    "motorway": (14.0, "Asphalt"),
    "trunk": (11.0, "Asphalt"),
    "primary": (9.0, "Asphalt"),
    "secondary": (8.0, "Asphalt"),
    "tertiary": (7.0, "Asphalt"),
    "unclassified": (5.5, "Asphalt"),
    "residential": (5.5, "Asphalt"),
    "service": (4.5, "Dirt"),
    "track": (3.5, "Track"),
}

# Painted in this order, so later kinds lie over earlier ones.
AREA_ORDER = ["Farmland", "Grass", "Allotments", "Residential", "Industrial", "Spoil", "Wood", "Wetland", "Water", "Apron", "Runway"]


def cover_kind(t):
    v = t.get("natural") or t.get("landuse") or t.get("aeroway") or t.get("man_made")
    return {
        "farmland": "Farmland",
        "grass": "Grass",
        "grassland": "Grass",
        "meadow": "Grass",
        "scrub": "Grass",
        "orchard": "Allotments",
        "allotments": "Allotments",
        "residential": "Residential",
        "industrial": "Industrial",
        "garages": "Industrial",
        "construction": "Industrial",
        "farmyard": "Industrial",
        "water_works": "Industrial",
        "wastewater_plant": "Industrial",
        "storage_tank": "Industrial",
        "cemetery": "Grass",
        "recreation_ground": "Grass",
        "beach": "Grass",
        # A mine's or a works' spoil heap, and a tip: bare black ground.
        "spoil_heap": "Spoil",
        "landfill": "Spoil",
        "wood": "Wood",
        "forest": "Wood",
        "wetland": "Wetland",
        "water": "Water",
        "apron": "Apron",
        # The airfield's ground between the strips: grass.
        "aerodrome": "Grass",
    }.get(v)


def building_kind(t):
    b = t.get("building")
    if t.get("aeroway") == "hangar" or b == "hangar":
        return "Hangar"
    return {
        "house": "House",
        "detached": "House",
        "residential": "House",
        "apartments": "Block",
        "dormitory": "Block",
        "industrial": "Industrial",
        "warehouse": "Industrial",
        "cowshed": "Industrial",
        "service": "Industrial",
        "shed": "Shed",
        "garage": "Shed",
        "greenhouse": "Shed",
        "kiosk": "Shed",
    }.get(b, "Other")


def number(v, default):
    """A tag's number, whatever unit it is written with ("50", "50 m")."""
    try:
        return float(str(v).split()[0])
    except (ValueError, IndexError):
        return default


def height(t, a):
    if "height" in t:
        try:
            return float(t["height"].split()[0])
        except ValueError:
            pass
    if "building:levels" in t:
        try:
            return float(t["building:levels"]) * 3.0 + 1.2
        except ValueError:
            pass
    if t.get("aeroway") == "hangar":
        return 30.0
    b = t.get("building")
    if b == "apartments":
        return 16.2
    if b in ("shed", "garage", "kiosk", "greenhouse"):
        return 3.2
    if b in ("house", "detached", "residential"):
        return 6.5
    if b == "ruins":
        return 3.0
    # Untagged: the big ones are the airfield's workshops and stores.
    return 10.0 if a > 1500 else 7.0 if a > 300 else 5.0


def untagged(t):
    """A building the map gives no kind of use and no height."""
    return t.get("building") == "yes" and "building:levels" not in t and "height" not in t


def by_footprint(foot, in_works):
    """A mining town's untagged building, by its footprint (design/myrnohrad.md): in the works' grounds a workshop;
    elsewhere a long narrow one of 450-2500 m² is a five-storey block of flats (the Khrushchyovka's 12 by 60-80 m),
    a small one a house of one storey or two, a big squat one a school, a shop or a works."""
    a = area(foot)
    long, short = extent(foot)
    if in_works:
        return (10.0 if a > 1500 else 7.0), "Industrial"
    if 450 <= a <= 2500 and long >= 2.5 * short:
        return 16.2, "Block"
    if a < 160:
        return 4.2, "House"
    if a < 400:
        return 7.2, "House"
    return 10.0, "Other"


def extent(foot):
    """The footprint's length and width along its longest edge."""
    best, ax = 0.0, (1.0, 0.0)
    for (x1, z1), (x2, z2) in zip(foot, foot[1:] + foot[:1]):
        d = math.hypot(x2 - x1, z2 - z1)
        if d > best:
            best, ax = d, ((x2 - x1) / d, (z2 - z1) / d)
    u = [x * ax[0] + z * ax[1] for x, z in foot]
    v = [-x * ax[1] + z * ax[0] for x, z in foot]
    return max(u) - min(u), max(v) - min(v)


def centroid(pts):
    return (sum(q[0] for q in pts) / len(pts), sum(q[1] for q in pts) / len(pts))


def point_in(c, poly):
    inside_ = False
    for (x1, z1), (x2, z2) in zip(poly, poly[1:] + poly[:1]):
        if (z1 > c[1]) != (z2 > c[1]) and c[0] < (x2 - x1) * (c[1] - z1) / (z2 - z1) + x1:
            inside_ = not inside_
    return inside_


def area(pts):
    s = 0.0
    for i in range(len(pts)):
        (x1, z1), (x2, z2) = pts[i - 1], pts[i]
        s += x1 * z2 - x2 * z1
    return abs(s) / 2


def widen(pts, w):
    (x0, z0), (x1, z1) = pts[0], pts[-1]
    dx, dz = x1 - x0, z1 - z0
    n = math.hypot(dx, dz)
    nx, nz = -dz / n * w / 2, dx / n * w / 2
    return [(x0 + nx, z0 + nz), (x1 + nx, z1 + nz), (x1 - nx, z1 - nz), (x0 - nx, z0 - nz)]


def simplify(pts, eps):
    """Douglas-Peucker."""
    if len(pts) < 3:
        return pts
    (ax, az), (bx, bz) = pts[0], pts[-1]
    dx, dz = bx - ax, bz - az
    n = math.hypot(dx, dz) or 1e-9
    best, k = 0.0, 0
    for i in range(1, len(pts) - 1):
        d = abs((pts[i][0] - ax) * dz - (pts[i][1] - az) * dx) / n
        if d > best:
            best, k = d, i
    if best <= eps:
        return [pts[0], pts[-1]]
    return simplify(pts[: k + 1], eps)[:-1] + simplify(pts[k:], eps)


def inside(p):
    return abs(p[0]) <= HALF_X and abs(p[1]) <= HALF_Z


def clip_line(pts):
    """The runs of a line inside the box (Liang-Barsky, segment by segment)."""
    parts, cur = [], []
    for a, b in zip(pts, pts[1:]):
        seg = clip_segment(a, b)
        if seg is None:
            if cur:
                parts.append(cur)
                cur = []
            continue
        s, e = seg
        if cur and cur[-1] == s:
            cur.append(e)
        else:
            if cur:
                parts.append(cur)
            cur = [s, e]
    if cur:
        parts.append(cur)
    return parts


def clip_segment(a, b):
    t0, t1 = 0.0, 1.0
    dx, dz = b[0] - a[0], b[1] - a[1]
    for pp, q in ((-dx, a[0] + HALF_X), (dx, HALF_X - a[0]), (-dz, a[1] + HALF_Z), (dz, HALF_Z - a[1])):
        if pp == 0:
            if q < 0:
                return None
            continue
        r = q / pp
        if pp < 0:
            t0 = max(t0, r)
        else:
            t1 = min(t1, r)
        if t0 > t1:
            return None
    s = a if t0 == 0.0 else (a[0] + t0 * dx, a[1] + t0 * dz)
    e = b if t1 == 1.0 else (a[0] + t1 * dx, a[1] + t1 * dz)
    return s, e


def clip_poly(pts):
    """Sutherland-Hodgman against the box."""
    for ax, lim, sgn in ((0, HALF_X, 1), (0, -HALF_X, -1), (1, HALF_Z, 1), (1, -HALF_Z, -1)):
        out = []
        ins = lambda p: (p[ax] - lim) * sgn <= 0
        for i in range(len(pts)):
            a, b = pts[i - 1], pts[i]
            if ins(b):
                if not ins(a):
                    out.append(cross(a, b, ax, lim))
                out.append(b)
            elif ins(a):
                out.append(cross(a, b, ax, lim))
        pts = out
        if not pts:
            return []
    return pts


def cross(a, b, ax, lim):
    t = (lim - a[ax]) / (b[ax] - a[ax])
    return (a[0] + t * (b[0] - a[0]), a[1] + t * (b[1] - a[1]))


def p(q):
    return f"({q[0]:.1f}, {q[1]:.1f})"


def pts_ron(pts):
    return ", ".join(p(q) for q in pts)


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else "hostomel")
