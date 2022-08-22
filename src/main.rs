use std::str::FromStr;

use anyhow::Context;
use chrono::{Offset, TimeZone, Utc};

use crate::config::{Config, APP_NAME};

mod app;
mod backend;
mod config;
mod event;
mod ical;
mod notifier;
mod util;
mod widget;

fn main() -> anyhow::Result<()> {
  env_logger::init();

  let mut config = Config::read_or_initialize()?;

  config.calendar_location = config
    .calendar_location
    .replace('~', &std::env::var("HOME")?);

  log::info!("Config loaded {:?}", &config);

  let timezone = if let Some(ref tz) = config.timezone {
    chrono_tz::Tz::from_str(tz)
      .map_err(|x| anyhow::anyhow!("{}", x))?
      .offset_from_utc_datetime(&Utc::now().naive_utc())
      .fix()
  } else {
    util::local_tz()
  };

  let local_backend = backend::LocalDirBuilder::default()
    .calendar(&config.calendar_name)
    .dir(&config.calendar_location)
    .build()?;

  let db_path = {
    let mut path = dirs::data_dir()
      .with_context(|| "Cannot find a directory to store data")?;
    path.push(format!("{APP_NAME}/{APP_NAME}.db"));
    path
  };
  let backend = backend::IndexedLocalDir::new(local_backend, db_path)?;

  let mut app = app::App::new(&config, 3, timezone, backend)?;

  app.load_events();

  let options = eframe::NativeOptions::default();
  eframe::run_native(Box::new(app), options);
}
