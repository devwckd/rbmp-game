use crate::game_loop::FrameContext;
use crate::renderer::Renderer;
use crate::state::ClientState;
use imgui::StyleVar::SelectableTextAlign;
use imgui::{Context as ImguiContext, FontConfig, FontSource, MouseCursor, Ui};
use imgui_wgpu::{Renderer as ImguiRenderer, RendererConfig};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use wgpu::{Queue, RenderPass};
use winit::event::Event;
use winit::window::Window;

pub struct UiRenderer {
    imgui: ImguiContext,
    platform: WinitPlatform,
    imgui_renderer: ImguiRenderer,
    last_cursor: Option<Option<MouseCursor>>,
}

impl UiRenderer {
    pub fn new(window: &Window, renderer: &mut Renderer) -> Self {
        let mut imgui = ImguiContext::create();

        let mut platform = WinitPlatform::init(&mut imgui);
        platform.attach_window(imgui.io_mut(), window, HiDpiMode::Default);

        imgui.set_ini_filename(None);

        let font_size = (13.0 * window.scale_factor()) as f32;
        imgui.io_mut().font_global_scale = (1.0 / window.scale_factor()) as f32;

        imgui.fonts().add_font(&[FontSource::DefaultFontData {
            config: Some(FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            }),
        }]);

        let imgui_renderer = ImguiRenderer::new(
            &mut imgui,
            renderer.device(),
            renderer.queue(),
            RendererConfig {
                texture_format: renderer.surface_configuration().format,
                ..Default::default()
            },
        );

        Self {
            imgui,
            platform,
            imgui_renderer,
            last_cursor: None,
        }
    }

    // pub fn prepare_frame(&mut self, window: &Window) -> &mut Ui {
    //     self.platform
    //         .prepare_frame(self.imgui.io_mut(), &window)
    //         .expect("failed to prepare imgui frame");
    //
    //     self.imgui.frame()
    // }
    //
    // pub fn render<'a, 'b: 'a>(
    //     &'b mut self,
    //     window: &Window,
    //     renderer: &Renderer,
    //     ui: &mut Ui,
    //     render_pass: &mut RenderPass<'a>,
    // ) {
    //     let mouse_cursor_option = ui.mouse_cursor();
    //     if self.last_cursor != Some(mouse_cursor_option) {
    //         self.last_cursor = Some(mouse_cursor_option);
    //         self.platform.prepare_render(&ui, &window);
    //     }
    //
    //     self.imgui_renderer
    //         .render(
    //             self.imgui.render(),
    //             renderer.queue(),
    //             renderer.device(),
    //             render_pass,
    //         )
    //         .expect("failed to render imgui");
    // }

    pub fn render<'a, 'b: 'a>(
        &'b mut self,
        ctx: &mut FrameContext,
        state: &mut ClientState,
        render_pass: &mut RenderPass<'a>,
        ui_fn: fn(&mut FrameContext, &mut ClientState, &mut Ui),
    ) {
        self.platform
            .prepare_frame(self.imgui.io_mut(), &state.window)
            .expect("failed to prepare imgui frame");

        let ui = self.imgui.frame();

        ui_fn(ctx, state, ui);

        let mouse_cursor_option = ui.mouse_cursor();
        if self.last_cursor != Some(mouse_cursor_option) {
            self.last_cursor = Some(mouse_cursor_option);
            self.platform.prepare_render(&ui, &state.window);
        }

        self.imgui_renderer
            .render(
                self.imgui.render(),
                state.renderer.queue(),
                state.renderer.device(),
                render_pass,
            )
            .expect("failed to render imgui");
    }

    pub fn handle_event(&mut self, window: &Window, event: &Event<()>) {
        self.platform
            .handle_event(self.imgui.io_mut(), window, event);
    }
}
