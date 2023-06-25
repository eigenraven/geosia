use bevy::a11y::AccessibilityPlugin;
use bevy::audio::AudioPlugin;
use bevy::core_pipeline::CorePipelinePlugin;
use bevy::diagnostic::DiagnosticsPlugin;
use bevy::gltf::GltfPlugin;
use bevy::input::InputPlugin;
use bevy::log::LogPlugin;
use bevy::pbr::PbrPlugin;
use bevy::prelude::*;
use bevy::render::pipelined_rendering::PipelinedRenderingPlugin;
use bevy::render::RenderPlugin;
use bevy::scene::ScenePlugin;
use bevy::sprite::SpritePlugin;
use bevy::text::TextPlugin;
use bevy::time::TimePlugin;
use bevy::ui::UiPlugin;
use bevy::window::{ExitCondition, PresentMode};
use bevy::winit::WinitPlugin;

fn main() {
    // Unset the manifest dir to make bevy load assets from the workspace root
    std::env::set_var("CARGO_MANIFEST_DIR", "");

    let mut app = App::new();
    // Bevy Base
    app.add_plugins(LogPlugin::default())
        .add_plugins(TaskPoolPlugin::default())
        .add_plugins(TypeRegistrationPlugin::default())
        .add_plugins(FrameCountPlugin::default())
        .add_plugins(TimePlugin::default())
        .add_plugins(TransformPlugin::default())
        .add_plugins(HierarchyPlugin::default())
        .add_plugins(DiagnosticsPlugin::default())
        .add_plugins(InputPlugin::default())
        .add_plugins(WindowPlugin {
            primary_window: Some(Window {
                title: "Geosia".to_string(),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            exit_condition: ExitCondition::OnPrimaryClosed,
            close_when_requested: true,
        })
        .add_plugins(AccessibilityPlugin)
        .add_plugins(AssetPlugin::default())
        .add_plugins(ScenePlugin::default())
        .add_plugins(WinitPlugin::default())
        .add_plugins(RenderPlugin::default())
        .add_plugins(ImagePlugin::default())
        .add_plugins(PipelinedRenderingPlugin::default())
        .add_plugins(CorePipelinePlugin::default())
        .add_plugins(SpritePlugin::default())
        .add_plugins(TextPlugin::default())
        .add_plugins(UiPlugin::default())
        .add_plugins(PbrPlugin::default())
        .add_plugins(AudioPlugin::default())
        .add_plugins(GilrsPlugin::default())
        .add_plugins(AnimationPlugin::default())
        .add_plugins(GltfPlugin::default());

    app.add_plugins(debug_window::DebugWindow);

    app.run();
}

mod debug_window {
    use bevy::log;
    use bevy::prelude::*;

    pub struct DebugWindow;

    impl Plugin for DebugWindow {
        fn build(&self, app: &mut App) {
            app.add_systems(Startup, debug_window_setup);
        }
    }

    fn debug_window_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        log::warn!("Setting up debug window");
        let font: Handle<Font> = asset_server.load("fonts/cascadiacode.ttf");
        commands.spawn(Camera3dBundle::default());
        commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(50.0),
                    height: Val::Percent(50.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::CRIMSON.into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Hello Bevy",
                    TextStyle {
                        font: font.clone(),
                        font_size: 75.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
                log::warn!("Child made");
            });
        log::warn!("Setting up debug window done");
    }
}
