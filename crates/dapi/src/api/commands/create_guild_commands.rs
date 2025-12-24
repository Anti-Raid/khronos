use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::CreateCommand};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateGuildCommands {
    pub data: Vec<CreateCommand>,
}

impl ApiReq for CreateGuildCommands {
    type Resp = serde_json::Value;

    /// Validates each command and dispatches the batch to the provider to create guild commands.
    ///
    /// Performs validation for every `CreateCommand` in `self.data`. If all validations pass, calls the
    /// Discord provider controller to create the guild commands and returns the provider's JSON response.
    ///
    /// # Returns
    ///
    /// `serde_json::Value` containing the controller's response.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use crate::api::commands::CreateGuildCommands;
    /// # async fn example<T: crate::DiscordProvider>(ctx: &crate::DiscordContext<T>, req: CreateGuildCommands) -> Result<(), crate::Error> {
    /// let resp = req.execute(ctx).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        for cmd in &self.data {
            cmd.validate()?;
        }

        let resp = this.controller()
            .create_guild_commands(&self.data)
            .await?;

        Ok(resp)
    }

    /// Convert this `CreateGuildCommands` request into the global `API::CreateGuildCommands` enum variant.
    ///
    /// # Returns
    ///
    /// An `crate::apilist::API` value containing this `CreateGuildCommands` request.
    ///
    /// # Examples
    ///
    /// ```
    /// let req = CreateGuildCommands { data: Vec::new() };
    /// let api = req.to_apilist();
    /// match api {
    ///     crate::apilist::API::CreateGuildCommands(inner) => {
    ///         // `inner` is the original `CreateGuildCommands` value
    ///         let _ = inner;
    ///     }
    ///     _ => panic!("unexpected API variant"),
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::CreateGuildCommands(self)
    }
}