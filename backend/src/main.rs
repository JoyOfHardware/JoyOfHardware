use moon::*;

async fn frontend() -> Frontend {
    Frontend::new()
        .title("Joy Of Hardware")
        .append_to_head(include_str!("../favicon.html")) // realfavicongenerator.net
        .append_to_head(
            "
        <style>
            html {
                background-color: white;
            }
        </style>",
        )
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

#[moon::main]
async fn main() -> std::io::Result<()> {
    start(frontend, up_msg_handler, |_| {}).await
}
