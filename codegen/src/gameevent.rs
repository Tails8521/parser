extern crate proc_macro;

use inflector::Inflector;
use lazy_static::lazy_static;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use std::collections::hash_map::HashMap;
use tf_demo_parser::demo::gameevent_gen::get_sizes;
use tf_demo_parser::demo::gamevent::{GameEventDefinition, GameEventValueType};
use tf_demo_parser::demo::parser::MessageHandler;
use tf_demo_parser::{Demo, ParserState};
use tf_demo_parser::{DemoParser, MessageType};

struct GameEventAnalyser;

impl MessageHandler for GameEventAnalyser {
    type Output = Vec<GameEventDefinition>;

    fn does_handle(_message_type: MessageType) -> bool {
        false
    }

    fn into_output(self, state: &ParserState) -> Self::Output {
        state.event_definitions.clone()
    }
}

fn should_box_event(name: &str) -> bool {
    lazy_static! {
        static ref SIZES: HashMap<&'static str, usize> = get_sizes();
    }

    SIZES.get(name).cloned().unwrap_or_default() > 120
}

fn get_type_name(ty: GameEventValueType) -> &'static str {
    match ty {
        GameEventValueType::String => "String",
        GameEventValueType::Float => "f32",
        GameEventValueType::Boolean => "bool",
        GameEventValueType::Byte => "u8",
        GameEventValueType::Local => "()",
        GameEventValueType::Long => "u32",
        GameEventValueType::Short => "u16",
        GameEventValueType::None => "()",
    }
}

fn get_entry_name(name: &str) -> String {
    lazy_static! {
        static ref REPLACEMENTS: Vec<(&'static str, &'static str)> = vec!(
            ("mapname", "map_name"),
            ("cvarname", "cvar_name"),
            ("cvarvalue", "cvar_value"),
            ("userid", "user_id"),
            ("networkid", "network_id"),
            ("teamid", "team_id"),
            ("teamname", "team_name"),
            ("oldteam", "old_team"),
            ("autoteam", "auto_team"),
            ("entindex", "ent_index"),
            ("weaponid", "weapon_id"),
            ("damagebit", "damage_bit"),
            ("customkill", "custom_kill"),
            ("logclassname", "log_class_name"),
            ("playerpenetratecount", "player_penetrate_count"),
            ("damageamount", "damage_amount"),
            ("showdisguisedcrit", "show_disguised_crit"),
            ("minicrit", "mini_crit"),
            ("allseecrit", "all_see_crit"),
            ("weaponid", "weapon_id"),
            ("bonuseffect", "bonus_effect"),
            ("teamonly", "team_only"),
            ("oldname", "old_name"),
            ("newname", "new_name"),
            ("hintmessage", "hint_message"),
            ("roundslimit", "rounds_limit"),
            ("timelimit", "time_limit"),
            ("fraglimit", "frag_limit"),
            ("numadvanced", "num_advanced"),
            ("numbronze", "num_bronze"),
            ("numsilver", "num_silver"),
            ("numgold", "num_gold"),
            ("oldmode", "old_mode"),
            ("newmode", "new_mode"),
            ("entityid", "entity_id"),
            ("winreason", "win_reason"),
            ("flagcaplimit", "flag_cap_limit"),
            ("cpname", "cp_name"),
            ("capteam", "cap_team"),
            ("captime", "cap_time"),
            ("eventtype", "event_type"),
            ("killstreak", "kill_stream"),
            ("forceupload", "force_upload"),
            ("targetid", "target_id"),
            ("isbuilder", "is_builder"),
            ("objecttype", "object_type"),
            ("namechange", "name_change"),
            ("readystate", "ready_state"),
            ("builderid", "builder_id"),
            ("recedetime", "recede_time"),
            ("ownerid", "owner_id"),
            ("sapperid", "sapper_id"),
            ("itemdef", "item_def"),
            ("bitfield", "bit_field"),
            ("playsound", "play_sound"),
            ("totalhits", "total_hits"),
            ("posx", "pos_x"),
            ("posy", "pos_y"),
            ("posz", "pos_z"),
            ("ineye", "in_eye"),
            ("maxplayers", "max_players"),
            ("levelname", "level_name"),
            ("isstrange", "is_strange"),
            ("isunusual", "is_unusual"),
            ("defindex", "definition_index"),
            ("matchgroup", "match_group")
        );
    }

    if name == "type" {
        return "kind".to_string();
    }

    let mut snake: String = name.to_snake_case();
    for (search, replace) in REPLACEMENTS.iter() {
        snake = snake.replace(search, replace);
    }

    snake
}

fn get_event_name(name: &str) -> String {
    lazy_static! {
        static ref REPLACEMENTS: Vec<(&'static str, &'static str)> = vec!(
            ("ReplayReplaysavailable", "ReplayReplaysAvailable"),
            ("ServerAddban", "ServerAddBan"),
            ("ServerRemoveban", "ServerRemoveBan"),
            ("ClientBeginconnect", "ClientBeginConnect"),
            ("ClientFullconnect", "ClientFullConnect"),
            ("PlayerChangename", "PlayerChangeName"),
            ("PlayerHintmessage", "PlayerHintMessage"),
            ("GameNewmap", "GameNewMap"),
            ("IntroNextcamera", "IntroNextCamera"),
            ("PlayerChangeclass", "PlayerChangeClass"),
            ("Updateimages", "UpdateImages"),
            ("Updatelayout", "UpdateLayout"),
            ("Updatecapping", "UpdateCapping"),
            ("Updateowner", "UpdateOwner"),
            ("Starttouch", "StartTouch"),
            ("Endtouch", "EndTouch"),
            ("FakeCaptureMult", "FakeCaptureMultiplier"),
            ("TeamplayWaitingAbouttoend", "TeamPlayWaitingAboutToEnd"),
            ("TeamplayPointStartcapture", "TeamPlayPointStartCapture"),
            ("FreezecamStarted", "FreezeCamStarted"),
            ("LocalplayerChangeteam", "LocalPlayerChangeTeam"),
            ("LocalplayerChangeclass", "LocalPlayerChangeClass"),
            ("LocalplayerChangedisguise", "LocalPlayerChangeDisguise"),
            ("FlagstatusUpdate", "FlagStatusUpdate"),
            ("TournamentEnablecountdown", "TournamentEnableCountdown"),
            ("PlayerCalledformedic", "PlayerCalledForMedic"),
            ("PlayerAskedforball", "PlayerAskedForBall"),
            ("LocalplayerBecameobserver", "LocalPlayerBecameObserver"),
            ("PlayerHealedmediccall", "PlayerHealedMedicCall"),
            ("ArenaMatchMaxstreak", "ArenaMatchMaxStreak"),
            ("StatsResetround", "StatsResetRound"),
            ("FishNotice_arm", "FishNoticeArm"),
            ("PlayerBonuspoints", "PlayerBonusPoints"),
            ("PlayerUsedPowerupBottle", "PlayerUsedPowerUpBottle"),
            ("ReplayStartrecord", "ReplayStartRecord"),
            ("ReplaySessioninfo", "ReplaySessionInfo"),
            ("ReplayEndrecord", "ReplayEndRecord"),
            ("ReplayServererror", "ReplayServerError"),
            ("Teamplay", "TeamPlay"),
            ("death", "Death"),
            ("panel", "Panel"),
            ("object", "Object"),
            ("update", "Update"),
            ("ready", "Ready"),
            ("Gameui", "GameUI"),
            ("onhit", "OnHit"),
            ("bymedic", "ByMedic"),
            ("Controlpoint", "ControlPoint"),
            ("Pipebomb", "PipeBomb"),
            ("Scorestats", "ScoreStats"),
            ("Creditbonus", "CreditBonus"),
            ("Sentrybuster", "SentryBuster"),
            ("Questlog", "QuestLog"),
            ("Localplayer", "LocalPlayer"),
            ("Minigame", "MiniGame"),
            ("Winlimit", "WinLimit"),
            ("Skillrating", "SkillRating"),
            ("Directhit", "DirectHit"),
            ("Chargedeployed", "ChargeDeployed"),
            ("Winddown", "WindDown"),
            ("Stealsandvich", "StealSandvich"),
            ("Pricesheet", "PriceSheet"),
            ("Teambalanced", "TeamBalanced"),
            ("Highfive", "HighFive"),
            ("Powerup", "PowerUp"),
            ("Hltv", "HLTV"),
            ("Changelevel", "ChangeLevel"),
            ("Rocketpack", "RocketPack"),
            ("Deadringer", "DeadRinger"),
            ("Mainmenu", "MainMenu"),
            ("Mmstats", "MMStats"),
        );
    }

    let mut snake: String = name.to_pascal_case();
    for (search, replace) in REPLACEMENTS.iter() {
        snake = snake.replace(search, replace);
    }

    snake
}

pub fn generate_game_events(demo: Demo) -> TokenStream {
    let (_, mut events) = DemoParser::new_with_analyser(demo.get_stream(), GameEventAnalyser)
        .parse()
        .unwrap();

    events.sort();

    let span = Span::call_site();

    let imports = quote!(
        use super::gamevent::{FromGameEventValue, FromRawGameEvent, GameEventValue, RawGameEvent};
        use crate::Result;
    );

    let event_definitions = events.iter().map(|event| {
        let fields = event.entries.iter().map(|entry| {
            let name = Ident::new(&get_entry_name(&entry.name), span);
            let ty = Ident::new(get_type_name(entry.kind), span);

            quote!(pub #name: #ty,)
        });

        let name = Ident::new(&format!("{}Event", get_event_name(&event.name)), span);

        let entry_constructors = event.entries.iter().map(|entry| {
            let name_str = get_entry_name(&entry.name);
            let name = Ident::new(&name_str, span);
            let ty = Ident::new(get_type_name(entry.kind), span);

            quote!(
                #name: #ty::from_value(iter.next(), #name_str)?,
            )
        });

        let iter = if event.entries.len() > 0 {
            quote!(
                let mut iter = values.into_iter();
            )
        } else {
            quote!()
        };

        let param_name = if event.entries.len() > 0 {
            quote!(values)
        } else {
            quote!(_values)
        };

        quote!(
            #[derive(Debug)]
            pub struct #name {
                #(#fields)*
            }

            impl FromRawGameEvent for #name {
                fn from_raw_event(#param_name: Vec<GameEventValue>) -> Result<Self> {
                    #iter

                    Ok(#name {
                        #(#entry_constructors)*
                    })
                }
            }
        )
    });

    let event_variants = events.iter().map(|event| {
        let name_str = get_event_name(&event.name);
        let name = Ident::new(&name_str, span);
        let struct_name = Ident::new(&format!("{}Event", name_str), span);

        if should_box_event(&name_str) {
            quote!(#name(Box<#struct_name>),)
        } else {
            quote!(#name(#struct_name),)
        }
    });

    let event_types = events.iter().map(|event| {
        let name_str = get_event_name(&event.name);
        let name = Ident::new(&name_str, span);
        let id = Literal::u16_unsuffixed(event.id.into());

        quote!(#name = #id,)
    });

    let type_from_names = events.iter().map(|event| {
        let name_str = &event.name;
        let variant_name = Ident::new(&get_event_name(&name_str), span);

        quote!(#name_str => GameEventType::#variant_name,)
    });

    let from_raw_events = events.iter().map(|event| {
        let name = get_event_name(&event.name);
        let variant_name = Ident::new(&name, span);
        let struct_name = Ident::new(&format!("{}Event", name), span);

        if should_box_event(&name) {
            quote!(
                GameEventType::#variant_name => {
                    GameEvent::#variant_name(<Box<#struct_name>>::from_raw_event(event.values)?)
                }
            )
        } else {
            quote!(
                GameEventType::#variant_name => {
                    GameEvent::#variant_name(#struct_name::from_raw_event(event.values)?)
                }
            )
        }
    });

    let sizes = events.iter().map(|event| {
        let name = get_event_name(&event.name);
        let struct_name = Ident::new(&format!("{}Event", name), span);

        quote!(
            (#name, std::mem::size_of::<#struct_name>())
        )
    });

    quote!(
        #imports

        #(#event_definitions)*

        #[derive(Debug)]
        pub enum GameEvent {
            #(#event_variants)*
            Unknown(RawGameEvent),
        }

        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum GameEventType {
            #(#event_types)*
            Unknown,
        }

        impl GameEventType {
            pub fn from_type_name(name: &str) -> Self {
                match name {
                    #(#type_from_names)*
                    _ => GameEventType::Unknown,
                }
            }
        }

        impl GameEvent {
            pub fn from_raw_event(event: RawGameEvent) -> Result<Self> {
                Ok(match event.event_type {
                    #(#from_raw_events)*
                    GameEventType::Unknown => GameEvent::Unknown(event),
                })
            }
        }

        pub fn get_sizes() -> std::collections::hash_map::HashMap<&'static str, usize> {
            vec![
                #(#sizes,)*
            ].into_iter().collect()
        }
    )
}
