use firefox_rs::{list_tabs, FFResult, Tab};
use futures_lite::{AsyncWriteExt, StreamExt};
use pop_launcher::{
    async_stdin, async_stdout, json_input_stream, PluginResponse, PluginSearchResult, Request,
};
use std::io::Stdout;

macro_rules! trycont {
    ($result:expr, $literal:expr) => {
        match $result {
            Ok(ok) => ok,
            Err(e) => {
                log::error!($literal, e);
                continue;
            }
        }
    };
}

struct Responder {
    out: blocking::Unblock<Stdout>,
}

impl Default for Responder {
    fn default() -> Self {
        Self {
            out: async_stdout(),
        }
    }
}

impl Responder {
    async fn send(&mut self, response: PluginResponse) {
        if let Ok(mut bytes) = serde_json::to_string(&response) {
            bytes.push('\n');
            let _w = self.out.write_all(bytes.as_bytes()).await;
        }
    }
}

#[derive(Default)]
struct Plugin {
    tabs: Vec<Tab>,
}

impl Plugin {
    async fn search(&mut self, _query: &str, responder: &mut Responder) -> FFResult<()> {
        self.tabs = list_tabs()?;
        let results = self
            .tabs
            .iter()
            .enumerate()
            .map(|(i, tab)| PluginSearchResult {
                id: i as u32,
                name: tab.title.clone(),
                description: "firefox tab".into(),
                ..Default::default()
            })
            .map(PluginResponse::Append);
        for search in results {
            responder.send(search).await;
        }
        // let search = PluginSearchResult {
        //     id: 0,
        //     name: "huachimingo".into(),
        //     description: "test".into(),
        //     ..Default::default()
        // };
        // let res = PluginResponse::Append(search);
        // responder.send(res).await;
        responder.send(PluginResponse::Finished).await;
        Ok(())
    }

    async fn activate(&self, i: u32, responder: &mut Responder) -> FFResult<()> {
        if let Some(tab) = self.tabs.get(i as usize) {
            tab.focus()?;
            responder.send(PluginResponse::Close).await;
        }
        Ok(())
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut requests = json_input_stream(async_stdin());

    let mut plugin = Plugin::default();
    let mut responder = Responder::default();

    // main loop
    while let Some(request_res) = requests.next().await {
        let request = trycont!(request_res, "Failed to parse request: {}");
        match request {
            Request::Search(query) => trycont!(
                plugin.search(&query, &mut responder).await,
                "Failed to search: {}"
            ),
            Request::Activate(i) => trycont!(
                plugin.activate(i, &mut responder).await,
                "Failed to activate: {}"
            ),
            Request::Exit => break,
            other => log::debug!("Unsupported request: {other:?}"),
        }
    }
}
