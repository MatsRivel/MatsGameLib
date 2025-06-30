use super::*;
use crate::{camera::following_camera::{make_camera_follow, move_following_camera}};
#[derive(Default)]
pub struct MyCameraPlugin<CameraTarget: Component>(pub std::marker::PhantomData<CameraTarget>);
impl<CameraTarget: Component> Plugin for MyCameraPlugin<CameraTarget> {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,    (
            spawn_camera, 
            make_camera_follow::<CameraTarget>.after(spawn_camera),
            apply_camera_zoom.after(spawn_camera)
        )).add_systems(Update, move_following_camera);
    }
}

pub fn spawn_camera(mut commands: Commands){
    commands.spawn(Camera2d);
}
pub fn apply_camera_zoom(
    mut query: Query<&mut Projection, With<Camera2d>>,
) {
    for mut projection in query.iter_mut() {
        if let Projection::Orthographic(ref mut ortho) = *projection {
            ortho.scale = 2.0; // Zoomed out; try 0.5 to zoom in
        }
    }
}