use bevy::{prelude::*, render::{mesh::Indices, pipeline::PrimitiveTopology}};
use libh3::{self, GeoCoord};
use map_3d;
use bevy::input::mouse::{MouseWheel,MouseMotion};
use bevy::render::camera::PerspectiveProjection;

struct PanOrbitCamera {
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}

/// Pan the camera with middle mouse click, zoom with scroll wheel, orbit with right mouse click.
fn pan_orbit_camera(
    windows: Res<Windows>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform, &PerspectiveProjection)>,
) {
    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Left;
    let pan_button = MouseButton::Right;

    let mut pan = Vec2::ZERO;
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut orbit_button_changed = false;

    if input_mouse.pressed(orbit_button) {
        for ev in ev_motion.iter() {
            rotation_move += ev.delta;
        }
    } else if input_mouse.pressed(pan_button) {
        // Pan only if we're not rotating at the moment
        for ev in ev_motion.iter() {
            pan += ev.delta;
        }
    }
    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }
    if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
        orbit_button_changed = true;
    }

    for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
        if orbit_button_changed {
            // only check for upside down when orbiting started or ended this frame
            // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
            let up = transform.rotation * Vec3::Y;
            pan_orbit.upside_down = up.y <= 0.0;
        }

        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;
            let window = get_primary_window_size(&windows);
            let delta_x = {
                let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
                if pan_orbit.upside_down { -delta } else { delta }
            };
            let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation = transform.rotation * pitch; // rotate around local x axis
        } else if pan.length_squared() > 0.0 {
            any = true;
            // make panning distance independent of resolution and FOV,
            let window = get_primary_window_size(&windows);
            pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
            // translate by local axes
            let right = transform.rotation * Vec3::X * -pan.x;
            let up = transform.rotation * Vec3::Y * pan.y;
            // make panning proportional to distance away from focus point
            let translation = (right + up) * pan_orbit.radius;
            pan_orbit.focus += translation;
        } else if scroll.abs() > 0.0 {
            any = true;
            pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
            // dont allow zoom to reach zero or you get stuck
            pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
        }

        if any {
            // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
            // parent = x and y rotation
            // child = z-offset
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation = pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
        }
    }
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}

/// Spawn a camera like this
fn spawn_camera(mut commands: Commands) {
    let translation = Vec3::new(-20.0, 35.5, 15.0);
    let radius = translation.length();

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(translation)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    }).insert(PanOrbitCamera {
        radius,
        ..Default::default()
    });
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

        //println!("x{:?}, y{:?}, z{:?}",x, y, z);

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
    .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))

    .add_plugins(DefaultPlugins)
    .add_startup_system(setup.system())
    .add_startup_system(spawn_camera.system())
    .add_system(pan_orbit_camera.system())
    .run();
}

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let zero_indexes = libh3::get_res_0_indexes();
  assert_eq!(libh3::get_res_0_indexes().len(), 122);

  for polygon in zero_indexes {
    let boundary = libh3::h3_to_geo_boundary(polygon);
    
      let h3material = StandardMaterial {
        base_color: Color::rgba(rand::random(), 0.50, 0.75, 1.0),
        double_sided: false,
        unlit: true,
        ..Default::default()
      };

      commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(H3Polygon { altitude: 1.0, geo_boundary: boundary })),
        material: materials.add(h3material.into()),
        ..Default::default()
      });
      
  }
  
  
}
