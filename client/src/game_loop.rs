use std::collections::HashMap;
use std::time::{Duration, Instant};

use imgui::Ui;
use wgpu::RenderPass;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

use crate::state::ClientState;
use crate::ui_renderer::UiRenderer;

#[derive(Copy, Clone, PartialEq)]
enum KeyState {
    Pressed,
    Released,
}

pub struct FrameContext {
    is_fixed: bool,
    delta_time: Duration,
    fixed_delta_time: Duration,
    key_states: [(KeyState, u64); 163],
    frame: u64,
}

impl FrameContext {
    pub fn is_fixed(&self) -> bool {
        self.is_fixed
    }

    pub fn just_pressed(&self, keycode: VirtualKeyCode) -> bool {
        self.key_states[keycode as usize] == (KeyState::Pressed, self.frame)
    }

    pub fn pressed(&self, keycode: VirtualKeyCode) -> bool {
        self.key_states[keycode as usize].0 == KeyState::Pressed
    }

    pub fn just_released(&self, keycode: VirtualKeyCode) -> bool {
        self.key_states[keycode as usize] == (KeyState::Released, self.frame)
    }

    pub fn released(&self, keycode: VirtualKeyCode) -> bool {
        self.key_states[keycode as usize].0 == KeyState::Released
    }

    fn handle_keyboard_event(
        &mut self,
        KeyboardInput {
            state,
            virtual_keycode,
            ..
        }: KeyboardInput,
    ) {
        if let Some(virtual_keycode) = virtual_keycode {
            match state {
                ElementState::Pressed => {
                    let state = &mut self.key_states[virtual_keycode as usize];
                    if state.0 == KeyState::Released {
                        *state = (KeyState::Pressed, self.frame);
                    }
                }
                ElementState::Released => {
                    let state = &mut self.key_states[virtual_keycode as usize];
                    if state.0 == KeyState::Pressed {
                        *state = (KeyState::Released, self.frame);
                    }
                }
            }
        }
    }
}

pub fn client_game_loop(
    event_loop: EventLoop<()>,
    state: ClientState,
    on_update: fn(&mut FrameContext, &mut ClientState),
    on_ui: fn(&mut FrameContext, &mut ClientState, &mut Ui),
    on_render: fn(&mut FrameContext, &mut ClientState, &mut RenderPass),
    interval: Duration,
) {
    let mut state = state;
    let mut ctx = FrameContext {
        is_fixed: true,
        delta_time: Duration::new(0, 0),
        fixed_delta_time: Duration::new(0, 0),
        key_states: [(KeyState::Released, 0); 163],
        frame: 1,
    };

    let mut ui_renderer = UiRenderer::new(&state.window, &mut state.renderer);

    let mut last_tick = Instant::now();
    let mut last_fixed_tick = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        ui_renderer.handle_event(&state.window, &event);

        match event {
            Event::WindowEvent { window_id, event } => {
                if window_id == state.window.id() {
                    match event {
                        WindowEvent::Resized(new_size) => {
                            state.renderer.resize(new_size);
                        }
                        WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                        }
                        WindowEvent::KeyboardInput { input, .. } => {
                            ctx.handle_keyboard_event(input)
                        }
                        _ => {}
                    }
                }
            }

            Event::MainEventsCleared => {
                state.window.request_redraw();
            }

            Event::RedrawRequested(window_id) => {
                if window_id == state.window.id() {
                    let now = Instant::now();

                    ctx.delta_time = last_tick.elapsed();
                    last_tick = now;

                    if last_fixed_tick.elapsed() > interval {
                        ctx.is_fixed = true;
                        ctx.fixed_delta_time = last_fixed_tick.elapsed();
                        last_fixed_tick = now;
                    } else {
                        ctx.is_fixed = false;
                    }

                    on_update(&mut ctx, &mut state);

                    let output = state.renderer.get_output();
                    let view = state.renderer.create_texture_view(&output);
                    let mut command_encoder = state.renderer.create_command_encoder();

                    {
                        let mut render_pass = state
                            .renderer
                            .create_render_pass(&mut command_encoder, &view);

                        ui_renderer.render(&mut ctx, &mut state, &mut render_pass, on_ui);

                        on_render(&mut ctx, &mut state, &mut render_pass);
                    }

                    state.renderer.present(output, command_encoder);

                    ctx.frame += 1;
                }
            }
            _ => {}
        }
    });
}
