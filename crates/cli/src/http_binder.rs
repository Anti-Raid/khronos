use axum::{http::Request, Router};
use hyper::body::Incoming;
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server,
};
use std::{convert::Infallible, path::PathBuf};
use tokio::net::UnixListener;
use tower_service::Service;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum CreateRpcServerBind {
    /// Bind to a specific address
    Address(String),
    /// Bind to a unix socket
    #[cfg(unix)]
    UnixSocket(String),
}

#[derive(Debug, Clone)]
pub struct CreateRpcServerOptions {
    /// The bind address for the RPC server
    pub bind: CreateRpcServerBind,
}

pub async fn start_rpc_server(
    opts: CreateRpcServerOptions,
    mut make_service: axum::routing::IntoMakeService<Router>,
    done_start: tokio::sync::oneshot::Sender<Result<(), String>>,
) {
    match opts.bind {
        CreateRpcServerBind::Address(addr) => {
            println!("Trying to bind to address: {addr}");
            let listener = match tokio::net::TcpListener::bind(addr).await {
                Ok(ok) => ok,
                Err(err) => {
                    let _ = done_start.send(Err(format!("failed to bind to address: {err:#}")));
                    return;
                }
            };

            let _ = done_start.send(Ok(()));

            loop {
                let (socket, _remote_addr) = match listener.accept().await {
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
                        eprintln!("Failed to serve connection: {err:#}");
                    }
                });
            }
        }
        #[cfg(unix)]
        CreateRpcServerBind::UnixSocket(path) => {
            let path = PathBuf::from(path);

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
                let (socket, _remote_addr) = match uds.accept().await {
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
