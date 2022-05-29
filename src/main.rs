use futures_lite::{AsyncWriteExt, StreamExt};
use pop_launcher::{
    async_stdin, async_stdout, json_input_stream, PluginResponse, PluginSearchResult, Request,
};
use std::io::Stdout;

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
struct Plugin {}

impl Plugin {
    async fn search(&mut self, _query: &str, responder: &mut Responder) {
        let search = PluginSearchResult {
            id: 0,
            name: "huachimingo".into(),
            description: "test".into(),
            ..Default::default()
        };
        let res = PluginResponse::Append(search);
        responder.send(res).await;
        responder.send(PluginResponse::Finished).await;
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut requests = json_input_stream(async_stdin());

    let mut plugin = Plugin::default();
    let mut responder = Responder::default();

    // main loop
    while let Some(req_res) = requests.next().await {
        match req_res {
            Ok(request) => match &request {
                Request::Search(query) => plugin.search(query, &mut responder).await,
                Request::Exit => break,
                unsup => log::debug!("Unsupported request: {unsup:?}"),
            },
            Err(e) => log::error!("Failed to parse request: {e}"),
        }
    }
}
