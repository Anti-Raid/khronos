use axum::{http::Request, Router};
use hyper::body::Incoming;
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server,
};
use std::net::SocketAddr;
use std::{convert::Infallible, path::PathBuf};
use tokio::net::{TcpListener, UnixListener};
use tower::{Service, ServiceExt};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum CreateRpcServerBind {
    /// Bind to a specific address
    Address(SocketAddr),
    /// Bind to a unix socket
    #[cfg(unix)]
    UnixSocket(PathBuf),
}

#[derive(Debug, Clone)]
pub struct CreateRpcServerOptions {
    /// The bind address for the RPC server
    pub bind: CreateRpcServerBind,
}

pub async fn start_rpc_server(
    opts: CreateRpcServerOptions,
    router: Router<()>,
    done_start: tokio::sync::oneshot::Sender<Result<(), String>>,
    mut stop_chan: tokio::sync::watch::Receiver<()>,
) {
    match opts.bind {
        CreateRpcServerBind::Address(addr) => {
            let mut make_service = router.into_make_service_with_connect_info::<SocketAddr>();

            let listener = match TcpListener::bind(addr).await {
                Ok(ok) => ok,
                Err(err) => {
                    let _ = done_start.send(Err(format!("failed to bind to address: {err:#}")));
                    return;
                }
            };

            let _ = done_start.send(Ok(()));

            loop {
                tokio::select! {
                    recv = listener.accept() => {
                        let (socket, remote_addr) = match recv {
                            Ok(ok) => ok,
                            Err(err) => {
                                eprintln!("failed to accept connection: {err:#}");
                                continue;
                            }
                        };

                        let tower_service = unwrap_infallible(make_service.call(remote_addr).await);

                        tokio::spawn(async move {
                            let socket = TokioIo::new(socket);

                            let hyper_service =
                                hyper::service::service_fn(move |request: Request<Incoming>| {
                                    tower_service.clone().oneshot(request)
                                });

                            if let Err(err) = server::conn::auto::Builder::new(TokioExecutor::new())
                                .serve_connection_with_upgrades(socket, hyper_service)
                                .await
                            {
                                eprintln!("Failed to serve connection: {err:#}");
                            }
                        });
                    },
                    _ = stop_chan.changed() => {
                        break;
                    }
                }
            }
        }
        #[cfg(unix)]
        CreateRpcServerBind::UnixSocket(path) => {
            let mut make_service = router.into_make_service();

            let _ = tokio::fs::remove_file(&path).await;

            let Some(parent) = path.parent() else {
                let _ = done_start.send(Err(
                    "Parent directory of unix socket path does not exist".into()
                ));
                return;
            };

            if let Err(e) = tokio::fs::create_dir_all(parent).await {
                let _ = done_start.send(Err(format!("failed to create parent directory: {e}")));
                return;
            }

            let uds = match UnixListener::bind(path.clone()) {
                Ok(ok) => ok,
                Err(err) => {
                    let _ = done_start.send(Err(format!("failed to bind to unix socket: {err:#}")));
                    return;
                }
            };

            loop {
                tokio::select! {
                    recv = uds.accept() => {
                        let (socket, _remote_addr) = match recv {
                            Ok(ok) => ok,
                            Err(err) => {
                                eprintln!("failed to accept connection: {err:#}");
                                continue;
                            }
                        };

                        let tower_service = unwrap_infallible(make_service.call(&socket).await);

                        tokio::spawn(async move {
                            let socket = TokioIo::new(socket);

                            let hyper_service =
                                hyper::service::service_fn(move |request: Request<Incoming>| {
                                    tower_service.clone().call(request)
                                });

                            if let Err(err) = server::conn::auto::Builder::new(TokioExecutor::new())
                                .serve_connection_with_upgrades(socket, hyper_service)
                                .await
                            {
                                eprintln!("failed to serve connection: {err:#}");
                            }
                        });
                    },
                    _ = stop_chan.changed() => {
                        break;
                    },
                }
            }
        }
    }
}

fn unwrap_infallible<T>(result: Result<T, Infallible>) -> T {
    match result {
        Ok(value) => value,
        #[allow(unreachable_patterns)]
        Err(never) => match never {},
    }
}
