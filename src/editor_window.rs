use std::{borrow::Cow, fs};

use bevy_editor_pls::egui::Slider;
use bevy_inspector_egui::{bevy_inspector, reflect_inspector};

use bevy::{asset::io::AssetSource, prelude::*};
use bevy_editor_pls::{
    editor_window::EditorWindow, egui::epaint::tessellator::path, AddEditorWindow,
};

use crate::map::{Team, Tile, TileDescriptor};

pub fn setup(app: &mut App) {
    app.add_editor_window::<TileEditorWindow>();
}

struct TileEditorWindow;

#[derive(Default)]
struct TileEditorState {
    descriptor: Option<TileDescriptor>,
    prop_rules: Option<FindAndRep>,
    done: bool,
    path: String,
    error: TileError,
}

#[derive(Default)]
struct FindAndRep {
    find_file: String,
    rep_file: String,
    find_word: String,
    rep_word: String,
}

impl TileEditorState {
    fn error(&mut self, text: impl Into<Cow<'static, str>>, clear: ClearOn) {
        self.error.set(text, clear);
    }

    fn get_error(&self) -> &Option<Cow<'static, str>> {
        &self.error.text
    }

    fn clear(&mut self, event: ClearOn) {
        if self.error.clear == event {
            self.error.text = None;
        }
    }
}

#[derive(Default, PartialEq, Eq)]
enum ClearOn {
    #[default]
    SucceedNew,
    ReadDir,
}

#[derive(Default)]
struct TileError {
    clear: ClearOn,
    text: Option<Cow<'static, str>>,
}

impl TileError {
    fn set(&mut self, text: impl Into<Cow<'static, str>>, clear: ClearOn) {
        self.text = Some(text.into());
        self.clear = clear;
    }
}

impl EditorWindow for TileEditorWindow {
    const NAME: &'static str = "Tile Editor";
    const DEFAULT_SIZE: (f32, f32) = (100., 100.);
    type State = TileEditorState;

    fn ui(
        world: &mut World,
        mut cx: bevy_editor_pls::editor_window::EditorWindowContext,
        ui: &mut bevy_editor_pls::egui::Ui,
    ) {
        let Some(state) = cx.state_mut::<Self>() else {
            error!("State not Loaded");
            return;
        };
        let type_registry = world.resource::<AppTypeRegistry>().read();
        // display error
        if let Some(text) = &state.get_error() {
            ui.colored_label(bevy_editor_pls::egui::Color32::RED, text.to_owned());
        };
        if state.done && state.prop_rules.is_some() {
            state.prop_rules = None;
            state.done = false;
        }
        if let Some(rep) = &mut state.prop_rules {
            ui.text_edit_singleline(&mut rep.find_file);
            ui.text_edit_singleline(&mut rep.rep_file);
            ui.horizontal(|ui| {
                ui.label("Find");
                ui.text_edit_singleline(&mut rep.find_word);
            });
            ui.horizontal(|ui| {
                ui.label("Rep With");
                ui.text_edit_singleline(&mut rep.rep_word);
            });
            if ui.button("Go").clicked() {
                let Ok(mut dir) = fs::read_dir("assets/tiles") else {
                    state.error("Failed to read Dir", ClearOn::ReadDir);
                    return;
                };
                // state.clear(ClearOn::ReadDir);
                for file in dir {
                    let Ok(file) = file else {
                        continue;
                    };
                    let name = file.file_name();
                    let name = name.to_str().expect("file names to be ascii");
                    if !name.contains(&rep.find_file) {
                        continue;
                    }
                    let name = name.replace(&rep.find_file, &rep.rep_file);
                    let mut path = file.path();
                    path.pop();
                    path.push(name);
                    let Ok(data) = fs::read_to_string(file.path()) else {
                        error!("failed to read file {:?}", file.path());
                        continue;
                    };
                    let data = data.replace(&rep.find_word, &rep.rep_word);
                    fs::write(path, &data);
                    state.done = true;
                }
            }
        } else {
            if ui.button("New Rep").clicked() {
                state.prop_rules = Some(FindAndRep::default());
            };
        }

        ui.text_edit_singleline(&mut state.path);
        ui.horizontal(|ui| {
            if !state.path.is_empty() {
                if state.descriptor.is_some() && ui.button("Save").clicked() {
                    let mut path = state.path.clone();
                    if !path.ends_with(".tile") {
                        path.push_str(".tile");
                    }
                    if let Some(descriptor) = &state.descriptor {
                        if let Ok(data) =
                            ron::ser::to_string_pretty(descriptor, ron::ser::PrettyConfig::new())
                        {
                            let _ = std::fs::write(format!("assets/tiles/{}", path), data);
                        } else {
                            error!("Ron Failed to make data");
                        }
                    } else {
                        error!("Descriptor not found");
                    }
                }
                if ui.button("Load").clicked() {
                    let mut path = state.path.clone();
                    if !path.ends_with(".tile") {
                        path.push_str(".tile");
                    }
                    if let Ok(data) = std::fs::read_to_string(format!("assets/tiles/{}", path)) {
                        state.descriptor = ron::from_str(&data).ok();
                        state.clear(ClearOn::SucceedNew);
                    } else {
                        state.error("Failed to load path", ClearOn::SucceedNew);
                    };
                };
            };
            if state.descriptor.is_none() && ui.button("New").clicked() {
                state.path.clear();
                state.descriptor = Some(TileDescriptor::new());
                state.clear(ClearOn::SucceedNew);
            }
        });

        // pub struct TileDescriptor {
        //     priority: i8,
        //     tile: Tile,
        //     is_sold: bool,
        //     team: Team,
        //     can_be_solid: [bool; 8],
        //     must_be_solid: [bool; 8],
        //     variants: Vec<TileSprite>,
        // }

        let Some(descriptor) = &mut state.descriptor else {
            return;
        };
        ui.add(Slider::new(&mut descriptor.priority, -100..=100));
        ui.push_id("TILE", |ui| {
            reflect_inspector::ui_for_value(descriptor.tile.as_reflect_mut(), ui, &type_registry);
        });
        ui.checkbox(&mut descriptor.is_sold, "Solid");
        reflect_inspector::ui_for_value(descriptor.team.as_reflect_mut(), ui, &type_registry);
        ui.scope(|ui| {
            ui.label("Can Be Solid");
            ui.horizontal(|ui| {
                ui.checkbox(&mut descriptor.can_be_solid[0], "");
                ui.checkbox(&mut descriptor.can_be_solid[1], "");
                ui.checkbox(&mut descriptor.can_be_solid[2], "");
            });
            ui.horizontal(|ui| {
                ui.checkbox(&mut descriptor.can_be_solid[7], "");
                ui.label("   ");
                ui.checkbox(&mut descriptor.can_be_solid[3], "");
            });
            ui.horizontal(|ui| {
                ui.checkbox(&mut descriptor.can_be_solid[6], "");
                ui.checkbox(&mut descriptor.can_be_solid[5], "");
                ui.checkbox(&mut descriptor.can_be_solid[4], "");
            });
        });

        ui.scope(|ui| {
            ui.label("Must Be Solid");
            ui.horizontal(|ui| {
                ui.checkbox(&mut descriptor.must_be_solid[0], "");
                ui.checkbox(&mut descriptor.must_be_solid[1], "");
                ui.checkbox(&mut descriptor.must_be_solid[2], "");
            });
            ui.horizontal(|ui| {
                ui.checkbox(&mut descriptor.must_be_solid[7], "");
                ui.label("   ");
                ui.checkbox(&mut descriptor.must_be_solid[3], "");
            });
            ui.horizontal(|ui| {
                ui.checkbox(&mut descriptor.must_be_solid[6], "");
                ui.checkbox(&mut descriptor.must_be_solid[5], "");
                ui.checkbox(&mut descriptor.must_be_solid[4], "");
            });
        });
        ui.horizontal(|ui| {
            ui.label("Variants: ");
            reflect_inspector::ui_for_value(
                descriptor.variants.as_reflect_mut(),
                ui,
                &type_registry,
            );
        });
    }
}
