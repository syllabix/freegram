use bevy::{
    app::{Plugin, Update},
    color::Color,
    ecs::{
        component::Component,
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    input::{keyboard::KeyCode, mouse::MouseButton, ButtonInput},
    log,
    math::Vec2,
    prelude::*,
    render::camera::Camera,
    sprite::Sprite,
    transform::components::Transform,
    utils::default,
    window::{PrimaryWindow, Window},
};

use crate::camera::PanningLock;

#[derive(Resource, Default)]
pub struct DragState {
    pub dragged_entity: Option<Entity>,
    pub click_offset: Option<Vec2>,
    pub resizing_entity: Option<Entity>,
    pub initial_size: Option<Vec2>,
    pub initial_distance: Option<f32>,
}

impl DragState {
    pub fn reset(&mut self) {
        self.dragged_entity = None;
        self.click_offset = None;
        self.resizing_entity = None;
        self.initial_size = None;
        self.initial_distance = None;
    }
}

pub struct WidgetPlugin;

impl Plugin for WidgetPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<DragState>().add_systems(
            Update,
            (Sticky::spawn, Widget::update_position, Widget::resize),
        );
    }
}

#[derive(Component)]
#[require(Sprite, Transform)]
pub struct Widget;

impl Widget {
    pub fn update_position(
        mut _commands: Commands,
        mouse_button: Res<ButtonInput<MouseButton>>,
        window: Query<&Window, With<PrimaryWindow>>,
        camera: Query<(&Camera, &GlobalTransform)>,
        mut drag_state: ResMut<DragState>,
        mut panning_lock: ResMut<PanningLock>,
        mut widgets: Query<(Entity, &mut Transform, &Sprite), With<Self>>,
    ) {
        // Don't allow dragging if we're resizing
        if drag_state.resizing_entity.is_some() {
            return;
        }

        // Handle drag end
        if mouse_button.just_released(MouseButton::Left) {
            drag_state.reset();
            panning_lock.is_locked = false;
            return;
        }

        // Start dragging if mouse is pressed and over a widget
        if mouse_button.just_pressed(MouseButton::Left) {
            if let (Ok(window), Ok((camera, camera_transform))) =
                (window.get_single(), camera.get_single())
            {
                if let Some(cursor_pos) = window.cursor_position() {
                    // Convert cursor position to world coordinates
                    let world_pos = camera
                        .viewport_to_world_2d(camera_transform, cursor_pos)
                        .unwrap_or_default();

                    // Check if cursor is over any widget
                    for (entity, transform, sprite) in widgets.iter() {
                        let widget_pos = transform.translation.truncate();
                        let half_size = sprite.custom_size.unwrap_or_default() / 2.0;

                        // Simple AABB collision check
                        if (world_pos.x >= widget_pos.x - half_size.x
                            && world_pos.x <= widget_pos.x + half_size.x)
                            && (world_pos.y >= widget_pos.y - half_size.y
                                && world_pos.y <= widget_pos.y + half_size.y)
                        {
                            // Calculate the offset from the widget's center to the click position
                            let click_offset = world_pos - widget_pos;
                            drag_state.dragged_entity = Some(entity);
                            drag_state.click_offset = Some(click_offset);
                            panning_lock.is_locked = true;
                            break;
                        }
                    }
                }
            }
        }

        // Update position while dragging
        if let Some(dragged_entity) = drag_state.dragged_entity {
            if let (Ok(window), Ok((camera, camera_transform))) =
                (window.get_single(), camera.get_single())
            {
                if let Some(position) = window.cursor_position() {
                    if let Ok((_, mut transform, _)) = widgets.get_mut(dragged_entity) {
                        let world_position = camera
                            .viewport_to_world_2d(camera_transform, position)
                            .unwrap_or_default();

                        // Apply the stored offset to maintain the relative position
                        if let Some(offset) = drag_state.click_offset {
                            transform.translation = (world_position - offset).extend(0.0);
                        } else {
                            transform.translation = world_position.extend(0.0);
                        }
                    }
                }
            }
        }
    }

    pub fn resize(
        mut _commands: Commands,
        mouse_button: Res<ButtonInput<MouseButton>>,
        window: Query<&Window, With<PrimaryWindow>>,
        camera: Query<(&Camera, &GlobalTransform)>,
        mut drag_state: ResMut<DragState>,
        mut panning_lock: ResMut<PanningLock>,
        mut widgets: Query<(Entity, &mut Transform, &mut Sprite), With<Self>>,
    ) {
        // Handle resize end
        if mouse_button.just_released(MouseButton::Left) {
            drag_state.reset();
            panning_lock.is_locked = false;
            return;
        }

        // Check if we're near the edge of any widget
        if mouse_button.just_pressed(MouseButton::Left) {
            if let (Ok(window), Ok((camera, camera_transform))) =
                (window.get_single(), camera.get_single())
            {
                if let Some(cursor_pos) = window.cursor_position() {
                    let world_pos = camera
                        .viewport_to_world_2d(camera_transform, cursor_pos)
                        .unwrap_or_default();

                    for (entity, transform, sprite) in widgets.iter() {
                        let widget_pos = transform.translation.truncate();
                        let size = sprite.custom_size.unwrap_or_default();
                        let half_size = size / 2.0;

                        // Check if cursor is within 15 pixels of the edge
                        let edge_threshold = 15.0;
                        let is_near_edge = (world_pos.x
                            >= widget_pos.x - half_size.x - edge_threshold
                            && world_pos.x <= widget_pos.x - half_size.x + edge_threshold)
                            || (world_pos.x >= widget_pos.x + half_size.x - edge_threshold
                                && world_pos.x <= widget_pos.x + half_size.x + edge_threshold)
                            || (world_pos.y >= widget_pos.y - half_size.y - edge_threshold
                                && world_pos.y <= widget_pos.y - half_size.y + edge_threshold)
                            || (world_pos.y >= widget_pos.y + half_size.y - edge_threshold
                                && world_pos.y <= widget_pos.y + half_size.y + edge_threshold);

                        if is_near_edge {
                            drag_state.dragged_entity = None;
                            drag_state.resizing_entity = Some(entity);
                            drag_state.initial_size = Some(size);
                            drag_state.initial_distance = Some((world_pos - widget_pos).length());
                            panning_lock.is_locked = true;
                            break;
                        }
                    }
                }
            }
        }

        // Handle resizing
        if let Some(resizing_entity) = drag_state.resizing_entity {
            if let (Ok(window), Ok((camera, camera_transform))) =
                (window.get_single(), camera.get_single())
            {
                if let Some(position) = window.cursor_position() {
                    if let Ok((_, transform, mut sprite)) = widgets.get_mut(resizing_entity) {
                        let world_position = camera
                            .viewport_to_world_2d(camera_transform, position)
                            .unwrap_or_default();

                        let widget_pos = transform.translation.truncate();
                        let current_distance = (world_position - widget_pos).length();

                        if let (Some(initial_size), Some(initial_distance)) =
                            (drag_state.initial_size, drag_state.initial_distance)
                        {
                            // Calculate size change based on distance change
                            let distance_ratio = current_distance / initial_distance;
                            let new_size = initial_size * distance_ratio;

                            // Apply minimum and maximum size constraints
                            let min_size = 50.0;
                            let max_size = 1000.0;

                            let clamped_size = Vec2::new(
                                new_size.x.clamp(min_size, max_size),
                                new_size.y.clamp(min_size, max_size),
                            );

                            // Update the sprite size
                            sprite.custom_size = Some(clamped_size);
                        }
                    }
                }
            }
        }
    }
}

#[derive(Component)]
pub struct Sticky;

impl Sticky {
    pub fn spawn(
        mut commands: Commands,
        keys: Res<ButtonInput<KeyCode>>,
        window: Query<&Window, With<PrimaryWindow>>,
        camera: Query<(&Camera, &GlobalTransform)>,
    ) {
        if keys.just_pressed(KeyCode::KeyS) {
            if let (Ok(window), Ok((camera, camera_transform))) =
                (window.get_single(), camera.get_single())
            {
                // Get viewport center in world coordinates
                let viewport_center = Vec2::new(window.width() / 2.0, window.height() / 2.0);
                let world_position = camera
                    .viewport_to_world_2d(camera_transform, viewport_center)
                    .unwrap_or_default();

                commands.spawn((
                    Self,
                    Widget,
                    Sprite {
                        color: Color::srgb(1.0, 0.95, 0.5), // Sticky pad yellow
                        custom_size: Some(Vec2 { x: 150., y: 150. }),
                        ..default()
                    },
                    Transform::from_translation(world_position.extend(0.0)),
                ));
            }
        }
    }
}
