use super::*;

#[derive(Component)]
pub struct FollowingCameraTag;

pub fn make_camera_follow<T:Component>(mut commands: Commands, query: Single<Entity, (With<T>, Without<FollowingCameraTag>)>) {
    commands.entity(query.into_inner()).insert(FollowingCameraTag);
}
pub fn move_following_camera(
    cam_query: Single<&mut Transform, With<Camera2d>>,
    target_query: Single<&Transform, (With<FollowingCameraTag>, Without<Camera2d>)>
){
    let target_pos = target_query.into_inner();
    let mut cam = cam_query.into_inner();
    cam.translation = target_pos.translation;
}