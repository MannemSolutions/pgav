use crate::dsn::Dsn;
use crate::generic;
use duration_string::DurationString;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.

#[derive(StructOpt)]
#[structopt(about = "I detect availability issues and recovery times. Pass `-h` for more info.")]
pub struct Params {
    /// debug
    #[structopt(short, long, help = "Debugmode")]
    pub debug: bool,
    /// timeout
    #[structopt(
        default_value,
        short,
        long,
        help = "If last date in table is longer ago than this number of seconds, we log it"
    )]
    pub timeout: String,
    /// sleeptime
    #[structopt(
        default_value,
        short,
        long,
        help = "Time between checks. Decrease to be more finegrained, increase to bring down logging"
    )]
    pub sleeptime: String,
    /// Connection string
    #[structopt(
        default_value,
        short = "D",
        long,
        help = "the DSN to connect to (or use env vars PG...)"
    )]
    pub dsn: String,
}

impl Params {
    fn from_args() -> Params {
        <Params as StructOpt>::from_args()
    }
    pub fn get_args() -> Params {
        let mut args = Params::from_args();
        args.sleeptime = generic::get_env_str(
            &args.sleeptime,
            &String::from("PGAVSLEEPTIME"),
            &String::from("5s"),
        );
        args.timeout = generic::get_env_str(
            &args.timeout,
            &String::from("PGAVTIMEOUT"),
            &String::from("10s"),
        );
        args.dsn = generic::get_env_str(&args.dsn, &String::from("PGAVDSN"), "");
        args.debug = generic::get_env_bool(args.debug, &String::from("PGAVDEBUG"));
        args
    }
    pub fn as_dsn(&self) -> Dsn {
        Dsn::from_string(self.dsn.as_str())
    }
    pub fn sleepduration(&self) -> f32 {
        match DurationString::from_string(self.sleeptime.clone()) {
            Ok(ds) => match chrono::Duration::from_std(ds.into()) {
                Ok(duration) => {
                    let mut sleeptime =
                        (duration.num_microseconds().unwrap() as f32) / 1_000_000_f32;
                    let timeout = self.timeoutduration();
                    if sleeptime > timeout {
                        sleeptime = timeout;
                    }
                    sleeptime
                }
                Err(_) => panic!(
                    "invalid value for max_wait: {} is not a Duration",
                    self.sleeptime
                ),
            },
            Err(_) => panic!(
                "invalid value for max_wait: {} is not a Duration",
                self.sleeptime
            ),
        }
    }
    pub fn timeoutduration(&self) -> f32 {
        match DurationString::from_string(self.timeout.clone()) {
            Ok(ds) => match chrono::Duration::from_std(ds.into()) {
                Ok(duration) => (duration.num_microseconds().unwrap() as f32) / 1_000_000_f32,
                Err(_) => panic!(
                    "invalid value for max_wait: {} is not a Duration",
                    self.sleeptime
                ),
            },
            Err(_) => panic!(
                "invalid value for max_wait: {} is not a Duration",
                self.sleeptime
            ),
        }
    }
}
