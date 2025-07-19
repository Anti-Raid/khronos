// https://github.com/seanmonstar/reqwest/blob/master/Cargo.toml

//! DNS resolution via the [hickory-resolver](https://github.com/hickory-dns/hickory-dns) crate

use hickory_resolver::{
    config::LookupIpStrategy, lookup_ip::LookupIpIntoIter, ResolveError, TokioResolver,
};

use std::net::SocketAddr;
use std::sync::Arc;
use std::{fmt, net::IpAddr};

use reqwest::dns::{Addrs, Name, Resolve, Resolving};

/// Wrapper around an `AsyncResolver`, which implements the `Resolve` trait.
#[derive(Debug, Clone)]
pub struct HickoryDnsResolver {
    state: Arc<TokioResolver>,
    allow_localhost: bool,
}

impl HickoryDnsResolver {
    /// Creates a new `HickoryDnsResolver` with the default configuration.
    pub fn new(allow_localhost: bool) -> Result<Self, HickoryDnsSystemConfError> {
        Ok(Self {
            state: Arc::new(new_resolver()?),
            allow_localhost,
        })
    }
}

struct SocketAddrs {
    iter: LookupIpIntoIter,
}

#[derive(Debug)]
pub struct HickoryDnsSystemConfError(ResolveError);

impl Resolve for HickoryDnsResolver {
    fn resolve(&self, name: Name) -> Resolving {
        let resolver = self.clone();
        Box::pin(async move {
            let lookup = resolver.state.lookup_ip(name.as_str()).await?;

            // Avoid localhost rebind attacks
            if !resolver.allow_localhost {
                for addr in lookup.iter() {
                    if is_not_allowed(&addr) {
                        // global includes localhost
                        return Err("localhost resolution is not allowed".into());
                    }
                }
            }

            let addrs: Addrs = Box::new(SocketAddrs {
                iter: lookup.into_iter(),
            });

            Ok(addrs)
        })
    }
}

impl Iterator for SocketAddrs {
    type Item = SocketAddr;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|ip_addr| SocketAddr::new(ip_addr, 0))
    }
}

/// Create a new resolver with the default configuration,
/// which reads from `/etc/resolve.conf`. The options are
/// overridden to look up for both IPv4 and IPv6 addresses
/// to work with "happy eyeballs" algorithm.
fn new_resolver() -> Result<TokioResolver, HickoryDnsSystemConfError> {
    let mut builder = TokioResolver::builder_tokio().map_err(HickoryDnsSystemConfError)?;
    builder.options_mut().ip_strategy = LookupIpStrategy::Ipv4AndIpv6;
    Ok(builder.build())
}

impl fmt::Display for HickoryDnsSystemConfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("error reading DNS system conf for hickory-dns")
    }
}

impl std::error::Error for HickoryDnsSystemConfError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

const fn is_not_allowed(addr: &IpAddr) -> bool {
    match addr {
        IpAddr::V4(addr) => {
            !(addr.octets()[0] == 0 // "This network"
            || addr.is_private()
            || addr.is_unspecified()
            || addr.is_loopback()
            || addr.is_link_local()
            // addresses reserved for future protocols (`192.0.0.0/24`)
            // .9 and .10 are documented as globally reachable so they're excluded
            || (
                addr.octets()[0] == 192 && addr.octets()[1] == 0 && addr.octets()[2] == 0
                && addr.octets()[3] != 9 && addr.octets()[3] != 10
            )
            || addr.is_documentation()
            || addr.is_broadcast())
        }
        IpAddr::V6(addr) => {
            !(addr.is_unicast_link_local()
                || addr.is_loopback()
                || addr.is_unique_local()
                || addr.is_unicast_link_local()
                || addr.is_unspecified())
        }
    }
}
