// GUI VERSION

// TODO:
// ---------------------------------------------------
// -[/] *When these ↓ todos are complete we in beta ver!!!*
//
// -[x] More start settings and better start screen
// -[x] Better sim screen
// -[x] More stat tracking for end screen
// -[/] Epidemics
// ---------------------------------------------------

// #![windows_subsystem = "windows"] // Disables terminal on windows machines

use crate::simulation::*;
mod simulation;

use std::{
    convert::From,
    ops::RangeInclusive,
    time::{Duration, Instant},
    vec,
};

use eframe::egui;
use eframe::emath::Align;
use egui::{
    plot::{Line, Plot, PlotPoints},
    Color32, Pos2, Ui, Vec2, Visuals,
};

struct AppData {
    app_scale: f32,
    table_shown: bool,

    frame_time: Duration,
    frame_start: Instant,
    all_frame_times: Duration,
    num_of_frame_updates: u32,
}

fn load_icon(path: &str) -> eframe::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    eframe::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

fn main() {
    pub struct App {
        app: AppData,
        sim: Sim,
        sim_epidemic: Epidemic,
        sim_stats: simulation::SimStats,
    }

    impl App {
        fn better_button(&mut self, ui: &mut Ui, bool_data: bool, states: Vec<&str>) -> bool {
            if bool_data && ui.add(egui::Button::new(states[0])).clicked() {
                false
            } else if !bool_data && ui.add(egui::Button::new(states[1])).clicked() {
                true
            } else {
                bool_data
            }
        }
    }

    // The code which renders the application
    // This section is a wrapper to simulation which may be decoupled to increase performance itf
    impl eframe::App for App {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            ctx.set_pixels_per_point(self.app.app_scale);

            egui::CentralPanel::default().show(ctx, |ui| {
                // Bottom settings panel
                egui::TopBottomPanel::bottom("settings").show(ctx, |ui| {
                    // Left to right side ui elements
                    ui.with_layout(egui::Layout::left_to_right(Align::TOP), |ui| {
                        egui::CollapsingHeader::new(
                            egui::RichText::new(format!("THEME")).size(15.0),
                        )
                        .show(ui, egui::widgets::global_dark_light_mode_buttons);

                        egui::CollapsingHeader::new(
                            egui::RichText::new(format!("APPLICATION SIZE")).size(15.0),
                        )
                        .show(ui, |ui| {
                            ui.add(egui::Slider::new(
                                &mut self.app.app_scale,
                                RangeInclusive::new(0.5, 2.0),
                            ))
                        });
                        egui::CollapsingHeader::new(
                            egui::RichText::new(format!("DEV SETTINGS")).size(15.0),
                        )
                        .show(ui, |ui| {
                            self.sim.lover_fix = self.better_button(
                                ui,
                                self.sim.lover_fix,
                                vec!["Lover fix enabled", "Lover fix disabled"],
                            );
                        });

                        // Right to left side ui elements
                        ui.with_layout(egui::Layout::right_to_left(Align::TOP), |ui| {
                            if self.sim.months_to_sim != 0 && self.sim.start_settings_set {
                                ui.add(egui::Label::new(
                                    egui::RichText::new(format!("{:?}", self.app.frame_time))
                                        .size(15.0),
                                ));
                            }
                        });
                    });
                });

                // Setting the start settings
                if !self.sim.start_settings_set {
                    self.sim.start_months = self.sim.months_to_sim;

                    ui.add_space(5.0);
                    if ui.style_mut().visuals == Visuals::light() {
                        ui.label(
                            egui::RichText::new(format!("Pre-simulation settings"))
                                .size(60.0)
                                .color(Color32::from_rgb(102, 153, 255)),
                        );
                    } else {
                        ui.label(
                            egui::RichText::new(format!("Pre-simulation settings"))
                                .size(60.0)
                                .color(Color32::from_rgb(128, 255, 0)),
                        );
                    }
                    ui.separator();
                    ui.add_space(5.0);

                    egui::Grid::new("start_settings_1").show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(format!(
                                "Number of months to simulate(12 - 48000):"
                            ))
                            .size(30.0),
                        );
                        ui.add_sized(
                            [120.0, 40.0],
                            egui::DragValue::new(&mut self.sim.months_to_sim)
                                .clamp_range(RangeInclusive::new(12, 48000)),
                        );
                        ui.end_row();

                        ui.label(
                            egui::RichText::new(format!("Years: {}", self.sim.start_months / 12))
                                .size(25.0),
                        );
                        ui.end_row();
                    });

                    egui::Grid::new("start_settings_2").show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(format!(
                                "Number of pairs of people to begin with:"
                            ))
                            .size(30.0),
                        );
                        ui.add_sized(
                            [120.0, 40.0],
                            egui::DragValue::new(&mut self.sim.start_pairs_of_people)
                                .clamp_range(RangeInclusive::new(1, 1000)),
                        );
                        ui.end_row();
                    });
                    ui.add_space(15.0);

                    if ui
                        .add_sized([120.0, 40.0], egui::Button::new("Begin simulation"))
                        .clicked()
                    {
                        if !self.sim.start_people_created {
                            // Creates Adam and Eve
                            for _ in 0..self.sim.start_pairs_of_people {
                                let adam: Person = self.sim.create_person(Sex::Male);
                                let eve: Person = self.sim.create_person(Sex::Female);
                                self.sim.people.push(adam);
                                self.sim.people.push(eve);
                            }

                            self.sim.start_people_created = true;
                        }
                        self.sim.start_settings_set = true;
                    }
                }

                // Main sim update loop screen
                if self.sim.start_settings_set && self.sim.months_to_sim != 0 {
                    self.app.frame_start = Instant::now();

                    if self.sim.months_to_sim != 0 && self.sim.sim_running {
                        // Updating the sim

                        if self.sim.people.len() != 0 {
                            self.sim
                                .update_sim(&mut self.sim_epidemic, &mut self.sim_stats);

                            self.sim.months_to_sim -= 1;

                            // Graph data pushing
                            self.sim_stats.graph_data.push([
                                (self.sim.start_months as f64) - (self.sim.months_to_sim as f64),
                                self.sim.people.len() as f64,
                            ]);
                            self.sim_epidemic.stats.graph_data.push([
                                (self.sim.start_months as f64) - (self.sim.months_to_sim as f64),
                                self.sim_epidemic.stats.number_of_infected as f64,
                            ]);
                        } else {
                            ui.colored_label(Color32::from_rgb(222, 0, 0), "Population died :(");
                        }
                    }

                    ui.label(
                        egui::RichText::new(format!("Population: {}", self.sim.people.len()))
                            .size(125.0),
                    );

                    ui.label(
                        egui::RichText::new(format!(
                            "Months Passed: {}",
                            self.sim.start_months - self.sim.months_to_sim
                        ))
                        .size(25.0),
                    );

                    ui.label(
                        egui::RichText::new(format!("Months left: {}", self.sim.months_to_sim))
                            .size(15.0),
                    );

                    self.sim.sim_running =
                        self.better_button(ui, self.sim.sim_running, vec!["Playing", "Paused"]);

                    let mut end_sim = false;
                    end_sim = self.better_button(ui, end_sim, vec!["", "End simulation"]);
                    if end_sim {
                        self.sim.months_to_sim = 0;
                    }

                    self.app.table_shown = self.better_button(
                        ui,
                        self.app.table_shown,
                        vec!["Table enabled", "Table disabled"],
                    );

                    egui::CollapsingHeader::new(
                        egui::RichText::new(format!("Control epidemic")).size(15.0),
                    )
                    .show(ui, |ui| {
                        if !self.sim_epidemic.progress_epidemic {
                            egui::Grid::new("epidemic_settings").show(ui, |ui| {
                                ui.label(egui::RichText::new(format!(
                                    "Number of people to infect(1 - 100):"
                                )));
                                ui.add(
                                    egui::DragValue::new(
                                        &mut self.sim_epidemic.start_vals.num_of_people_to_infect,
                                    )
                                    .clamp_range(RangeInclusive::new(1, 100)),
                                );
                                ui.end_row();

                                ui.label(egui::RichText::new(format!("R number(0 - 20):")));
                                ui.add(
                                    egui::DragValue::new(
                                        &mut self.sim_epidemic.start_vals.r_number,
                                    )
                                    .clamp_range(RangeInclusive::new(0, 20)),
                                );
                                ui.end_row();

                                ui.label(egui::RichText::new(format!("Infectivity(0 - 1000):")));
                                ui.add(
                                    egui::DragValue::new(
                                        &mut self.sim_epidemic.start_vals.infectivity,
                                    )
                                    .clamp_range(RangeInclusive::new(0.0, 1000.0)),
                                );
                                ui.end_row();
                            });
                        }

                        self.sim_epidemic.progress_epidemic = self.better_button(
                            ui,
                            self.sim_epidemic.progress_epidemic,
                            vec!["Epidemic progressing", "Begin epidemic"],
                        );

                        if self.sim_epidemic.progress_epidemic {
                            if !self.sim_epidemic.progress_cure {
                                self.sim_epidemic.progress_cure = self.better_button(
                                    ui,
                                    self.sim_epidemic.progress_cure,
                                    vec!["", "Begin cure"],
                                );
                            } else if self.sim_epidemic.cure_produced {
                                ui.label(egui::RichText::new(format!("Cure complete!")).size(15.0));
                            } else {
                                ui.label(
                                    egui::RichText::new(format!("Cure progressing...")).size(15.0),
                                );
                            }

                            egui::ScrollArea::vertical()
                                .max_height(300.0)
                                .max_width(250.0)
                                .always_show_scroll(false)
                                .show(ui, |ui| {
                                    ui.label(
                                        egui::RichText::new(format!("{}", self.sim_epidemic))
                                            .size(15.0),
                                    );
                                });
                        }
                    });

                    // Plot which shows population through time
                    egui::Window::new("Plot - Population against months")
                        .default_pos(Pos2 { x: 200.0, y: 400.0 })
                        .show(ctx, |ui| {
                            let data: PlotPoints =
                                PlotPoints::new(self.sim_stats.graph_data.clone());
                            let line = Line::new(data);
                            Plot::new("Population against months")
                                .view_aspect(2.0)
                                .allow_drag(false)
                                .allow_scroll(false)
                                .allow_zoom(false)
                                .allow_boxed_zoom(false)
                                .allow_double_click_reset(false)
                                .show(ui, |plot_ui| plot_ui.line(line));
                        });

                    // Plot which shows number of infected people through time
                    if self.sim_epidemic.progress_epidemic {
                        egui::Window::new("Plot - Number of infected against months")
                            .default_pos(Pos2 { x: 7.0, y: 550.0 })
                            .show(ctx, |ui| {
                                let data: PlotPoints =
                                    PlotPoints::new(self.sim_epidemic.stats.graph_data.clone());
                                let line = Line::new(data);
                                Plot::new("plot")
                                    .view_aspect(2.0)
                                    .allow_drag(false)
                                    .allow_scroll(false)
                                    .allow_zoom(false)
                                    .allow_boxed_zoom(false)
                                    .allow_double_click_reset(false)
                                    .show(ui, |plot_ui| plot_ui.line(line));
                            });
                    }

                    // A table with all the people in the simulation
                    if self.app.table_shown {
                        egui::SidePanel::right("Table").show(ctx, |ui| {
                            let text_style = egui::TextStyle::Body;
                            let row_height = ui.text_style_height(&text_style);

                            egui::ScrollArea::vertical()
                                .stick_to_bottom(true)
                                .auto_shrink([false; 2])
                                .show_rows(
                                    ui,
                                    row_height,
                                    self.sim.people.len(),
                                    |ui, row_range| {
                                        for id in row_range {
                                            let text = format!(
                                                "[ID: {:?}] Name: {:?} |  Age: {:?} | Sex: {:?} | \
                                        Fertility: {:?} | Lover: {:?} | {:?} | Seed: {:?}",
                                                self.sim.people[id].id,
                                                self.sim.people[id].name,
                                                ((if self.sim.people[id].age.is_some() {
                                                    self.sim.people[id].age.unwrap() as f32
                                                } else {
                                                    0.0
                                                }) / 12.0)
                                                    as i32,
                                                self.sim.people[id].sex,
                                                self.sim.people[id].fertility,
                                                self.sim.people[id].lover,
                                                self.sim.people[id].epidemic,
                                                self.sim.people[id].seed
                                            );
                                            let collap_header_text =
                                                self.sim.people[id].name.to_string()
                                                    + " | Age: "
                                                    + &(((if self.sim.people[id].age.is_some() {
                                                        self.sim.people[id].age.unwrap() as f32
                                                    } else {
                                                        0.0
                                                    }) / 12.0)
                                                        as i32)
                                                        .to_string();
                                            ui.push_id(self.sim.people[id].id, |ui| {
                                                egui::CollapsingHeader::new(collap_header_text)
                                                    .open(Some(true))
                                                    .show(ui, |ui| {
                                                        ui.label(text);
                                                    });
                                                ui.separator();
                                            });
                                        }
                                    },
                                );
                        });
                    }

                    // Frame time calculations
                    self.app.frame_time = self.app.frame_start.elapsed();
                    self.app.all_frame_times += self.app.frame_start.elapsed();
                    self.app.num_of_frame_updates += 1;
                }

                // Simulation completion screen
                if self.sim.months_to_sim == 0 {
                    ui.add_space(5.0);
                    if ui.style_mut().visuals == Visuals::light() {
                        ui.label(
                            egui::RichText::new(format!("Simulation completed :)"))
                                .size(60.0)
                                .color(Color32::from_rgb(102, 153, 255)),
                        );
                    } else {
                        ui.label(
                            egui::RichText::new(format!("Simulation completed :)"))
                                .size(60.0)
                                .color(Color32::from_rgb(128, 255, 0)),
                        );
                    }
                    ui.separator();
                    ui.label(
                        egui::RichText::new(format!("Simulation stats:"))
                            .size(45.0)
                            .text_style(egui::TextStyle::Heading),
                    );
                    ui.label(
                        egui::RichText::new(format!("-Population: {}", self.sim.people.len()))
                            .size(30.0),
                    );

                    ui.label(
                        egui::RichText::new(format!(
                            "-Months Passed: {}\n
                            -Total people that ever existed: {}\n
                            -Total ever born: {}\n
                            -Total ever dead: {}",
                            self.sim.start_months - self.sim.months_to_sim,
                            self.sim_stats.people_born + self.sim_stats.people_dead,
                            self.sim_stats.people_born,
                            self.sim_stats.people_dead
                        ))
                        .size(25.0),
                    );
                    ui.separator();
                    ui.label(
                        egui::RichText::new(format!("Application stats:"))
                            .size(45.0)
                            .text_style(egui::TextStyle::Heading),
                    );
                    ui.label(
                        egui::RichText::new(format!(
                            "-Average frame time: {:?}",
                            self.app.all_frame_times / self.app.num_of_frame_updates
                        ))
                        .size(25.0),
                    );
                }

                ctx.request_repaint();
            });
        }
    }

    impl Default for App {
        fn default() -> Self {
            Self {
                sim: Sim {
                    people: vec![],
                    population: -1,

                    sim_running: true,
                    lover_fix: false,
                    months_to_sim: 2400,
                    start_months: 0,
                    start_settings_set: false,
                    start_people_created: false,
                    start_pairs_of_people: 5,
                },
                sim_epidemic: Epidemic::default(),

                // Checks for spawning Adam and Eve, months, start button, amount of pairs, etc
                app: AppData {
                    table_shown: false,
                    app_scale: 1.0,

                    frame_time: Duration::new(0, 0),
                    frame_start: Instant::now(),
                    all_frame_times: Duration::new(0, 0),
                    num_of_frame_updates: 0,
                },

                sim_stats: simulation::SimStats {
                    graph_data: vec![],

                    people_born: 0,
                    people_dead: 0,
                    average_lifespan: 0,
                    amount_of_lovers_total: 0,
                    average_fertility: 0,
                },
            }
        }
    }

    // Custom options
    let options = eframe::NativeOptions {
        maximized: true,
        initial_window_size: Option::from(Vec2::new(1500_f32, 750_f32)),
        min_window_size: Option::from(Vec2::new(600_f32, 400_f32)),
        vsync: true,
        hardware_acceleration: eframe::HardwareAcceleration::Required,
        follow_system_theme: true,
        default_theme: eframe::Theme::Dark,
        centered: true,
        icon_data: Some(load_icon("./PopSimLogo.png")),
        ..Default::default()
    };

    // Runs the application
    eframe::run_native("PopSim", options, Box::new(|_cc| Box::new(App::default()))).expect("OUCH");
}
