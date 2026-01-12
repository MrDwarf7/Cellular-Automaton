use iced::widget::{button, column, container, row, slider, text, text_input};
use iced::{time, window, Application, Command, Element, Settings, Subscription};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub mod cell;
pub mod genetics;
pub mod grid;
pub mod logging;
pub mod metrics;
pub mod ml_layer;
pub mod nca;
pub mod presets;
pub mod rules;
pub mod stats;
pub mod ui;

use grid::Grid;
use logging::init_logging;
use metrics::MetricsCollector;
use ui::GridDisplay;

use crate::presets::{Preset, PresetT};

const GRID_WIDTH: u32 = 500;
const GRID_HEIGHT: u32 = 500;

// ============================================================================
// Messages
// ============================================================================

#[derive(Debug, Clone)]
enum Message {
    Play,
    Pause,
    Reset,
    SpeedChanged(f32),
    PresetInputChanged(String), // PERF: store a Preset enum type not a String
    LoadPreset,
    Tick,
}

// ============================================================================
// Application State
// ============================================================================

struct CellularApp {
    grid: Arc<Mutex<Grid>>,
    is_running: bool,
    tick_count: u64,
    speed: f32,
    // selected_preset: Preset, // PERF: use the Preset enum type directly
    selected_preset: String,
    tick_accumulator: f32,
    metrics: Arc<Mutex<MetricsCollector>>,
    last_tick_time: Instant,
}

impl Application for CellularApp {
    type Message = Message;
    type Theme = iced::Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        // Initialize logging
        let _ = init_logging();

        let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);

        // Initialize with sparse genesis preset by default

        // TODO:[better_pattern] : load like this, let initialize_random handle the fallback
        let preset = presets::Preset::from("sparse_genesis");
        grid.initialize_random(&preset.data());

        // if let Some(densities) = presets::load_preset("sparse_genesis") {
        //     grid.initialize_random(&densities);
        // } else {
        //     // PERF:[wtf] is this
        //     grid.initialize_random(
        //         &serde_json::json!({
        //             "Green": 0.5,
        //             "Orange": 0.2,
        //             "Blue": 0.3,
        //             "Purple": 0.1,
        //         })
        //         .as_object()
        //         .unwrap()
        //         .clone(),
        //     );
        // }

        logging::log_startup_info(GRID_WIDTH, GRID_HEIGHT, "sparse_genesis");

        (
            CellularApp {
                grid: Arc::new(Mutex::new(grid)),
                is_running: false,
                tick_count: 0,
                speed: 1.0,
                selected_preset: preset.name().to_string(),
                tick_accumulator: 0.0,
                metrics: Arc::new(Mutex::new(MetricsCollector::new())),
                last_tick_time: Instant::now(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Cellular Ecosystem Simulation")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        let preset = &self.selected_preset.clone();
        match message {
            Message::Play => {
                self.is_running = true;
            }
            Message::Pause => {
                self.is_running = false;
            }
            Message::Reset => {
                if let Ok(mut grid) = self.grid.lock() {
                    let width = grid.width;
                    let height = grid.height;
                    *grid = Grid::new(width, height);

                    // TODO:[better_pattern] : use the same pattern as above in new() fn
                    //
                    // Reinitialize with current preset
                    if let Some(densities) = presets::load_preset(preset) {
                        grid.initialize_random(&densities);
                    } else {
                        // Fallback to balanced
                        if let Some(densities) = presets::load_preset("balanced") {
                            grid.initialize_random(&densities);
                        }
                    }
                    self.tick_count = 0;
                }
                self.is_running = false;
            }
            Message::SpeedChanged(speed) => {
                self.speed = speed.max(0.1);
            }
            Message::PresetInputChanged(preset) => {
                self.selected_preset = preset;
            }
            Message::LoadPreset => {
                if let Ok(mut grid) = self.grid.lock() {
                    // TODO:[better_pattern] : use the same pattern as above in new() fn
                    if let Some(densities) = presets::load_preset(preset) {
                        grid.initialize_random(&densities);
                        self.tick_count = 0;
                    }
                }
            }
            Message::Tick => {
                if self.is_running {
                    // Accumulate tick time based on speed
                    self.tick_accumulator += self.speed;

                    // Execute ticks when accumulated time >= 1.0
                    while self.tick_accumulator >= 1.0 {
                        let tick_start = Instant::now();
                        if let Ok(mut grid) = self.grid.lock() {
                            rules::apply_rules(&mut grid);
                            self.tick_count += 1;

                            // Record tick performance
                            let tick_duration = tick_start.elapsed();
                            if let Ok(mut metrics) = self.metrics.lock() {
                                metrics.record_tick(tick_duration);
                                let cells = (grid.width as u64) * (grid.height as u64);
                                logging::log_tick_performance(
                                    self.tick_count,
                                    tick_duration.as_secs_f64() * 1000.0,
                                    cells,
                                );
                            }
                        }
                        self.tick_accumulator -= 1.0;
                    }
                }

                // Record frame and update metrics
                if let Ok(mut metrics) = self.metrics.lock() {
                    metrics.record_frame();
                    metrics.record_metrics(GRID_WIDTH);
                }
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.is_running {
            time::every(Duration::from_millis(16)).map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let preset_label = text("Preset:").size(16);
        let preset = &self.selected_preset.clone();
        let preset_input =
            text_input("Enter preset name", preset).on_input(Message::PresetInputChanged);

        let load_btn = button("Load Preset").on_press(Message::LoadPreset);

        let presets = row![preset_label, preset_input, load_btn]
            .spacing(10)
            .padding(10);

        let play_btn = button("▶ Play").on_press(Message::Play);

        let pause_btn = button("⏸ Pause").on_press(Message::Pause);

        let reset_btn = button("↻ Reset").on_press(Message::Reset);

        let speed_label = text(format!("Speed: {:.1}x", self.speed));
        let speed_slider =
            slider(0.1..=10.0, self.speed, Message::SpeedChanged).width(iced::Length::Fixed(200.0));

        let status = if self.is_running {
            text(format!("▶ Running | Ticks: {}", self.tick_count)).size(14)
        } else {
            text(format!("⏸ Paused | Ticks: {}", self.tick_count)).size(14)
        };

        let controls = row![play_btn, pause_btn, reset_btn, speed_label, speed_slider]
            .spacing(10)
            .padding(10);

        // Get metrics for display
        let metrics_text = if let Ok(metrics) = self.metrics.lock() {
            let status_str = metrics.get_status_string(self.tick_count, self.is_running);
            text(status_str).size(11)
        } else {
            text("Metrics unavailable").size(11)
        };

        let grid_display = GridDisplay::new(Arc::clone(&self.grid));

        let main_column =
            column![presets, controls, status, metrics_text, grid_display].spacing(10);

        container(main_column)
            .padding(10)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

fn main() -> iced::Result {
    CellularApp::run(Settings {
        window: window::Settings {
            size: iced::Size::new(1280.0, 1400.0),
            ..Default::default()
        },
        ..Default::default()
    })
}
