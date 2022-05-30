mod ico;

use firefox_rs::{list_tabs, FFResult, Tab};
use futures::future::join_all;
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
    icocache: ico::Cache,
}

impl Plugin {
    async fn search(&mut self, query: &str, responder: &mut Responder) -> FFResult<()> {
        let tabs = list_tabs()?;
        let query = query.to_ascii_lowercase();
        self.tabs = tabs
            .into_iter()
            .filter(|tab| {
                let title_lower = tab.title.to_ascii_lowercase();
                let mut title_tokens = title_lower.split_ascii_whitespace();
                let found_in_title = query
                    .split_ascii_whitespace()
                    .all(|token| title_tokens.any(|title_token| title_token.contains(token)));
                if found_in_title {
                    return true;
                }
                let url = tab.url.to_ascii_lowercase();
                let mut url_tokens = url.split("/");
                query
                    .split_ascii_whitespace()
                    .all(|token| url_tokens.any(|url_token| url_token.contains(token)))
            })
            .collect();

        // define async tasks to load icons
        let icotasks = self.tabs.iter().map(|tab| async {
            // for each tab, if it has an icon
            // try to load it
            match &tab.icon {
                // this will use cached loading
                Some(ico) => Some(self.icocache.load(ico).await),
                None => None,
            }
            // then if it exists; convert Result to Option
            .and_then(|res| {
                res.map_err(|e| {
                    let title = &tab.title;
                    log::error!("Failed to load icon for {title}: {e}");
                })
                .ok()
            })
        });
        // execute icon loading
        let icos = join_all(icotasks).await;

        let results = self
            .tabs
            .iter()
            .zip(icos)
            .enumerate()
            .map(|(i, (tab, icon))| PluginSearchResult {
                id: i as u32,
                name: tab.title.clone(),
                description: String::from("Firefox Tab"),
                icon,
                ..Default::default()
            })
            .map(PluginResponse::Append);
        for search in results {
            responder.send(search).await;
        }
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
    #[cfg(feature = "journald")]
    {
        systemd_journal_logger::init().expect("Failed to initialize journald backend");
        // enable log level debug if on debug compilation
        debug_assert!({
            log::set_max_level(log::LevelFilter::Debug);
            true
        });
    }

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
