//! Vehicle models baked from real 3D models by `tools/bake` (see `assets/vehicles/CREDITS.md`).
//!
//! A `.veh` file is a hull and a turret (or a helicopter's rotors), each a few meshes, one per material, already in
//! the game's frame: metres, the front towards -Z, the tracks on y = 0, and the turret's parts relative to its
//! ring. They are built into the binary, so it runs from anywhere.

use bevy::asset::RenderAssetUsages;
use bevy::image::{CompressedImageFormats, ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor, ImageType};
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;

/// One mesh of a baked model with its material, and the material it takes when the vehicle burns.
#[derive(Clone)]
pub struct Part {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub charred: Handle<StandardMaterial>,
}

pub struct Baked {
    pub hull: Vec<Part>,
    pub turret: Vec<Part>,
    /// The turret ring in the hull frame.
    pub turret_at: Vec3,
    /// The gun's muzzle in the turret frame.
    pub muzzle: Vec3,
}

/// Every baked model, by the name `tools/bake` gives it.
pub const IDS: [&str; 10] = ["t80", "t72", "t64", "bmp1", "bmp2", "mtlb", "leo2a6", "leo2r", "bradley", "ka52"];

fn file(id: &str) -> &'static [u8] {
    macro_rules! veh {
        ($($id:literal),*) => {
            match id {
                $($id => include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/vehicles/", $id, ".veh")),)*
                _ => panic!("no baked vehicle {id}"),
            }
        };
    }
    veh!("t80", "t72", "t64", "bmp1", "bmp2", "mtlb", "leo2a6", "leo2r", "bradley", "ka52")
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

/// Builds the meshes and materials of a baked model. `paint` multiplies every colour (weathering).
pub fn load(id: &str, paint: Color, meshes: &mut Assets<Mesh>, images: &mut Assets<Image>, materials: &mut Assets<StandardMaterial>) -> Baked {
    let mut r = Reader { b: file(id), at: 0 };
    assert_eq!(r.bytes(4), b"VEH1", "{id}.veh: not a baked vehicle");
    let (turret_at, muzzle, _hull_lo, _hull_hi) = (r.v3(), r.v3(), r.v3(), r.v3());
    let paint = paint.to_linear();
    let mut mats = Vec::new();
    for _ in 0..r.u32() {
        let c = LinearRgba::new(r.f32(), r.f32(), r.f32(), r.f32());
        let alpha = r.u8();
        let cutoff = r.f32();
        let double_sided = r.u8() != 0;
        let levels = r.u32();
        let texture = (levels > 0).then(|| images.add(mip_chain(&mut r, levels)));
        let base_color = Color::LinearRgba(LinearRgba::new(c.red * paint.red, c.green * paint.green, c.blue * paint.blue, c.alpha));
        let alpha_mode = match alpha {
            0 => AlphaMode::Opaque,
            1 => AlphaMode::Mask(cutoff),
            _ => AlphaMode::Blend,
        };
        let cull_mode = if double_sided { None } else { Some(bevy::render::render_resource::Face::Back) };
        let live = StandardMaterial { base_color, base_color_texture: texture.clone(), alpha_mode, double_sided, cull_mode, perceptual_roughness: 0.85, ..default() };
        // Burned out: the paint gone to rust and soot, the texture's panel lines still there.
        let charred = StandardMaterial { base_color: Color::srgb(0.3, 0.26, 0.23), perceptual_roughness: 1.0, ..live.clone() };
        mats.push((materials.add(live), materials.add(charred)));
    }
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
        let part = Part { mesh: meshes.add(mesh), material: mats[m].0.clone(), charred: mats[m].1.clone() };
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
    Baked { hull, turret, turret_at, muzzle }
}

/// A texture and its mip levels, each stored encoded, smallest last.
fn mip_chain(r: &mut Reader, levels: u32) -> Image {
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
    image
}
