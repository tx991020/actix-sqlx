use crate::state::*;
use crate::state::{redis::Client, KvPool, RedisConnectionManager};
use nonblock_logger::{
    log::{LevelFilter, Record},
    BaseFilter, BaseFormater, FixedLevel, JoinHandle, NonblockLogger,
};

use std::path::PathBuf;
use std::sync::Arc;
use wither::mongodb::Client as MgoClient;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    pub sql: String,
    pub redis: String,
    pub listen: String,
    pub jwt_priv: String,


}

impl Config {
    pub fn parse_from_file(file: &PathBuf) -> Self {
        use std::fs::read_to_string;

        info!("confp: {}", file.display());
        let confstr = read_to_string(file).expect("confile read");
        json5::from_str(&confstr).expect("confile deser")
    }
    pub async fn into_state(self) -> AppStateRaw {
        info!("config: {:?}", self);
        let sql = SqlPool::new(&self.sql).await.expect("sql open");
        let kvm =
            RedisConnectionManager::new(Client::open(self.redis.clone()).expect("redis open"));
        let kv = KvPool::builder().build(kvm);

        // let client_options = ClientOptions::parse("mongodb://localhost:27017").unwrap();
        // let client = MgoClient::with_options(client_options).unwrap();
        // let mgo = client.database("mydb1");
        // let mgo = MgoClient::with_uri_str("mongodb://localhost:27017/").await.unwrap().database("mydb1");


        Arc::new(State {
            config: self,
            sql,
            kv,
        })
    }
    // generate and show config string
    pub fn show() {
        let de: Self = Default::default();
        println!("{}", serde_json::to_string_pretty(&de).unwrap())
    }
}

pub fn version_with_gitif() -> &'static str {
    concat!(
    env!("CARGO_PKG_VERSION"),
    " ",
    env!("VERGEN_COMMIT_DATE"),
    ": ",
    env!("VERGEN_SHA_SHORT")
    )
}

#[derive(structopt::StructOpt, Debug)]
#[structopt(version = version_with_gitif())]
pub struct Opt {
    // /// Activate debug mode
    // #[structopt(short, long)]
    // debug: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    /// Output file
    #[structopt(
    short = "c",
    long = "config",
    parse(from_os_str),
    default_value = "template.json"
    )]
    pub config: PathBuf,
}

impl Opt {
    pub fn parse_from_args() -> (JoinHandle, Self) {
        use structopt::StructOpt;

        let opt: Self = Opt::from_args();

        let level = match opt.verbose {
            0 => LevelFilter::Warn,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _more => LevelFilter::Trace,
        };

        let formater = BaseFormater::new()
            .local(true)
            .color(true)
            .level(4)
            .formater(format);
        let filter = BaseFilter::new()
            .starts_with(true)
            .notfound(true)
            .max_level(level);

        let handle = NonblockLogger::new()
            .filter(filter)
            .unwrap()
            .formater(formater)
            .log_to_stdout()
            .map_err(|e| eprintln!("failed to init nonblock_logger: {:?}", e))
            .unwrap();

        info!("opt: {:?}", opt);

        (handle, opt)
    }
}


pub fn format(base: &BaseFormater, record: &Record) -> String {
    let level = FixedLevel::with_color(record.level(), base.color_get())
        .length(base.level_get())
        .into_colored()
        .into_coloredfg();

    format!(
        "[{} {}#{}:{} {}] {}\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S.%3f"),
        level,
        record.module_path().unwrap_or("*"),
        record.line().unwrap_or(0),
        nonblock_logger::current_thread_name(),
        record.args()
    )
}
