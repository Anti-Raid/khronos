use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::CreateCommand};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateGuildCommand {
    pub data: CreateCommand,
}

impl ApiReq for CreateGuildCommand {
    type Resp = serde_json::Value;

    /// Creates a guild command via the configured Discord provider and returns the provider's JSON response.
    ///
    /// The returned value is the provider's JSON response describing the created command as `serde_json::Value`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use dapi::api::commands::CreateGuildCommand;
    /// # use dapi::commands::CreateCommand;
    /// # async fn run(ctx: &dapi::DiscordContext<impl dapi::DiscordProvider>) -> Result<(), Box<dyn std::error::Error>> {
    /// let req = CreateGuildCommand { data: CreateCommand::default() };
    /// let resp = req.execute(ctx).await?;
    /// // `resp` is a serde_json::Value containing the created command
    /// # Ok(())
    /// # }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        self.data.validate()?;

        let resp = this.controller()
            .create_guild_command(&self.data)
            .await?;

        Ok(resp)
    }

    /// Convert this request into the API enum variant for creating a guild command.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assume `CreateCommand` implements `Default`.
    /// let req = CreateGuildCommand { data: CreateCommand::default() };
    /// let api = req.to_apilist();
    /// match api {
    ///     crate::apilist::API::CreateGuildCommand(_) => {}
    ///     _ => panic!("expected CreateGuildCommand variant"),
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::CreateGuildCommand(self)
    }
}