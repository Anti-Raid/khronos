use std::num::NonZeroU32;
use std::time::Duration;

use governor::clock::Clock;
use governor::{clock::QuantaClock, DefaultKeyedRateLimiter};

#[allow(dead_code)]
pub struct LuaRatelimits {
    pub clock: QuantaClock,
    pub global: Vec<DefaultKeyedRateLimiter<()>>,
    pub per_bucket: indexmap::IndexMap<String, Vec<DefaultKeyedRateLimiter<()>>>,
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

    pub fn check(&self, bucket: &str) -> Result<(), crate::Error> {
        // Check global ratelimits
        if bucket != "antiraid_bulk_op" && bucket != "antiraid_bulk_op_wait" {
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
