use bevy::{prelude::*, render::{mesh::Indices, pipeline::PrimitiveTopology}};
use libh3::{self, GeoCoord};
use map_3d;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

// This is a simple example of a camera that flies around.
// There's an included example of a system that toggles the "enabled"
// property of the fly camera with "T"

fn init(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands.spawn().insert_bundle(LightBundle {
		transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
		..Default::default()
	});
	commands
		.spawn()
		.insert_bundle(PerspectiveCameraBundle::new_3d())
		.insert(FlyCamera::default());

	let box_mesh = meshes.add(Mesh::from(shape::Cube { size: 0.25 }));
	let box_material = materials.add(Color::rgb(1.0, 0.2, 0.3).into());

	const AMOUNT: i32 = 6;
	for x in -(AMOUNT / 2)..(AMOUNT / 2) {
		for y in -(AMOUNT / 2)..(AMOUNT / 2) {
			for z in -(AMOUNT / 2)..(AMOUNT / 2) {
				commands.spawn().insert_bundle(PbrBundle {
					mesh: box_mesh.clone(),
					material: box_material.clone(),
					transform: Transform::from_translation(Vec3::new(
						x as f32, y as f32, z as f32,
					)),
					..Default::default()
				});
			}
		}
	}

	println!("Started example!");
}

// Press "T" to toggle keyboard+mouse control over the camera
fn toggle_button_system(
	input: Res<Input<KeyCode>>,
	mut query: Query<&mut FlyCamera>,
) {
	for mut options in query.iter_mut() {
		if input.just_pressed(KeyCode::T) {
      options.enabled = !options.enabled;
			println!("Toggled FlyCamera {}!",options.enabled);
		}
	}
}




pub struct H3Polygon {
    /// The total side length of the square.
    pub altitude: f64,
    pub geo_boundary: Vec<GeoCoord>
}


impl From<H3Polygon> for Mesh {
    fn from(h3polygon: H3Polygon) -> Self {

      let mut vertices = Vec::new();
      
      for geocoord in h3polygon.geo_boundary {

        let (x, y, z ) = map_3d::geodetic2ecef(geocoord.lat, geocoord.lon, h3polygon.altitude);
        
        let divisor = 1000000.0;
        let smallercords = [x as f32 / divisor as f32, y as f32 / divisor as f32, z as f32 / divisor as f32];

        println!("x{:?}, y{:?}, z{:?}",x, y, z);

        vertices.push(([smallercords[0], smallercords[1], smallercords[2]], [0.0, 1.0, 0.0], [1.0, 1.0]))
      }


      //   let vertices = [
      //     ([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0]),
      //     ([0.5, 0.0, 0.866], [0.0, 1.0, 0.0], [0.0, 0.0]),
      //     ([-0.5, 0.0, 0.866], [0.0, 1.0, 0.0], [0.0, 0.0]),
      //     ([-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0]),
      //     ([-0.5, 0.0, -0.866], [0.0, 1.0, 0.0], [0.0, 0.0]),
      //     ([0.5, 0.0, -0.866], [0.0, 1.0, 0.0], [0.0, 0.0]),
      //   ];




        let indices = match vertices.len() {
          5 => {Indices::U32(vec![0, 1, 2, 0, 2, 3, 0, 3, 4])}
          6 => {Indices::U32(vec![0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5])}
          _ => {Indices::U32(vec![])}
        };

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
    .add_startup_system(init.system())
		.add_plugin(FlyCameraPlugin)
		.add_system(toggle_button_system.system())

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
    
    //if boundary.len() == 6 {
      // hexagon

      let h3material = StandardMaterial {
        base_color: Color::rgba(rand::random(), 0.50, 0.75, 1.0),
        double_sided: false,
        unlit: true,
        ..Default::default()
      };


      //let material = materials.add(Color::rgb(0.2, 0.2, 0.3).into());
      commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(H3Polygon { altitude: 1.0, geo_boundary: boundary })),
        material: materials.add(h3material.into()),
        ..Default::default()
      });
    //}
      
  }
  
  


  // plane
  // commands.spawn_bundle(PbrBundle {
  //   mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
  //   material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
  //   ..Default::default()
  // });
  // cube
  // commands.spawn_bundle(PbrBundle {
  //   mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
  //   material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
  //   transform: Transform::from_xyz(0.0, 0.5, 0.0),
  //   ..Default::default()
  // });
  // light
  commands.spawn_bundle(LightBundle {
    transform: Transform::from_xyz(4.0, 8.0, 4.0),
    ..Default::default()
  });
  // camera
  commands.spawn_bundle(PerspectiveCameraBundle {
    transform: Transform::from_xyz(-20.0, 35.5, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ..Default::default()
  });
}
