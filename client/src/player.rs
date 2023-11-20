use crate::game_loop::FrameContext;
use crate::state::ClientState;
use shared::protocol::{ReliablePacket, UnreliablePacket};
use winit::event::VirtualKeyCode;

pub struct Player {
    id: u32,
    movement: PlayerMovement,
}

pub struct PlayerMovement {
    directions: [bool; 6],
    rotations: [f32; 2],
}

impl Player {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            movement: PlayerMovement {
                directions: [false; 6],
                rotations: [0.0; 2],
            },
        }
    }
}

pub fn update_player_movement(ctx: &mut FrameContext, state: &mut ClientState) {
    let movement = &mut state.player.movement;

    movement.directions[0] = ctx.pressed(VirtualKeyCode::A);
    movement.directions[1] = ctx.pressed(VirtualKeyCode::W);
    movement.directions[2] = ctx.pressed(VirtualKeyCode::D);
    movement.directions[3] = ctx.pressed(VirtualKeyCode::S);
    movement.directions[4] = ctx.pressed(VirtualKeyCode::LShift);
    movement.directions[5] = ctx.pressed(VirtualKeyCode::Space);

    movement.rotations[0] = 0.0;
    movement.rotations[1] = 0.0;
}

pub fn send_player_movement_packet(_ctx: &mut FrameContext, state: &mut ClientState) {
    let movement = &state.player.movement;
    state.send_reliable_packet(ReliablePacket::MovementInput {
        directions: movement.directions.clone(),
        rotations: movement.rotations.clone(),
    });
}
