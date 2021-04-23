use clap::{crate_version, App, AppSettings, Arg, ArgGroup};

pub fn build_cli() -> App<'static, 'static> {
    App::new("ac")
        .about("andrew's CLI")
        .version(crate_version!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("generate-zsh-completions")
                        .help("gen completions"))
        .subcommand(
            App::new("git")
                .about("some git helpers")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("remote")
                        .about("change that remote")
                        .group(
                            ArgGroup::with_name("remote_group")
                                .args(&["flip", "to-ssh", "to-https"])
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("flip")
                                .short("f")
                                .long("flip")
                                .help("flip the remote"),
                        )
                        .arg(
                            Arg::with_name("to-ssh")
                                .short("s")
                                .long("to-ssh")
                                .help("change the remote to ssh"),
                        )
                        .arg(
                            Arg::with_name("to-https")
                                .short("h")
                                .long("to-https")
                                .help("change the remote to https"),
                        ),
                )
                .subcommand(
                    App::new("status").about("status of repos")
                        .arg(
                           Arg::with_name("dir")
                            .short("d")
                            .long("dir")
                            .takes_value(true)
                            .help("the dir to check")
                            .required(true),
                        )
                        .arg(
                            Arg::with_name("modified")
                            .short("m")
                            .long("modified")
                            .required(false)
                            .takes_value(false)
                            .help("only display the modified repos if passed in, other wise will display all repos."),
                        ),
                ),
    )
}
