#![feature(variant_count)]

use std::time::Duration;

use imgui::{Condition, Ui};
use wgpu::RenderPass;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use crate::game_loop::{client_game_loop, FrameContext};
use crate::player::{send_player_movement_packet, update_player_movement, Player};
use crate::renderer::Renderer;
use crate::state::ClientState;

mod game_loop;
mod networking;
mod player;
mod renderer;
mod state;
mod ui_renderer;

#[tokio::main]
async fn main() {
    shared::tracing::init();

    networking::init().await;

    loop {}

    // let event_loop = EventLoop::new();
    // let window = WindowBuilder::new().build(&event_loop).unwrap();
    //
    // let renderer = Renderer::new(&window);
    //
    // let state = ClientState {
    //     renderer,
    //     player: Player::new(id),
    //     window,
    //     tcp_receiver,
    //     udp_receiver,
    //     packet_action_sender,
    // };
    //
    // client_game_loop(
    //     event_loop,
    //     state,
    //     update,
    //     ui,
    //     render,
    //     Duration::from_millis(1000 / 30),
    // );
}

fn update(ctx: &mut FrameContext, state: &mut ClientState) {
    update_player_movement(ctx, state);
    send_player_movement_packet(ctx, state);
}

fn ui(ctx: &mut FrameContext, state: &mut ClientState, ui: &mut Ui) {
    let window = ui.window("sask");
    window
        .size([300.0, 200.0], Condition::FirstUseEver)
        .build(|| {
            ui.text("velho calvo");
        });
}

fn render(ctx: &mut FrameContext, state: &mut ClientState, ui: &mut RenderPass) {}
