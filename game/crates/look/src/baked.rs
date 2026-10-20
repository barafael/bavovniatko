//! Vehicle models baked from real 3D models by `tools/bake` (see `assets/vehicles/CREDITS.md`).
//!
//! A `.veh` file is a hull and a turret (or a helicopter's rotors), each a few meshes, one per material, already in
//! the game's frame: metres, the front towards -Z, the tracks on y = 0, and the turret's parts relative to its
//! ring. They are built into the binary, so it runs from anywhere.
//!
//! The models' own textures are dark and clean, and from 600 m they were dark blobs on a dark road. They are worn
//! here as the panel lines under a livery the vehicle shader (`shaders/vehicle.wgsl`) paints in the hull's frame:
//! each side's camouflage for the season, mud thrown up the lower hull, decks pale with dust or frost, and for
//! a wreck, rust, ash and soot.

use bevy::asset::RenderAssetUsages;
use bevy::image::{CompressedImageFormats, ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor, ImageType};
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::pbr::ExtendedMaterial;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;

pub use bav_sim::units::{Livery, Part, VehicleMaterial};

pub struct Baked {
    pub hull: Vec<Part>,
    pub turret: Vec<Part>,
    /// The turret ring in the hull frame.
    pub turret_at: Vec3,
    /// The gun's muzzle in the turret frame.
    pub muzzle: Vec3,
    /// The hull's half width and its height, for painting what is built on it.
    pub half_width: f32,
    pub deck: f32,
}

/// A material's live and burned-out versions.
pub type Pair = (Handle<VehicleMaterial>, Handle<VehicleMaterial>);

/// Every baked model, by the name `tools/bake` gives it.
pub const IDS: [&str; 18] = ["t80", "t72", "t64", "bmp1", "bmp2", "mtlb", "leo2a6", "leo2r", "bradley", "ka52", "mi8", "mi24", "il76", "an225", "su24", "zu23", "btr80", "bmd2"];

fn file(id: &str) -> &'static [u8] {
    macro_rules! veh {
        ($($id:literal),*) => {
            match id {
                $($id => include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/vehicles/", $id, ".veh")),)*
                _ => panic!("no baked vehicle {id}"),
            }
        };
    }
    veh!("t80", "t72", "t64", "bmp1", "bmp2", "mtlb", "leo2a6", "leo2r", "bradley", "ka52", "mi8", "mi24", "il76", "an225", "su24", "zu23", "btr80", "bmd2")
}

struct Reader<'a> {
    b: &'a [u8],
    at: usize,
}

impl Reader<'_> {
    fn bytes(&mut self, n: usize) -> &[u8] {
        let s = &self.b[self.at..self.at + n];
        self.at += n;
        s
    }
    fn u8(&mut self) -> u8 {
        self.bytes(1)[0]
    }
    fn u32(&mut self) -> u32 {
        u32::from_le_bytes(self.bytes(4).try_into().unwrap())
    }
    fn f32(&mut self) -> f32 {
        f32::from_le_bytes(self.bytes(4).try_into().unwrap())
    }
    fn v3(&mut self) -> Vec3 {
        Vec3::new(self.f32(), self.f32(), self.f32())
    }
}

/// Builds the meshes and materials of a baked model, painted in `livery`'s scheme (the frame and what each
/// material is are filled in here).
pub fn load(id: &str, livery: Livery, meshes: &mut Assets<Mesh>, images: &mut Assets<Image>, materials: &mut Assets<VehicleMaterial>) -> Baked {
    let mut r = Reader { b: file(id), at: 0 };
    assert_eq!(r.bytes(4), b"VEH1", "{id}.veh: not a baked vehicle");
    let (turret_at, muzzle, hull_lo, hull_hi) = (r.v3(), r.v3(), r.v3(), r.v3());
    let half_width = hull_hi.x.max(-hull_lo.x);
    // Each material as read; a part's first wearing makes it a hull's or a turret's pair (live, charred), since
    // the turret's frame sits on its ring and the shader paints by height in the hull's.
    let mut stock = Vec::new();
    for _ in 0..r.u32() {
        let c = LinearRgba::new(r.f32(), r.f32(), r.f32(), r.f32());
        let alpha = r.u8();
        let cutoff = r.f32();
        let double_sided = r.u8() != 0;
        let levels = r.u32();
        let (texture, mean) = match levels {
            0 => (None, LinearRgba::WHITE),
            _ => {
                let (image, mean) = mip_chain(&mut r, levels);
                (Some(images.add(image)), mean)
            }
        };
        let alpha_mode = match alpha {
            0 => AlphaMode::Opaque,
            1 => AlphaMode::Mask(cutoff),
            _ => AlphaMode::Blend,
        };
        // What the material is, by its mean colour: paint, the dark steel and rubber of the running gear and the
        // guns, or what keeps its own colour (lamps, glass, markings).
        let m = LinearRgba::new(c.red * mean.red, c.green * mean.green, c.blue * mean.blue, 1.0);
        let lum = 0.2126 * m.red + 0.7152 * m.green + 0.0722 * m.blue;
        let vivid = m.red > 3.0 * m.green.max(0.002) || m.blue > 1.6 * m.green.max(0.002);
        let class = if alpha == 2 || vivid {
            2.0
        } else if lum < 0.012 {
            1.0
        } else {
            0.0
        };
        let cull_mode = if double_sided { None } else { Some(bevy::render::render_resource::Face::Back) };
        let base = StandardMaterial { base_color: Color::LinearRgba(c), base_color_texture: texture, alpha_mode, double_sided, cull_mode, perceptual_roughness: 0.6, ..default() };
        stock.push((base, Vec4::new(class, if levels > 0 { lum.max(1e-3) } else { 0.0 }, 0.0, 0.0)));
    }
    let mut made: HashMap<(usize, u8), Pair> = HashMap::new();
    let (mut hull, mut turret) = (Vec::new(), Vec::new());
    for _ in 0..r.u32() {
        let role = r.u8();
        let m = r.u32() as usize;
        let nv = r.u32() as usize;
        let (mut pos, mut nrm, mut uv) = (Vec::with_capacity(nv), Vec::with_capacity(nv), Vec::with_capacity(nv));
        for _ in 0..nv {
            pos.push(r.v3().to_array());
            nrm.push(r.v3().to_array());
            uv.push([r.f32(), r.f32()]);
        }
        let ni = r.u32() as usize;
        let idx: Vec<u32> = (0..ni).map(|_| r.u32()).collect();
        let mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, pos)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, nrm)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uv)
            .with_inserted_indices(Indices::U32(idx));
        let (material, charred) = made
            .entry((m, role))
            .or_insert_with(|| {
                let (base, stuff) = &stock[m];
                let lift = if role == 0 { 0.0 } else { turret_at.y };
                pair(materials, base.clone(), &livery, Vec4::new(half_width, hull_hi.y, lift, 0.0), *stuff)
            })
            .clone();
        let part = Part { mesh: meshes.add(mesh), material, charred };
        if role == 0 { hull.push(part) } else { turret.push(part) }
    }
    if turret.is_empty() {
        // Nothing turns on this one; the turret is an empty mesh, so it is still there to aim and burn.
        // (One degenerate triangle: the renderer will not take a mesh with no vertices.)
        let empty = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vec![[0.0f32; 3]; 3])
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0f32, 1.0, 0.0]; 3])
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0f32; 2]; 3])
            .with_inserted_indices(Indices::U32(vec![0, 1, 2]));
        turret.push(Part { mesh: meshes.add(empty), material: hull[0].material.clone(), charred: hull[0].charred.clone() });
    }
    Baked { hull, turret, turret_at, muzzle, half_width, deck: hull_hi.y }
}

/// A material's live and burned-out versions: the same texture (its panel lines stay), painted by the scheme or
/// by the fire.
pub fn pair(materials: &mut Assets<VehicleMaterial>, base: StandardMaterial, livery: &Livery, frame: Vec4, stuff: Vec4) -> Pair {
    let live = Livery { frame, stuff, ..livery.clone() };
    let charred = Livery { frame: frame.with_w(1.0), ..live.clone() };
    let charred_base = StandardMaterial { perceptual_roughness: 0.95, ..base.clone() };
    (materials.add(ExtendedMaterial { base, extension: live }), materials.add(ExtendedMaterial { base: charred_base, extension: charred }))
}

/// A texture and its mip levels, each stored encoded, smallest last; and its mean colour (the last level's),
/// linear.
fn mip_chain(r: &mut Reader, levels: u32) -> (Image, LinearRgba) {
    let mut image: Option<Image> = None;
    let mut data = Vec::new();
    for _ in 0..levels {
        let ext = if r.u8() == 1 { "jpg" } else { "png" };
        let n = r.u32() as usize;
        let level = Image::from_buffer(r.bytes(n), ImageType::Extension(ext), CompressedImageFormats::NONE, true, ImageSampler::Default, RenderAssetUsages::RENDER_WORLD).expect("baked texture");
        data.extend_from_slice(level.data.as_deref().unwrap_or_default());
        image.get_or_insert(level);
    }
    let mut image = image.unwrap();
    let last = &data[data.len().saturating_sub(4)..];
    let mean = Color::srgb_u8(last[0], last[1], last[2]).to_linear();
    image.data = Some(data);
    image.texture_descriptor.mip_level_count = levels;
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        mipmap_filter: ImageFilterMode::Linear,
        anisotropy_clamp: 8,
        ..default()
    });
    (image, mean)
}
