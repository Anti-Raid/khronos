use std::collections::HashSet;
use std::num::NonZeroU32;
use std::time::Duration;

use governor::clock::Clock;
use governor::{clock::QuantaClock, DefaultKeyedRateLimiter};

#[allow(dead_code)]
pub struct LuaRatelimits {
    pub clock: QuantaClock,
    pub global: Vec<DefaultKeyedRateLimiter<()>>,
    pub per_bucket: indexmap::IndexMap<String, Vec<DefaultKeyedRateLimiter<()>>>,
    pub global_ignore: HashSet<String>,
}

impl LuaRatelimits {
    pub fn create_quota(
        limit_per: NonZeroU32,
        limit_time: Duration,
    ) -> Result<governor::Quota, crate::Error> {
        let quota = governor::Quota::with_period(limit_time)
            .ok_or("Failed to create quota")?
            .allow_burst(limit_per);

        Ok(quota)
    }

    /// Creates a empty global_ignore set.
    pub fn create_empty_global_ignore() -> Result<HashSet<String>, crate::Error> {
        Ok(HashSet::new())
    }

    /// Creates a global_ignore set from a vector of strings.
    pub fn create_global_ignore(
        global_ignore: Vec<String>,
    ) -> Result<HashSet<String>, crate::Error> {
        let global_ignore_set: HashSet<String> = global_ignore.into_iter().collect();
        Ok(global_ignore_set)
    }

    pub fn check(&self, bucket: &str) -> Result<(), crate::Error> {
        // Check global ratelimits
        if !self.global_ignore.contains(bucket) {
            for global_lim in self.global.iter() {
                match global_lim.check_key(&()) {
                    Ok(()) => continue,
                    Err(wait) => {
                        return Err(format!(
                            "Global ratelimit hit for bucket '{}', wait time: {:?}",
                            bucket,
                            wait.wait_time_from(self.clock.now())
                        )
                        .into());
                    }
                };
            }
        }

        // Check per bucket ratelimits
        if let Some(per_bucket) = self.per_bucket.get(bucket) {
            for lim in per_bucket.iter() {
                match lim.check_key(&()) {
                    Ok(()) => continue,
                    Err(wait) => {
                        return Err(format!(
                            "Per bucket ratelimit hit for '{}', wait time: {:?}",
                            bucket,
                            wait.wait_time_from(self.clock.now())
                        )
                        .into());
                    }
                };
            }
        }

        Ok(())
    }
}
