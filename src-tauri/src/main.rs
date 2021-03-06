#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use bevy::{
    asset::AssetPlugin,
    audio::AudioPlugin,
    core::CorePlugin,
    core_pipeline::CorePipelinePlugin,
    diagnostic::DiagnosticsPlugin,
    ecs::event::Events,
    gltf::GltfPlugin,
    input::InputPlugin,
    log::LogPlugin,
    pbr::PbrPlugin,
    prelude::*,
    render::RenderPlugin,
    scene::ScenePlugin,
    sprite::SpritePlugin,
    text::TextPlugin,
    ui::UiPlugin,
    window::{CreateWindow, WindowCreated, WindowPlugin},
};
use raw_window_handle::HasRawWindowHandle;
use tauri::{Manager, RunEvent, WindowBuilder};
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

fn main() {
    let app = tauri::Builder::default()
        .build(tauri::generate_context!())
        .expect("error while build tauri application");

    let main_window = app.get_window("main").unwrap();
    // #[cfg(target_os = "macos")]
    // apply_vibrancy(&main_window, NSVisualEffectMaterial::ContentBackground).unwrap();

    let mut bevy_app = App::new();
    bevy_app.insert_resource(ClearColor(Color::RED));
    // Need to update twice, otherwise we get a panic about content view not available
    // bevy_app.update();
    // bevy_app.update();

    let mut iter_count: usize = 0;

    app.run(move |_app_handle, event| {
        match event {
            RunEvent::Ready => {}
            RunEvent::MainEventsCleared => {
                if iter_count == 0 {
                    bevy_app.add_plugin(LogPlugin::default());
                    bevy_app.add_plugin(CorePlugin::default());
                    bevy_app.add_plugin(TransformPlugin::default());
                    bevy_app.add_plugin(DiagnosticsPlugin::default());
                    bevy_app.add_plugin(InputPlugin::default());
                    bevy_app.add_plugin(WindowPlugin::default());
                    bevy_app.add_plugin(AssetPlugin::default());
                    #[cfg(feature = "debug_asset_server")]
                    bevy_app.add_plugin(
                        bevy_asset::debug_asset_server::DebugAssetServerPlugin::default(),
                    );
                    bevy_app.add_plugin(ScenePlugin::default());

                    // #[cfg(feature = "bevy_winit")]
                    // bevy_app.add_plugin(bevy_winit::WinitPlugin::default());
                    {
                        let world = bevy_app.world.cell();
                        let mut windows = world.get_resource_mut::<Windows>().unwrap();
                        let mut create_window_events =
                            world.get_resource_mut::<Events<CreateWindow>>().unwrap();
                        let mut window_created_events =
                            world.get_resource_mut::<Events<WindowCreated>>().unwrap();

                        for create_window_event in create_window_events.drain() {
                            let inner_size = main_window.inner_size().unwrap();
                            let scale_factor = main_window.scale_factor().unwrap();
                            let position = main_window
                                .outer_position()
                                .ok()
                                .map(|position| IVec2::new(position.x, position.y));
                            let raw_window_handle = main_window.raw_window_handle();
                            let window = Window::new(
                                create_window_event.id,
                                &create_window_event.descriptor,
                                inner_size.width,
                                inner_size.height,
                                scale_factor,
                                position,
                                raw_window_handle,
                            );
                            windows.add(window);
                            window_created_events.send(WindowCreated {
                                id: create_window_event.id,
                            });
                        }
                    }

                    bevy_app.add_plugin(RenderPlugin::default());
                    bevy_app.add_plugin(CorePipelinePlugin::default());
                    bevy_app.add_plugin(SpritePlugin::default());
                    bevy_app.add_plugin(TextPlugin::default());
                    bevy_app.add_plugin(UiPlugin::default());
                    bevy_app.add_plugin(PbrPlugin::default());
                    bevy_app.add_plugin(GltfPlugin::default());
                    bevy_app.add_plugin(AudioPlugin::default());
                    bevy_app.add_plugin(GilrsPlugin::default());

                    bevy_app
                        .add_startup_system(rect_system)
                        .add_system(debug_system);
                } else {
                    bevy_app.update();
                }
            }
            _ => {}
        };
        iter_count += 1;
    });
}

fn rect_system(mut commands: Commands) {
    println!("Hello from rect system");
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn debug_system(time: Res<Time>) {
    // println!("detla: {}", time.delta_seconds());
}
