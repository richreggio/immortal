use base64::{engine::general_purpose, Engine as _};
use gloo::console;
use gloo::storage::{errors::StorageError, LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use yew::prelude::*;

const STORAGE_KEY: &str = "myimmortalreincarnation";

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
enum SpiritualCultivation {
    Mortal,
    QiCondensation,
    FoundationEstablishment,
    CoreFormation,
    SpiritualSea,
    NascentSoul,
    SpiritualLord,
    SpiritualEmpyrean,
    DaoLord,
    DaoEmpyrean,
    SovereignEmpyrean,
}

impl SpiritualCultivation {
    fn value(&self) -> f64 {
        match self {
            SpiritualCultivation::Mortal => 0.01,
            SpiritualCultivation::QiCondensation => 0.05,
            SpiritualCultivation::FoundationEstablishment => 0.095,
            SpiritualCultivation::CoreFormation => 0.2,
            SpiritualCultivation::SpiritualSea => 0.5,
            SpiritualCultivation::NascentSoul => 1.0,
            SpiritualCultivation::SpiritualLord => 1.5,
            SpiritualCultivation::SpiritualEmpyrean => 5.0,
            SpiritualCultivation::DaoLord => 10.0,
            SpiritualCultivation::DaoEmpyrean => 20.0,
            SpiritualCultivation::SovereignEmpyrean => 100.0,
        }
    }

    fn text_value(&self) -> &str {
        match self {
            SpiritualCultivation::Mortal => "Mortal",
            SpiritualCultivation::QiCondensation => "Qi Condensation",
            SpiritualCultivation::FoundationEstablishment => "Foundation Establishment",
            SpiritualCultivation::CoreFormation => "Core Formation",
            SpiritualCultivation::SpiritualSea => "Spiritual Sea",
            SpiritualCultivation::NascentSoul => "Nascent Soul",
            SpiritualCultivation::SpiritualLord => "Spiritual Lord",
            SpiritualCultivation::SpiritualEmpyrean => "Spiritual Empyrean",
            SpiritualCultivation::DaoLord => "Dao Lord",
            SpiritualCultivation::DaoEmpyrean => "Dao Empyrean",
            SpiritualCultivation::SovereignEmpyrean => "Sovereign Empyrean",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
enum PhysicalCultivation {
    QiGathering,
    OrganRefinement,
    MeridianReforging,
}

impl PhysicalCultivation {
    fn text_value(&self) -> &str {
        match self {
            PhysicalCultivation::QiGathering => "Qi Gathering",
            PhysicalCultivation::OrganRefinement => "Organ Refinement",
            PhysicalCultivation::MeridianReforging => "Meridian Reforging",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
enum Quality {
    Worst,
    VeryHorrible,
    Horrible,
    VeryPoor,
    Poor,
    Average,
    Good,
    VeryGood,
    Great,
    VeryGreat,
    Best,
}

impl Quality {
    fn value(&self) -> f32 {
        match self {
            Quality::Worst => 0.1,
            Quality::VeryHorrible => 0.3,
            Quality::Horrible => 0.4,
            Quality::VeryPoor => 0.5,
            Quality::Poor => 0.8,
            Quality::Average => 1.0,
            Quality::Good => 1.2,
            Quality::VeryGood => 1.5,
            Quality::Great => 3.0,
            Quality::VeryGreat => 4.0,
            Quality::Best => 10.0,
        }
    }

    fn text_value(&self) -> &str {
        match self {
            Quality::Worst => "Worst",
            Quality::VeryHorrible => "Very Horrible",
            Quality::Horrible => "Horrible",
            Quality::VeryPoor => "Very Poor",
            Quality::Poor => "Poor",
            Quality::Average => "Average",
            Quality::Good => "Good",
            Quality::VeryGood => "Very Good",
            Quality::Great => "Great",
            Quality::VeryGreat => "Very Great",
            Quality::Best => "Best",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct Cultivator {
    vessel_qi: f64,
    vessel_cycles: u32,
    vessel_cultivation: PhysicalCultivation,
    spiritual_qi: f64,
    spiritual_harmony: Quality,
    spiritual_cultivation: SpiritualCultivation,
    meridians: Quality,
}

impl Default for Cultivator {
    fn default() -> Self {
        Self {
            vessel_qi: 0.0,
            vessel_cycles: 0,
            vessel_cultivation: PhysicalCultivation::QiGathering,
            spiritual_qi: 0.0,
            spiritual_harmony: Quality::Worst,
            spiritual_cultivation: SpiritualCultivation::Mortal,
            meridians: Quality::Worst,
        }
    }
}

pub enum Msg {
    Heartbeat,
    SaveGame,
    LoadGame,
    Spirit,
    Vessel,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
pub struct App {
    cultivator: Cultivator,
}

impl App {
    fn load_data() -> App {
        // Getting data from local storage
        let raw_data: Result<String, StorageError> = LocalStorage::get(STORAGE_KEY);

        if let Ok(raw) = raw_data {
            // Decode from base64, if decoding fails give the default state
            let decoded = general_purpose::STANDARD_NO_PAD
                .decode(raw)
                .unwrap_or(serde_json::to_string(&App::default()).unwrap().into_bytes());
            // Build data structure from string
            let data = serde_json::from_slice(&decoded).unwrap();
            return data;
        } else {
            App::default()
        }
    }

    fn save_data(data: &Self) {
        // Turn data structure into a string
        let raw = serde_json::to_string(data).unwrap();
        console::debug!(&raw);
        // base64 encode it
        let encoded = general_purpose::STANDARD_NO_PAD.encode(raw);
        console::debug!(&encoded);
        // Save to local storage
        LocalStorage::set(STORAGE_KEY, encoded).unwrap();
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App::load_data()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SaveGame => {
                let data = App {
                    cultivator: self.cultivator.clone(),
                };
                App::save_data(&data);
                true
            }
            Msg::LoadGame => {
                self.cultivator = App::load_data().cultivator;
                true
            }
            Msg::Heartbeat => todo!(),
            Msg::Spirit => {
                self.cultivator.spiritual_qi += 1.0 * self.cultivator.spiritual_cultivation.value();
                true // re-render component
            }
            Msg::Vessel => {
                self.cultivator.vessel_qi += 1.0;
                true // re-render component
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <>
                <h1>{ "My Immortal Reincarnation" }</h1>
                    <div class="status">
                        <h2>{ "Status" }</h2>
                        <p>{ format!{ "Level of Spiritual cultivation: {}", self.cultivator.spiritual_cultivation.text_value() } }</p>
                        <p>{ format!{ "Quality of Spiritual veins: {}", self.cultivator.spiritual_harmony.text_value() } }</p>
                        <p>{ format!{ "Spiritual qi : {:.2}", self.cultivator.spiritual_qi } }</p>
                        <button onclick={link.callback(|_| Msg::Spirit)}>{ "Cultivate Spirit" }</button>
                        <p>{ format!{ "Stage of Vessel cultivation: {}", self.cultivator.vessel_cultivation.text_value() } }</p>
                        <p>{ format!{ "Vessel qi : {:.2}", self.cultivator.vessel_qi } }</p>
                        <p>{ format!{ "Completed Vessel cycles: {}", self.cultivator.vessel_cycles } }</p>
                        <button onclick={link.callback(|_| Msg::Vessel)}>{ "Cultivate Vessel" }</button>
                        <p>{ format!{ "Quality of Meridians: {}", self.cultivator.meridians.text_value() } }</p>
                    </div>

                    <div class="settings">
                        <h2>{ "Settings" }</h2>
                        <button onclick={link.callback(|_| Msg::SaveGame)}>{ "Save Game" }</button>
                        <button onclick={link.callback(|_| Msg::LoadGame)}>{ "Load Game" }</button>
                    </div>
            </>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
