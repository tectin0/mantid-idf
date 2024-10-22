const TEST_DETECTOR_DEFINITION_PATH: &str = "assets/test_detector_definition.xml";

#[tokio::main]
async fn main() {
    run().await;
}

fn get_center_points() -> Vec<[f32; 3]> {
    let content =
        std::fs::read_to_string(TEST_DETECTOR_DEFINITION_PATH).expect("could not read file");

    let detector_definition = mantid_idf::DetectorDefinition::from_str(&content)
        .expect("could not parse detector definition");

    let root = detector_definition.component_tree;

    let points = root.get_special_type_points();

    points.into_iter().map(|p| [p.x, p.y, p.z]).collect()
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

    let points = get_center_points();

    let cpu_point_cloud = PointCloud {
        positions: Positions::F32(points.iter().map(|&p| vec3(p[0], p[1], p[2])).collect()),
        ..Default::default()
    };

    let mut point_mesh = CpuMesh::sphere(4);
    point_mesh.transform(&Mat4::from_scale(0.005)).unwrap();

    let mut point_cloud = Gm {
        geometry: InstancedMesh::new(&context, &cpu_point_cloud.into(), &point_mesh),
        material: ColorMaterial {
            color: Srgba::RED,
            ..Default::default()
        },
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
