use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GameAssets {
    pub ship: Handle<Image>,
    pub meteor: Handle<Image>,
    pub laser: Handle<Image>,
    pub life: Handle<Image>,
    pub bg: Handle<Image>,
    pub laser_audio: Handle<AudioSource>,
    pub collision_audio: Handle<AudioSource>,
    pub font: Handle<Font>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<GameAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = GameAssets {
        ship: asset_server.load("playerShip3_orange.png"),
        meteor: asset_server.load("meteorGrey_big1.png"),
        laser: asset_server.load("laserBlue04.png"),
        life: asset_server.load("playerLife2_orange.png"),
        bg: asset_server.load("darkPurple.png"),
        laser_audio: asset_server.load("sfx_laser2.ogg"),
        collision_audio: asset_server.load("sfx_lose.ogg"),
        font: asset_server.load("kenvector_future.ttf"),
    }
}
