/// A HTTP client provider trait for making HTTP requests.
///
/// Note: This trait should not be implemented/returned by httpclient_provider if HTTP client support should not be available in the context.
#[allow(async_fn_in_trait)] // We don't want Send/Sync whatsoever in Khronos anyways
pub trait HTTPClientProvider: 'static + Clone {
    /// This should return an error if ratelimited
    fn attempt_action(&self, bucket: &str, url: &str) -> Result<(), crate::Error>;

    /// Returns the maximum number of redirects allowed for the HTTP client.
    fn max_redirects(&self) -> usize {
        10 // Default value, can be overridden by implementations
    }

    /// Returns a domain whitelist for the HTTP client.
    fn domain_whitelist(&self) -> Vec<String> {
        Vec::with_capacity(0) // No domains = no whitelist
    }

    /// Returns a domain blacklist for the HTTP client.
    fn domain_blacklist(&self) -> Vec<String> {
        vec![
            "discord.com".to_string(),
            "discordapp.com".to_string(), // This should also block cdn.discordapp.com etc automatically
            "discord.gg".to_string(),
            "imgur.com".to_string(), // This should also block i.imgur.com etc automatically
            "tenor.com".to_string(), // This should also block i.tenor.com etc automatically
            "giphy.com".to_string(), // This should also block i.giphy.com etc
            "tenor.co".to_string(),  // This should also block i.tenor.co etc
            "giphy.co".to_string(),  // This should also block i.giphy.co etc
            "reddit.com".to_string(), // This should also block
            "redd.it".to_string(),   // This should also block i.redd.it etc
            "twitter.com".to_string(),
            "t.co".to_string(), // This should also block i.t.co etc
            "youtube.com".to_string(),
            "youtu.be".to_string(), // This should also block i.ytimg.com etc
            "youtube-nocookie.com".to_string(),
            "youtube-nocookie.be".to_string(), // This should also block i.ytimg.com etc
            "twitch.tv".to_string(),
            "twitchcdn.net".to_string(), // This should also block i.twitchcdn.net etc
            "twitchstatic.com".to_string(), // This should also block i.twitchstatic.com etc
            "twitchvideo.net".to_string(), // This should also block i.twitchvideo.net etc
            "twitchusercontent.com".to_string(),
            "roblox.com".to_string(),
        ]
    }

    /// Whether or not requests to localhost should be allowed.
    fn allow_localhost(&self) -> bool {
        false // Default value, can be overridden by implementations
    }
}
