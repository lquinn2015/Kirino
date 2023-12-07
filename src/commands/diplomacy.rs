#![allow(non_snake_case)]
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;

use serenity::{
    async_trait,
    framework::standard::macros::{check, command, group, help, hook},
    framework::standard::{
        help_commands, Args, CommandGroup, CommandOptions, CommandResult, DispatchError,
        HelpOptions, Reason, StandardFramework,
    },
    model::channel::Message,
    model::gateway::Ready,
    model::id::UserId,
    prelude::*,
};

#[group]
#[prefixes("dns")]
#[description("Diplomacy and Strife API Interface")]
#[allowed_roles("dns")]
#[commands(nation, alliance_tech, nation_tech)]
pub struct Diplomacy;

const API_BASE: &str = "http://diplomacyandstrifeapi.com/api/";
lazy_static! {
    static ref API_KEY: String = format!(
        "APICode={}",
        env::var("DNS_API_KEY").expect("API key not found")
    );
}

#[derive(Serialize, Deserialize, Debug)]
struct NationData {
    Alliance: String,
    ProtectionTime: String,
    LastOnline: String,
    LeaderName: String,
    NationName: String,
    DateOfJoining: String,
    AllianceId: usize,
    NationId: usize,
    NumberOfProjects: usize,
    Score: f64,
    Infra: f64,
    Land: f64,
    CoreLand: f64,
    NonCoreLand: f64,
    Pop: f64,
    StabilityIndex: f64,
    WarIndex: f64,
    TechIndex: f64,
    EducationIndex: f64,
    CommerceIndex: f64,
    TransportationIndex: f64,
    EmploymentIndex: f64,
    PowerIndex: f64,
    Devastation: f64,
}

#[command]
#[allow(unused_mut)]
async fn nation(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let nid = args.parse::<u32>();
    if nid.is_err() {
        msg.reply(ctx, "Parse error").await?;
    }

    let url = format!("{}Nation?{}&&NationId={}", API_BASE, *API_KEY, nid?);
    println!("{url} {}", url);
    let body = reqwest::get(url).await?.text().await?;
    println!("{}", body);
    let nation = serde_json::from_str::<Vec<NationData>>(&body);
    match nation {
        Err(e) => {
            msg.reply(ctx, format!("Error {}", e)).await.ok();
        }
        Ok(nation) => {
            msg.channel_id
                .send_message(&ctx.http, |m| m.content(format!("{:?}", nation)))
                .await?;
        }
    }

    Ok(())
}

#[derive(Deserialize, Serialize, Debug)]
struct NationBuildings {
    NationName: String,
    NationId: usize,
    NuclearPlants: usize,
    SolarPlants: usize,
    WindPlants: usize,
    TraditionalPowerPlants: usize,
    SchoolDistricts: usize,
    Universitys: usize,
    ResearchCenters: usize,
    CommercialDistricts: usize,
    FactoryDistricts: usize,
    MiningDistricts: usize,
    EnterainmentDistricts: usize,
    ResidentialDistricts: usize,
    FuelExtractors: usize,
    Roads: usize,
    RailNetworks: usize,
    Ports: usize,
    Airports: usize,
    Subways: usize,
    NavalBases: usize,
    AirBases: usize,
    ArmyBases: usize,
    UraniumMines: usize,
    RareMetalMines: usize,
    RichMiningArea: usize,
    AncientRuin: usize,
    PrecursorMatrix: usize,
    PrecursorFabricators: usize,
    PrecursorCoreExtractors: usize,
    PrecursorZeroPointReactors: usize,
    PrecursorTeleportationHubs: usize,
    TotalSlots: usize,
    OpenSlots: usize,
}

#[command]
#[aliases(nbuild)]
#[allow(unused_mut)]
async fn nation_tech(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let nid = args.parse::<u32>();
    if nid.is_err() {
        msg.reply(ctx, "Parse error").await?;
    }

    let url = format!(
        "{}NationBuildings?{}&&NationId={}",
        API_BASE, *API_KEY, nid?
    );
    println!("{url} {}", url);
    let body = reqwest::get(url).await?.text().await?;
    println!("{}", body);
    let nation = serde_json::from_str::<Vec<NationBuildings>>(&body);
    match nation {
        Err(e) => {
            msg.reply(ctx, format!("Error {}", e)).await.ok();
        }
        Ok(nation) => {
            msg.channel_id
                .send_message(&ctx.http, |m| m.content(format!("{:?}", nation)))
                .await?;
        }
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct NationTech {
    nationID: u32,
    Commerce: u32,
    Mining: u32,
    Factory: u32,
    Robotics: u32,
    RareMetals: u32,
    UrbanPlanning: u32,
    LandReclaimation: u32,
    CivilEngineering: u32,
    FuelExtraction: u32,
    Transportation: u32,
    SkyscraperDevelopment: u32,
    RenewableEnergy: u32,
    Energy: u32,
    ScientificTheory: u32,
    Espionage: u32,
    CounterIntelligence: u32,
    Nuclear: u32,
    Rocketry: u32,
    SpaceExploration: u32,
    ArtificialIntelligence: u32,
    OrbitialConstuction: u32,
    SpaceColonoization: u32,
    EducationTechnologys: u32,
    Entertainment: u32,
    PrecursorTech: u32,
    InfantryEquipment: u32,
    OrdnanceDevolopment: u32,
    StealthTechnology: u32,
    NavalTechnology: u32,
    ArmourImprovment: u32,
    TankTechnology: u32,
    MechDevolopment: u32,
    SensorTechnology: u32,
    AerospaceDevelopment: u32,
    MilitaryOrganization: u32,
    VirtualReality: u32,
    ComputerTech: u32,
    AllianceId: u32,
    NationName: String,
}

#[command]
#[aliases("atech")]
async fn alliance_tech(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let url = format!("{}AllianceTech?{}", API_BASE, *API_KEY);
    println!("Url: {}", url);
    let body = reqwest::get(url).await?.text().await?;

    println!("{}", body);
    let alliance = serde_json::from_str::<Vec<NationTech>>(&body);

    match alliance {
        Err(e) => {
            println!("Error {}", e);
            msg.reply(ctx, format!("Error {}", e)).await.ok();
        }
        Ok(alliance) => {
            for n in alliance {
                let msg = msg
                    .channel_id
                    .send_message(&ctx.http, |m| m.content(format!("{:?}", n)))
                    .await;

                if let Err(why) = msg {
                    println!("Error with avatar fn {:?}", why);
                }
            }
        }
    }

    Ok(())
}
