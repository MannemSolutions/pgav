extern crate args;
extern crate chrono;
extern crate getopts;
extern crate postgres;
use log::{debug, info, warn, LevelFilter};
use postgres::{error::SqlState, Client};
use simple_logger::SimpleLogger;
use std::time::Duration;

mod cli;
mod dsn;
mod generic;

fn create_table(client: &mut Client) {
    debug!(target: "create", "creating table");
    match client.query("create table pgavailability (last timestamp)", &[]) {
        Ok(_) => {
            debug!(target: "insert", "inserting timestamp 2");
            client
                .query("insert into pgavailability values(now())", &[])
                .unwrap();
        }
        Err(err) => {
            println!("Error occurred creating pgavailability: {}", err);
            panic!()
        }
    }
}

fn get_last_ts(client: &mut Client) -> Result<f32, Box<dyn std::error::Error>> {
    debug!(target: "select", "retrieving timestamp");
    match client.query(
        "select min(extract(epoch from now()-last))::real from pgavailability",
        &[],
    ) {
        Ok(rows) => {
            if let Some(row) = rows.into_iter().next() {
                let expired: f32 = row.get(0);
                return Ok(expired);
            }
        }
        Err(err) => match err.as_db_error() {
            Some(db_error) => {
                if db_error.code() == &SqlState::UNDEFINED_TABLE {
                    create_table(client);
                    return get_last_ts(client);
                }
            }
            None => return Err(err.into()),
        },
    }
    debug!(target: "insert", "inserting timestamp");
    client.query("insert into pgavailability values(now())", &[])?;
    get_last_ts(client)
}

fn update_ts(client: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    debug!(target: "update", "updating timestamp");
    client.query("update pgavailability set last = now()", &[])?;
    Ok(())
}

fn connect(dsn: dsn::Dsn, sleep: f32) -> Client {
    loop {
        match dsn.copy().client() {
            Ok(client) => return client,
            Err(err) => {
                debug!(target: "connect", "connection failed: {:?}, retry in {} sec.", err, sleep);
                let d: u64 = (1_000_000_f32 * sleep) as u64;
                std::thread::sleep(Duration::from_micros(d));
            }
        }
    }
}

fn main() {
    let args = cli::Params::get_args();
    if args.debug {
        SimpleLogger::new()
            .with_level(LevelFilter::Debug)
            .init()
            .unwrap();
    } else {
        SimpleLogger::new()
            .with_level(LevelFilter::Info)
            .init()
            .unwrap();
    }

    info!("Initializing");
    let sleep = args.sleepduration();
    let timeout = args.timeoutduration();
    info!("dsn: {}", args.as_dsn().debug());
    let mut client = connect(args.as_dsn(), sleep);
    let mut reconnect: bool = false;
    loop {
        match get_last_ts(&mut client) {
            Ok(expired) => {
                debug!(target: "check", "expired={}", expired);
                match update_ts(&mut client) {
                    Ok(_) => {}
                    Err(err) => warn!(target: "update", "failed: {:?}", err),
                }
                if expired > timeout {
                    warn!(target: "check", "timeout expired ({}>{})", expired, timeout);
                }
                if expired < sleep {
                    debug!(target: "sleep", "sleep {} seconds", sleep-expired);
                    let d: u64 = (1_000_000_f32 * (sleep - expired)) as u64;
                    std::thread::sleep(Duration::from_micros(d));
                }
            }
            Err(err) => {
                debug!("Error occurred getting last ts: {}", err);
                reconnect = true;
            }
        }
        if reconnect {
            client = connect(args.as_dsn(), sleep);
            reconnect = false;
        }
    }
}
