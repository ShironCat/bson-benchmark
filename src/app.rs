use js_sys::Date;
use sauron::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Test {
  text: String,
  number: i32,
  vector: Vec<i32>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Msg {
  Click,
}

pub struct App {
  progress: u8,
  running: bool,
  test: Test,
}

impl App {
  pub fn new() -> Self {
    App {
      progress: 0,
      running: false,
      test: Test {
        text: "Lorem ipsum".to_owned(),
        number: 15,
        vector: vec![1, 2, 3, 4],
      },
    }
  }
}

impl Component<Msg> for App {
  fn view(&self) -> Node<Msg> {
    div!(
      [],
      [
        input!(
          [
            if self.running {
              disabled("")
            } else {
              value("Start")
            },
            r#type("button"),
            on_click(|_| {
              trace!("Button is pressed.");
              Msg::Click
            })
          ],
          []
        ),
        text!("{}", self.progress)
      ]
    )
  }

  fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
    trace!("App is updating from msg: {:?}", msg);
    match msg {
      Msg::Click => {
        trace!("Disabling button.");
        self.running = true;
        trace!("Initializing benchmark.");
        for i in 1..5 {
          info!("Run #{}:", i);
          let start = Date::now();
          for i in 0..500_000 {
            if (i % 50_000) == 0 {
              self.progress += 10
            }
            serde_json::from_str::<Test>(serde_json::to_string(&self.test).unwrap().as_str())
              .unwrap();
          }
          let stop = Date::now();
          debug!("JSON: {} ms", (stop - start));
          let start = Date::now();
          for _ in 0..500_000 {
            bson::from_bson::<Test>(bson::to_bson(&self.test).unwrap()).unwrap();
          }
          let stop = Date::now();
          debug!("BSON: {} ms", (stop - start));
        }
        Cmd::none()
      }
    }
  }
}
