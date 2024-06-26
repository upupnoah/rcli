// use rcli::{process_csv, process_genpass, Opts, SubCommand};

use clap::Parser;

use rcli::{CmdExecutor, Opts}; // 因为需要使用 CmdExecutor trait 的 execute 方法, 所以导入

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init(); // init tracing
    let opts = Opts::parse();
    opts.cmd.execute().await?;

    // region:    --- Before refactoring with enum_dispatch
    // match opts.cmd {
    //     SubCommand::Csv(opts) => {
    //         process_csv(&opts.input, &opts.output, &opts.format)?;
    //     }
    //     SubCommand::GenPass(opts) => {
    //         let password = process_genpass(
    //             opts.length,
    //             opts.upper,
    //             opts.lower,
    //             opts.number,
    //             opts.symbol,
    //         )?;
    //         println!("Generate password: {:?}", password);
    //     }
    //     SubCommand::Base64(subcmd) => match subcmd {
    //         Base64Subcommand::Encode(opts) => {
    //             let mut reader = get_reader(&opts.input)?;
    //             process_encode(&mut reader, opts.format)?;
    //         }
    //         Base64Subcommand::Decode(opts) => {
    //             // get_reader 返回值是 Box 类型
    //             // Box 当做没有既可, 会自动 deref
    //             let mut reader = get_reader(&opts.input)?;
    //             process_decode(&mut reader, opts.format)?;
    //         }
    //     },
    //     SubCommand::Text(subcmd) => match subcmd {
    //         // region:    --- old code
    //         // TextSubcommand::Sign(opts) => match opts.format {
    //         //     TextSignFormat::Blake3 => {
    //         //         let mut reader = get_reader(&opts.input)?;
    //         //         let key = get_content(&opts.key)?;
    //         //         process_text_sign(&mut reader, &key, opts.format)?;
    //         //     }
    //         //     TextSignFormat::Ed25519 => {
    //         //         let mut reader = get_reader(&opts.input)?;
    //         //         let key = get_content(&opts.key)?;
    //         //         process_text_sign(&mut reader, &key, opts.format)?;
    //         //     }
    //         // },
    //         // endregion: --- old code
    //         TextSubcommand::Sign(opts) => {
    //             // 从 stdin 中输入要小心, 一定要使用 command + D 来结束输入(EOF)
    //             // 而且要按两次, 不然签出来的结果会不符合预期
    //             let mut reader = get_reader(&opts.input)?;
    //             // let mut reader = "hello\n".as_bytes();
    //             let key = get_content(&opts.key)?;
    //             let signature = process_text_sign(&mut reader, &key, opts.format)?;
    //             println!("Signature: {}", URL_SAFE_NO_PAD.encode(signature));
    //         }
    //         TextSubcommand::Verify(opts) => {
    //             let mut reader = get_reader(&opts.input)?;
    //             let key = get_content(&opts.key)?;
    //             // let sig = get_content(&opts.sig)?;
    //             let sig = opts.sig.as_bytes();
    //             let ret = process_text_verify(&mut reader, &key, sig, opts.format)?;
    //             println!("Verify: {}", ret);
    //         }
    //         TextSubcommand::Generate(opts) => {
    //             let map = process_text_key_generate(opts.format)?;
    //             // write to file
    //             for (k, v) in map {
    //                 // 文件创建在 output_path 下
    //                 //  make run ARGS="text generate --format ed25519 -o ./fixtures"
    //                 let filepath = opts.output_path.join(k);
    //                 std::fs::write(filepath, URL_SAFE_NO_PAD.encode(v))?;
    //             }
    //         }
    //     },
    //     SubCommand::Http(subcmd) => match subcmd {
    //         HttpSubCommand::Serve(opts) => {
    //             // dir and port, start http server
    //             // println!("{:?}", opts);
    //             process_http_serve(opts.dir, opts.port).await?;
    //         }
    //     },
    // }
    // endregion: --- Before refactoring with enum_dispatch

    Ok(())
}
