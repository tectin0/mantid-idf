const TEST_DETECTOR_DEFINITION_PATH: &str = "assets/test_detector_definition.xml";

#[tokio::main]
async fn main() {
    run().await;
}

struct DetectorPlotInformation {
    points: Vec<[f32; 3]>,
    units: Vec<usize>,
}

fn get_detector_information() -> DetectorPlotInformation {
    let args = std::env::args().collect::<Vec<String>>();

    let detector_definition_path = args
        .get(1)
        .cloned()
        .unwrap_or(TEST_DETECTOR_DEFINITION_PATH.to_string());

    let path = std::path::Path::new(&detector_definition_path);

    let content = std::fs::read_to_string(path).expect("could not read file");

    let detector_definition = mantid_idf::DetectorDefinition::from_str(&content)
        .expect("could not parse detector definition");

    let (_id_list_name, id_list) = detector_definition.id_lists.first_key_value().unwrap();

    let ids = id_list.get_ids();

    let units = ids.into_iter().map(|id| id / 100000).collect::<Vec<_>>();

    let root = detector_definition.component_tree;

    let points = root.get_special_type_points();

    let points = points.into_iter().map(|p| [p.x, p.y, p.z]).collect();

    DetectorPlotInformation { points, units }
}

use three_d::*;

pub async fn run() {
    let window = Window::new(WindowSettings {
        title: "Point Cloud!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.125, -0.25, -0.5),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.01,
        150.0,
    );
    let mut control = OrbitControl::new(*camera.target(), 0.1, 5.0);

    let DetectorPlotInformation { points, units } = get_detector_information();

    let (min_units, max_units) = units
        .iter()
        .fold((usize::MAX, usize::MIN), |(min, max), &unit| {
            (min.min(unit), max.max(unit))
        });

    let colors = units
        .into_iter()
        .map(|unit| {
            let n = (unit - min_units) as f32 / (max_units - min_units) as f32;
            sample_jet(n)
        })
        .collect::<Vec<_>>();

    let cpu_point_cloud = PointCloud {
        positions: Positions::F32(points.iter().map(|&p| vec3(p[0], p[1], p[2])).collect()),
        ..Default::default()
    };

    let mut point_mesh = CpuMesh::sphere(4);
    point_mesh.transform(&Mat4::from_scale(0.005)).unwrap();

    let mut instances: Instances = cpu_point_cloud.into();

    instances.colors = Some(colors);

    let instanced_mesh = InstancedMesh::new(&context, &instances, &point_mesh);

    let mut point_cloud = Gm {
        geometry: instanced_mesh,
        material: ColorMaterial::default(),
    };

    let c = -point_cloud.aabb().center();
    point_cloud.set_transformation(Mat4::from_translation(c));

    window.render_loop(move |mut frame_input| {
        let mut redraw = frame_input.first_frame;
        redraw |= camera.set_viewport(frame_input.viewport);
        redraw |= control.handle_events(&mut camera, &mut frame_input.events);

        if redraw {
            frame_input
                .screen()
                .clear(ClearState::color_and_depth(1.0, 1.0, 1.0, 1.0, 1.0))
                .render(
                    &camera,
                    point_cloud
                        .into_iter()
                        .chain(&Axes::new(&context, 0.01, 0.1)),
                    &[],
                );
        }

        FrameOutput {
            swap_buffers: redraw,
            ..Default::default()
        }
    });
}

// r g b 0..255
// n 0..1
fn sample_jet(n: f32) -> Srgba {
    let n = n.clamp(0.0, 1.0);
    let r = (255.0 * (1.5 - 4.0 * (n - 0.75).abs()))
        .round()
        .clamp(0.0, 255.0) as u8;
    let g = (255.0 * (1.5 - 4.0 * (n - 0.5).abs()))
        .round()
        .clamp(0.0, 255.0) as u8;
    let b = (255.0 * (1.5 - 4.0 * (n - 0.25).abs()))
        .round()
        .clamp(0.0, 255.0) as u8;
    Srgba::new(r, g, b, 255)
}
