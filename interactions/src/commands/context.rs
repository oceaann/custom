use std::collections::HashMap;
use std::ops::Index;
use std::str::FromStr;
use futures::TryFutureExt;
use twilight_model::application::interaction::application_command::{CommandData, CommandDataOption, CommandInteractionDataResolved, CommandOptionValue};
use twilight_model::application::interaction::application_command::CommandOptionValue::{SubCommand, SubCommandGroup};
use twilight_model::application::interaction::{ApplicationCommand, Interaction, InteractionData, MessageComponentInteraction};
use twilight_model::application::interaction::modal::ModalSubmitInteraction;
use twilight_model::guild::PartialMember;
use twilight_model::id::Id;
use twilight_model::id::marker::{ChannelMarker, GenericMarker, GuildMarker};
use twilight_model::user::User;
use utils::{ok_or_break, ok_or_skip};
use utils::errors::Error;
use crate::Application;

#[derive(Debug, Clone)]
pub struct InteractionContext {
    pub options: HashMap<String, CommandOptionValue>,
    pub command_vec: Vec<String>,
    pub command_text: String,
    pub custom_id: Option<String>,
    pub channel_id: Id<ChannelMarker>,
    pub member: PartialMember,
    pub user: User,
    pub resolved: CommandInteractionDataResolved,
    pub target_id: Option<Id<GenericMarker>>,
    pub guild_id: Id<GuildMarker>
}

impl Into<InteractionContext> for Interaction {
    fn into(self) -> Result<InteractionContext, Error> {
        let channel_id = self.channel_id.ok_or("There is no channel data")?;
        let member = self.member.ok_or("Commands can't be executed in the dm")?;
        let guild_id = self.guild_id.ok_or("Commands can't be executed in the dm")?;
        let user = self.user.ok_or("There is no user data")?;
        let data = self.data.ok_or("Interaction data is missing")?;

        let target_id = data.target_id();
        let custom_id = data.custom_id();

        let resolved = data.resolved().unwrap_or_else(|| {
            CommandInteractionDataResolved {
                attachments: HashMap::new(),
                channels: HashMap::new(),
                members: HashMap::new(),
                messages: HashMap::new(),
                roles: HashMap::new(),
                users: HashMap::new()
            }
        });

        let commands_vec = match data {
            InteractionData::ApplicationCommand(data) => {
                get_command_vec_from_application_command(data.name, data.options)
            }
            InteractionData::MessageComponent(data) => {
                vec![get_command_from_id(&data.custom_id)]
            }
            InteractionData::ModalSubmit(data) => {
                vec![get_command_from_id(&data.custom_id)]
            }
        };

        let command_text = commands_vec.join(" ");

        Ok(InteractionContext {
            options: Default::default(),
            command_vec,
            command_text,
            custom_id,
            channel_id,
            member,
            user,
            resolved,
            target_id,
            guild_id
        })
    }
}

fn get_command_from_id(custom_id: &String) -> Option<String> {
    let id_parts = custom_id.split(":").collect::<Vec<String>>();
    id_parts.get(2).copied()
}

fn get_command_vec_from_application_command(name: String, options: Vec<CommandDataOption>) -> Vec<String> {
    options.get(0).map(|data| {
        let mut command_vec = vec![name];
        push_subcommands_names(&mut command_vec, data.value.to_owned());
        command_vec
    }).unwrap_or_else(|| vec![name])
}

fn push_subcommands_names(before: &mut Vec<String>, value: CommandOptionValue) {
    let subcommand_value = match value {
        SubCommandGroup(value) => Some(value),
        SubCommand(value) => Some(value),
        _ => None
    };

    let option = subcommand_value.and_then(
        |value| value.get(0).copied()
    );

    if let Some(option) = option {
        before.push(option.name);
        if let SubCommandGroup(_) = value {
            push_subcommands_names(before, option.value)
        }
    }
}

trait InteractionDataHelpers {
    fn custom_id(self) -> Option<String>;
    fn target_id(self) -> Option<Id<GenericMarker>>;
    fn resolved(self) -> Option<CommandInteractionDataResolved>;
}

impl InteractionDataHelpers for InteractionData {
    fn custom_id(self) -> Option<String> {
        match self {
            InteractionData::MessageComponent(data) => Some(data.custom_id),
            InteractionData::ApplicationCommand(_) => None,
            InteractionData::ModalSubmit(_) => None
        }
    }

    fn target_id(self) -> Option<Id<GenericMarker>> {
        match self {
            InteractionData::ApplicationCommand(data) => data.target_id,
            InteractionData::MessageComponent(_) => {}
            InteractionData::ModalSubmit(_) => {}
        }
    }

    fn resolved(self) -> Option<CommandInteractionDataResolved> {
        match self {
            InteractionData::ApplicationCommand(data) => data.resolved,
            InteractionData::MessageComponent(_) => {}
            InteractionData::ModalSubmit(_) => {}
        }
    }
}