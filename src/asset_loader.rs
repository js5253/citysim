use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub building_one: Handle<Scene>,
    pub citizen_one: Handle<Scene>,
    pub road: Handle<Scene>
}
pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
        .add_systems(Startup, load_assets);
    }
}
fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        citizen_one: asset_server.load("Player.glb#Scene0"),
        building_one: asset_server.load("Building.glb#Scene0"),
        road: asset_server.load("Road.glb#Scene0"),
    }
}