//! Which downloaded model becomes which game vehicle, and how each is turned into the game's frame.
//!
//! The spec frame is the source after `z_up`, `yaw` and `units`: metres, +Y up, the front towards -Z, but not
//! yet centred. Rings are written in it; `verbose` prints every node's bounds in it to write them from.

/// How the turret (or rotor) is told apart from the hull.
#[derive(Clone, Copy)]
pub enum Pick {
    /// Only `turret_nodes` turn.
    Nodes,
    /// Besides `turret_nodes`: every piece above `y` within `r` of (`x`, `z`), and narrow pieces (the gun) that
    /// reach forward of it within `gun` of its axis.
    Ring { x: f32, z: f32, y: f32, r: f32, gun: f32 },
}

pub struct Spec {
    /// The game's name for it: `assets/vehicles/<id>.veh`.
    pub id: &'static str,
    /// Under `db/research/assets/files/`.
    pub dir: &'static str,
    pub z_up: bool,
    /// Degrees about +Y that bring the front to -Z.
    pub yaw: f32,
    /// Source units to metres.
    pub units: f32,
    /// The real hull length in metres, if the source is not to scale.
    pub length: Option<f32>,
    pub pick: Pick,
    pub turret_nodes: &'static [usize],
    pub hull_nodes: &'static [usize],
    /// Nodes left out (interiors, crew figures).
    pub drop: &'static [usize],
    /// Keep the hull only: the game builds what sits on it.
    pub turret_none: bool,
    pub max_tris: usize,
    /// Largest texture side.
    pub tex: u32,
    /// Multiplies every material's colour (for untextured models in the wrong paint).
    pub tint: Option<[f32; 4]>,
    pub verbose: bool,
}

const BASE: Spec = Spec {
    id: "",
    dir: "",
    z_up: false,
    yaw: 0.0,
    units: 1.0,
    length: None,
    pick: Pick::Nodes,
    turret_nodes: &[],
    hull_nodes: &[],
    drop: &[],
    turret_none: false,
    max_tris: 30_000,
    tex: 1024,
    tint: None,
    verbose: false,
};

pub const SPECS: &[Spec] = &[
    // T-80 MBT by MirzaArrafiERV_45: the Vuhledar T-80BV (the base hull; the ERA wrap is not modelled).
    Spec { id: "t80", dir: "tanks/tanks_t-80-base_mirzaarrafierv-45", yaw: 180.0, length: Some(7.0), turret_nodes: &[3, 4, 5], ..BASE },
    // T-72B3 by MirzaArrafiERV_45.
    Spec { id: "t72", dir: "tanks/tanks_t-72b3_mirzaarrafierv-45", turret_nodes: &[5, 6, 7, 8, 10, 15], ..BASE },
    // T-64 MBT by MirzaArrafiERV_45: the 72nd Brigade's T-64BV (model in millimetres).
    Spec { id: "t64", dir: "tanks/tanks_t-64_mirzaarrafierv-45", yaw: 180.0, units: 0.001, turret_nodes: &[3, 4], ..BASE },
    // BMP 2 by usedm.
    Spec { id: "bmp2", dir: "ifvs/ifvs_bmp-2_usedm", yaw: 180.0, length: Some(6.72), turret_nodes: &[10], ..BASE },
    // BMP-1 (low-poly) by ScurvyWoof: untextured.
    Spec { id: "bmp1", dir: "ifvs/ifvs_bmp-1_scurvywoof", length: Some(6.74), pick: Pick::Ring { x: 0.0, z: -0.2, y: 0.62, r: 0.5, gun: 0.15 }, tint: Some([0.85, 0.5, 0.55, 1.0]), ..BASE },
    // MTLB by citizensnip.
    Spec { id: "mtlb", dir: "ifvs/ifvs_mt-lb_citizensnip", yaw: 180.0, length: Some(6.45), drop: &[12], pick: Pick::Ring { x: 15.6, z: -33.6, y: 14.5, r: 10.0, gun: 2.0 }, ..BASE },
    // Leopard 2A6 by MirzaArrafiERV_45.
    Spec { id: "leo2a6", dir: "tanks/tanks_leopard-2a6_mirzaarrafierv-45", yaw: 90.0, pick: Pick::Ring { x: 0.0, z: -0.2, y: 1.1, r: 2.0, gun: 0.3 }, turret_nodes: &[7], ..BASE },
    // Leopard 2A4 by MirzaArrafiERV_45: the hull under the Leopard 2R's superstructure.
    Spec { id: "leo2r", dir: "tanks/tanks_leopard-2a4_mirzaarrafierv-45", yaw: 90.0, pick: Pick::Ring { x: 0.0, z: 0.0, y: 0.6, r: 1.9, gun: 0.3 }, turret_none: true, ..BASE },
    // M2A3 Bradley (low-poly) by ScurvyWoof: untextured.
    Spec { id: "bradley", dir: "ifvs/ifvs_bradley_scurvywoof", yaw: 180.0, length: Some(6.55), turret_nodes: &[23, 24, 25, 26, 27, 28, 29, 30, 31], tint: Some([1.0, 0.85, 0.8, 1.0]), ..BASE },
    // Lowpoly Ka-52 by S1Priv: the rotors turn.
    Spec { id: "ka52", dir: "helicopters/helicopters_ka-52_s1priv", length: Some(13.5), turret_nodes: &[46, 50, 55], ..BASE },
];
