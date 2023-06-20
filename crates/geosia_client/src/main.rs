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
    app.add_plugin(LogPlugin::default())
        .add_plugin(TaskPoolPlugin::default())
        .add_plugin(TypeRegistrationPlugin::default())
        .add_plugin(FrameCountPlugin::default())
        .add_plugin(TimePlugin::default())
        .add_plugin(TransformPlugin::default())
        .add_plugin(HierarchyPlugin::default())
        .add_plugin(DiagnosticsPlugin::default())
        .add_plugin(InputPlugin::default())
        .add_plugin(WindowPlugin {
            primary_window: Some(Window {
                title: "Geosia".to_string(),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            exit_condition: ExitCondition::OnPrimaryClosed,
            close_when_requested: true,
        })
        .add_plugin(AccessibilityPlugin)
        .add_plugin(AssetPlugin::default())
        .add_plugin(ScenePlugin::default())
        .add_plugin(WinitPlugin::default())
        .add_plugin(RenderPlugin::default())
        .add_plugin(ImagePlugin::default())
        .add_plugin(PipelinedRenderingPlugin::default())
        .add_plugin(CorePipelinePlugin::default())
        .add_plugin(SpritePlugin::default())
        .add_plugin(TextPlugin::default())
        .add_plugin(UiPlugin::default())
        .add_plugin(PbrPlugin::default())
        .add_plugin(AudioPlugin::default())
        .add_plugin(GilrsPlugin::default())
        .add_plugin(AnimationPlugin::default())
        .add_plugin(GltfPlugin::default());

    app.add_plugin(debug_window::DebugWindow);

    app.run();
}

mod debug_window {
    use bevy::log;
    use bevy::prelude::*;

    pub struct DebugWindow;

    impl Plugin for DebugWindow {
        fn build(&self, app: &mut App) {
            app.add_startup_system(debug_window_setup);
        }
    }

    fn debug_window_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        log::warn!("Setting up debug window");
        let font: Handle<Font> = asset_server.load("fonts/cascadiacode.ttf");
        commands.spawn(Camera3dBundle::default());
        commands
            .spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(50.0), Val::Percent(50.0)),
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
