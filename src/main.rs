use bevy::{prelude::*, render::{mesh::Indices, pipeline::PrimitiveTopology}};
use libh3::{self, GeoCoord};
use map_3d;


pub struct H3Polygon {
    /// The total side length of the square.
    pub altitude: f64,
    pub boundary: Vec<GeoCoord>
}


impl From<H3Polygon> for Mesh {
    fn from(h3polygon: H3Polygon) -> Self {

      // let mut vertices = Vec::new();
      
      // for geocoord in h3polygon.boundary {

      //   let (x, y, z ) = map_3d::geodetic2ecef(geocoord.lat, geocoord.lon, h3polygon.altitude);
      //   println!("x{:?}, y{:?}, z{:?}",x, y, z);

      //   vertices.push(([x as f32, y as f32, -z as f32], [0.0, 1.0, 0.0], [1.0, 1.0]))
      // }


        let vertices = [
          ([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0]),
          ([0.5, 0.0, 0.866], [0.0, 1.0, 0.0], [0.0, 0.0]),
          ([-0.5, 0.0, 0.866], [0.0, 1.0, 0.0], [0.0, 0.0]),
          ([-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0]),
          ([-0.5, 0.0, -0.866], [0.0, 1.0, 0.0], [0.0, 0.0]),
          ([0.5, 0.0, -0.866], [0.0, 1.0, 0.0], [0.0, 0.0]),
        ];

        let indices = Indices::U32(vec![0, 2, 1, 0, 3, 2, 0, 4, 3, 0, 5, 4]);

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        for (position, normal, uv) in vertices.iter() {
            positions.push(*position);
            normals.push(*normal);
            uvs.push(*uv);
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(indices));
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}


fn main() {
  App::build()
    .insert_resource(Msaa { samples: 4 })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup.system())
    .run();
}

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let zero_indexes = libh3::get_res_0_indexes();
  assert_eq!(libh3::get_res_0_indexes().len(), 122);
  // println!("{:?}", zero_indexes);

  for polygon in zero_indexes {
    let boundary = libh3::h3_to_geo_boundary(polygon);
    
    // hexagon
    commands.spawn_bundle(PbrBundle {
      mesh: meshes.add(Mesh::from(H3Polygon { altitude: 1.0, boundary: boundary })),
      material: materials.add(Color::rgb(0.2, 0.2, 0.3).into()),
      ..Default::default()
    });
      
  }
  
  


  // plane
  // commands.spawn_bundle(PbrBundle {
  //   mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
  //   material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
  //   ..Default::default()
  // });
  // cube
  commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    transform: Transform::from_xyz(0.0, 0.5, 0.0),
    ..Default::default()
  });
  // light
  commands.spawn_bundle(LightBundle {
    transform: Transform::from_xyz(4.0, 8.0, 4.0),
    ..Default::default()
  });
  // camera
  commands.spawn_bundle(PerspectiveCameraBundle {
    transform: Transform::from_xyz(-2.0, 5.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ..Default::default()
  });
}
