use bevy::{input_focus::InputFocus, prelude::*};

use crate::{
    app_state::AppState,
    components::SceneEntity,
    resources::{Colors, Fonts},
};

pub fn init_menu(
    mut commands: Commands,
    fonts: Res<Fonts>,
    colors: Res<Colors>,
) {
    commands.spawn((
        SceneEntity,
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(colors.background),
        children![(
            SceneEntity,
            Button,
            Node {
                width: px(150),
                height: px(65),
                border: UiRect::all(px(2)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(px(10)),
                ..default()
            },
            BorderColor::all(colors.foreground),
            BackgroundColor(colors.surface),
            children![(
                SceneEntity,
                Text::new("Button"),
                TextFont {
                    font: fonts.lexend.clone(),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(colors.foreground),
            )]
        )],
    ));
}
pub fn handle_button(
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut Button),
        Changed<Interaction>,
    >,
    colors: Res<Colors>,
) {
    for (entity, interaction, mut background_color, mut button) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);
                *background_color = colors.click.into();
                button.set_changed();
            }
            Interaction::Hovered => {
                input_focus.set(entity);
                *background_color = colors.hover.into();
                button.set_changed();
            }
            Interaction::None => {
                input_focus.clear();
                *background_color = colors.surface.into();
            }
        }
    }
}
pub fn handle_start_button(
    interaction: Single<&Interaction, (With<Button>, Changed<Interaction>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Interaction::Pressed = *interaction {
        next_state.set(AppState::Game);
    }
}
