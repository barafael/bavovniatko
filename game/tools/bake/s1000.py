#!/usr/bin/env python3
"""Assemble idmental's "Octocopter (DJI S1000 parts)" into the `vampire`: the heavy night bomber's stand-in.

The download (`db/research/assets/files/drones/drones_baba-yaga-proxy_idmental/`) is a parts sheet in
centimetres: the two octagonal centre plates, one arm (hinge clamp, tube and motor), the landing-gear mount, all
lying apart. This puts the plates together, the arm eight times round them (an S1000's 1045 mm motor to motor),
and dresses it as the Baba Yaga the plan asks for (`game/design/myrnohrad.md`): eight 15-inch props, a battery on
top, four legs, a TM-62 anti-tank mine under the belly on the gear mount's rails, a camera ball forward, all in
the dark of a night bomber. The result is written next to the source as `vampire/scene.gltf` (+ `scene.bin`,
and the source's `meta.yaml` for the credit), in the game's frame: metres, +Y up, the front to -Z.

    python3 -I game/tools/bake/s1000.py && (cd game/tools/bake && cargo run --release -- vampire)

Only numpy.
"""
import json
import os
import sys

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
SRC = os.path.join(HERE, "../../../db/research/assets/files/drones/drones_baba-yaga-proxy_idmental")
OUT = os.path.join(SRC, "vampire")

# The S1000's 15-inch props. Its 1045 mm motor to motor comes out of the parts as they are (106 cm).
PROP_D = 38.1

CT = {5120: "b", 5121: "B", 5122: "h", 5123: "H", 5125: "I", 5126: "f"}
NC = {"SCALAR": 1, "VEC2": 2, "VEC3": 3, "VEC4": 4, "MAT4": 16}

# Source material -> ours: a night bomber is dark all over; the anodised metal stays a shade lighter.
FRAME, METAL, PROP, OLIVE = 0, 1, 2, 3
MATERIALS = [
    ("frame", [0.07, 0.07, 0.075, 1.0]),
    ("metal", [0.22, 0.22, 0.23, 1.0]),
    ("prop", [0.035, 0.035, 0.035, 1.0]),
    ("olive", [0.24, 0.27, 0.15, 1.0]),
]


def ours(name):
    return METAL if name.startswith("Metal") else FRAME


def load(path):
    """Every triangle primitive in world space: (node path, positions, normals, triangles, material name)."""
    g = json.load(open(path))
    d = os.path.dirname(path)
    bufs = [open(os.path.join(d, b["uri"]), "rb").read() for b in g["buffers"]]

    def acc(i):
        a = g["accessors"][i]
        bv = g["bufferViews"][a["bufferView"]]
        c, dt = NC[a["type"]], np.dtype(CT[a["componentType"]])
        off = bv.get("byteOffset", 0) + a.get("byteOffset", 0)
        stride = bv.get("byteStride", 0) or dt.itemsize * c
        raw = np.frombuffer(bufs[bv["buffer"]], np.uint8, stride * (a["count"] - 1) + dt.itemsize * c, off)
        rows = np.lib.stride_tricks.as_strided(raw, (a["count"], dt.itemsize * c), (stride, 1))
        return np.ascontiguousarray(rows).view(dt).reshape(a["count"], c)

    def local(n):
        if "matrix" in n:
            return np.array(n["matrix"], float).reshape(4, 4).T
        assert not any(k in n for k in ("translation", "rotation", "scale")), "the parts sheet uses matrices only"
        return np.eye(4)

    out = []

    def walk(i, parent, path):
        n = g["nodes"][i]
        w = parent @ local(n)
        path = path + [n.get("name", "")]
        if "mesh" in n:
            for p in g["meshes"][n["mesh"]]["primitives"]:
                if p.get("mode", 4) != 4:
                    continue
                pos = acc(p["attributes"]["POSITION"]).astype(float)
                nrm = acc(p["attributes"]["NORMAL"]).astype(float)
                pos = pos @ w[:3, :3].T + w[:3, 3]
                nrm = nrm @ np.linalg.inv(w[:3, :3])
                nrm /= np.maximum(np.linalg.norm(nrm, axis=1, keepdims=True), 1e-12)
                idx = acc(p["indices"]).reshape(-1, 3).astype(np.uint32)
                if np.linalg.det(w[:3, :3]) < 0:
                    idx = idx[:, ::-1]
                out.append((path, pos, nrm, idx, g["materials"][p["material"]]["name"]))
        for c in n.get("children", []):
            walk(c, w, path)

    for r in g["scenes"][g.get("scene", 0)]["nodes"]:
        walk(r, np.eye(4), [])
    return out


def pick(parts, *names):
    return [p for p in parts if any(n in p[0] for n in names)]


def bounds(parts):
    v = np.concatenate([p[1] for p in parts])
    return v.min(0), v.max(0)


def centre(parts):
    lo, hi = bounds(parts)
    return (lo + hi) / 2


def rot_y(a):
    c, s = np.cos(a), np.sin(a)
    return np.array([[c, 0, s], [0, 1, 0], [-s, 0, c]])


def rot_between(a, b):
    """The rotation taking unit vector a onto unit vector b."""
    a, b = a / np.linalg.norm(a), b / np.linalg.norm(b)
    v, c = np.cross(a, b), float(a @ b)
    if np.linalg.norm(v) < 1e-9:
        return np.eye(3)
    k = np.array([[0, -v[2], v[1]], [v[2], 0, -v[0]], [-v[1], v[0], 0]])
    return np.eye(3) + k + k @ k / (1 + c)


def axis(v):
    """The principal axis of a point cloud."""
    v = v - v.mean(0)
    return np.linalg.svd(v, full_matrices=False)[2][0]


def thin(v):
    """The direction a point cloud is thinnest along."""
    v = v - v.mean(0)
    return np.linalg.svd(v, full_matrices=False)[2][2]


class Mesh:
    """What we write: per material, positions, normals and triangles."""

    def __init__(self):
        self.parts = {m: ([], [], []) for m in range(len(MATERIALS))}

    def add(self, mat, pos, nrm, idx):
        p, n, t = self.parts[mat]
        base = sum(len(x) for x in p)
        p.append(np.asarray(pos, np.float32))
        n.append(np.asarray(nrm, np.float32))
        t.append(np.asarray(idx, np.uint32) + base)

    def place(self, parts, r, t):
        """Source parts rotated by r (about the origin) and moved by t."""
        for _, pos, nrm, idx, name in parts:
            self.add(ours(name), pos @ r.T + t, nrm @ r.T, idx)

    def box(self, mat, c, size, r=np.eye(3)):
        h = np.asarray(size, float) / 2
        for ax in range(3):
            for sgn in (-1, 1):
                n = np.zeros(3)
                n[ax] = sgn
                u, v = np.zeros(3), np.zeros(3)
                u[(ax + 1) % 3], v[(ax + 2) % 3] = 1, 1
                if sgn < 0:
                    u, v = v, u
                q = [n * h + (a * u + b * v) * h for a, b in ((-1, -1), (1, -1), (1, 1), (-1, 1))]
                self.add(mat, [r @ x + c for x in q], [r @ n] * 4, [[0, 1, 2], [0, 2, 3]])

    def cylinder(self, mat, a, b, rad, seg=12, caps=True):
        """A cylinder from point a to point b."""
        a, b = np.asarray(a, float), np.asarray(b, float)
        d = (b - a) / np.linalg.norm(b - a)
        u = np.cross(d, [0, 1, 0] if abs(d[1]) < 0.9 else [1, 0, 0])
        u /= np.linalg.norm(u)
        v = np.cross(d, u)
        ring = [np.cos(2 * np.pi * k / seg) * u + np.sin(2 * np.pi * k / seg) * v for k in range(seg)]
        pos = [a + rad * r for r in ring] + [b + rad * r for r in ring]
        tri = []
        for k in range(seg):
            k1 = (k + 1) % seg
            tri += [[k, k1, seg + k1], [k, seg + k1, seg + k]]
        self.add(mat, pos, ring + ring, tri)
        if caps:
            for end, n, flip in ((a, -d, True), (b, d, False)):
                pos = [end] + [end + rad * r for r in ring]
                tri = [[0, 1 + (k + 1) % seg, 1 + k] if flip else [0, 1 + k, 1 + (k + 1) % seg] for k in range(seg)]
                self.add(mat, pos, [n] * (seg + 1), tri)

    def sphere(self, mat, c, rad, seg=10):
        pos, nrm, tri = [], [], []
        for i in range(seg + 1):
            th = np.pi * i / seg
            for j in range(seg * 2):
                ph = np.pi * j / seg
                n = np.array([np.sin(th) * np.cos(ph), np.cos(th), np.sin(th) * np.sin(ph)])
                pos.append(c + rad * n)
                nrm.append(n)
        w = seg * 2
        for i in range(seg):
            for j in range(w):
                a, b = i * w + j, i * w + (j + 1) % w
                tri += [[a, b, b + w], [a, b + w, a + w]]
        self.add(mat, pos, nrm, tri)

    def write(self, out):
        os.makedirs(out, exist_ok=True)
        blob = bytearray()
        views, accs, prims = [], [], []

        def put(arr, target, kind, minmax=False):
            while len(blob) % 4:
                blob.append(0)
            views.append({"buffer": 0, "byteOffset": len(blob), "byteLength": arr.nbytes, "target": target})
            blob.extend(arr.tobytes())
            a = {"bufferView": len(views) - 1, "componentType": 5126 if arr.dtype == np.float32 else 5125,
                 "count": len(arr) if arr.ndim > 1 else arr.size, "type": kind}
            if minmax:
                a["min"], a["max"] = arr.min(0).tolist(), arr.max(0).tolist()
            accs.append(a)
            return len(accs) - 1

        for m, (p, n, t) in self.parts.items():
            if not p:
                continue
            p, n, t = np.concatenate(p), np.concatenate(n), np.concatenate(t)
            prims.append({"attributes": {"POSITION": put(p, 34962, "VEC3", True), "NORMAL": put(n, 34962, "VEC3")},
                          "indices": put(t.ravel(), 34963, "SCALAR"), "material": m})
        g = {
            "asset": {"version": "2.0", "generator": "game/tools/bake/s1000.py"},
            "scene": 0,
            "scenes": [{"nodes": list(range(len(prims)))}],
            "nodes": [{"name": MATERIALS[p["material"]][0], "mesh": i} for i, p in enumerate(prims)],
            "meshes": [{"primitives": [p]} for p in prims],
            "materials": [{"name": nm, "pbrMetallicRoughness": {"baseColorFactor": c, "metallicFactor": 0.0, "roughnessFactor": 0.8}} for nm, c in MATERIALS],
            "buffers": [{"uri": "scene.bin", "byteLength": len(blob)}],
            "bufferViews": views,
            "accessors": accs,
        }
        open(os.path.join(out, "scene.bin"), "wb").write(blob)
        json.dump(g, open(os.path.join(out, "scene.gltf"), "w"), indent=1)
        return sum(len(np.concatenate(t)) for p, n, t in self.parts.values() if t)


def main():
    parts = load(os.path.join(SRC, "scene.gltf"))
    lower = pick(parts, "Mesh1")
    upper = pick(parts, "Group21")  # the plate with its eight arm markers
    arm = pick(parts, "Group26", "Group27")  # hinge clamp, tube, motor
    motor = pick(parts, "Group31", "Group36")
    hinge = pick(parts, "Gruppe_10_1")
    tube = [p for p in pick(parts, "Mesh50") if p[4] == "CarbonTe"]
    rails = pick(parts, "Group2")  # the landing-gear mount: a plate, four clamps, two rails
    markers = pick(parts, *[f"M{k}_1" for k in range(1, 9)])

    # The plates: centred, flat, the lower one at y = 0, the upper over the hinges.
    lo_l, hi_l = bounds(lower)
    up_c = centre(pick(parts, "Group22"))
    m = Mesh()
    hinge_lo, hinge_hi = bounds(hinge)
    gap = hinge_hi[1] - hinge_lo[1]
    m.place(lower, np.eye(3), -np.array([(lo_l[0] + hi_l[0]) / 2, hi_l[1], (lo_l[2] + hi_l[2]) / 2]))
    up_lo, _ = bounds(upper)
    m.place(upper, np.eye(3), np.array([-up_c[0], gap - up_lo[1], -up_c[2]]))

    # The arm directions: from the plate's centre to its markers.
    dirs = sorted(((centre([mk]) - up_c) for mk in markers), key=lambda d: np.arctan2(d[2], d[0]))
    hinge_r = float(np.mean([np.hypot(d[0], d[2]) for d in dirs]))

    # One arm, made level: the tube's axis horizontal (pointing out along +X), the motor's disc flat.
    mc, hc = centre(motor), centre(hinge)
    out_dir = axis(np.concatenate([p[1] for p in tube]))
    if out_dir @ (mc - hc) < 0:
        out_dir = -out_dir
    level = rot_between(out_dir, np.array([1.0, 0, 0]))
    bell = max(motor, key=lambda p: len(p[1]))  # the motor's bell, not its screws
    up = level @ thin(bell[1])
    if up[1] < 0:
        up = -up
    # Turn about the tube so the motor's disc lies flat.
    a = np.arctan2(up[2], up[1])
    c, s = np.cos(a), np.sin(a)
    roll = np.array([[1, 0, 0], [0, c, s], [0, -s, c]])
    level = roll @ level
    up = level @ thin(bell[1])
    print(f"tube tilted {np.degrees(np.arcsin(abs(out_dir[1]))):.1f} deg on the sheet; the motor's axis {np.degrees(np.arccos(abs(up[1]))):.1f} deg off vertical on the level arm")
    reach = float((level @ (mc - hc))[0])
    print(f"hinge to motor {reach:.1f} cm; hinge {hinge_r:.1f} cm out: motor to motor {2 * (hinge_r + reach):.0f} cm (S1000 104.5)")
    hinge_y = gap / 2
    for d in dirs:
        th = np.arctan2(d[2], d[0])
        r = rot_y(-th) @ level
        t = np.array([np.cos(th), 0, np.sin(th)]) * hinge_r + np.array([0, hinge_y, 0]) - r @ hc
        m.place(arm, r, t)
        mpos = r @ mc + t
        mtop = max(float((p[1] @ r.T + t)[:, 1].max()) for p in motor)
        # A two-blade 15-inch prop over the motor, its blades at a turn of their own.
        blade = rot_y(th * 3.1 + 0.4)
        m.box(PROP, np.array([mpos[0], mtop + 0.6, mpos[2]]), [PROP_D, 0.5, 3.2], blade)
        m.cylinder(METAL, [mpos[0], mtop, mpos[2]], [mpos[0], mtop + 1.2, mpos[2]], 1.2, 8)

    # Under the lower plate, the gear mount's rails, centred and turned along Z; the mine hangs from them.
    rl, rh = bounds(rails)
    rc = (rl + rh) / 2
    turn = rot_y(np.pi / 2)
    m.place(rails, turn, np.array([0, -0.2, 0]) - turn @ np.array([rc[0], rh[1], rc[2]]))
    rail_bottom = -0.2 - (rh[1] - rl[1])
    # TM-62M: 32 cm across, 11 cm deep (with its fuze well), slung 3 cm under the rails.
    top = rail_bottom - 3.0
    m.cylinder(OLIVE, [0, top - 9.0, 0], [0, top, 0], 16.0, 20)
    m.cylinder(OLIVE, [0, top - 11.0, 0], [0, top - 9.0, 0], 4.5, 10)
    m.box(METAL, np.array([0, top + 1.5, 0]), [3, 3, 34])  # the release's strap
    # The battery on top: a 12S pack, 20 x 9 x 7 cm, strapped along Z.
    m.box(FRAME, np.array([0, gap + 4.0, 2.0]), [9, 7, 20])
    # Four legs splayed out to two skids, 2 cm under the mine.
    foot = top - 13.0
    for sx in (-1, 1):
        for sz in (-1, 1):
            m.cylinder(FRAME, [sx * 8, -0.2, sz * 8], [sx * 20, foot, sz * 16], 0.9, 8)
        m.cylinder(FRAME, [sx * 20, foot, -26], [sx * 20, foot, 26], 0.9, 8)
    # The camera ball forward and under, marking the front (-Z).
    m.sphere(METAL, np.array([0, -5.0, -17.0]), 4.0)
    m.cylinder(FRAME, [0, -0.2, -17.0], [0, -2.0, -17.0], 1.5, 8)

    # To metres, the feet on y = 0.
    for k, (p, n, t) in m.parts.items():
        for i in range(len(p)):
            p[i] = (p[i] - np.array([0, foot - 0.9, 0], np.float32)) / 100.0
    tris = m.write(OUT)
    # The credit, with what was changed (CC BY asks for it); the bake copies this line into CREDITS.md.
    changed = ("the parts sheet assembled into the octocopter by `game/tools/bake/s1000.py`, painted dark, "
               "with props, a battery, legs, a camera ball and a TM-62 mine added")
    meta = []
    for line in open(os.path.join(SRC, "meta.yaml")):
        if line.startswith("attribution: "):
            line = "attribution: " + json.dumps(json.loads(line[len("attribution: "):]) + f" (changed: {changed})", ensure_ascii=False) + "\n"
        meta.append(line)
    open(os.path.join(OUT, "meta.yaml"), "w").write("".join(meta))
    print(f"wrote {os.path.normpath(OUT)}/scene.gltf: {tris} triangles")


if __name__ == "__main__":
    sys.exit(main())
