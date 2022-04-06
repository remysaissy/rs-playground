use std::collections::HashMap;
use std::env::args;
use clap::arg;
use rustyline::Editor;
use rustyline::error::ReadlineError;
// use repl_rs::{Command, Error, Parameter, Result, Value};
// use repl_rs::{Convert, Repl};
use crate::CliArguments;
use crate::settings::Settings;
//
// // Add two numbers.
// fn add<T>(args: HashMap<String, Value>, _context: &mut T) -> Result<Option<String>> {
//     let first: i32 = args["first"].convert()?;
//     let second: i32 = args["second"].convert()?;
//
//     Ok(Some((first + second).to_string()))
// }
//
// // Write "Hello"
// fn hello<T>(args: HashMap<String, Value>, _context: &mut T) -> Result<Option<String>> {
//     Ok(Some(format!("Hello, {}", args["who"])))
// }

// fn help<T>(args: HashMap<String, Value>, _context: &mut T) -> Result<Option<String>> {
//     Ok(Some((first + second).to_string()))
// }

// fn zookeeper<T>(args: HashMap<String, Value>, _context: &mut T) -> Result<Option<String>> {
//     let is_server: i32 = args["-server"].convert()?;
//     let second: i32 = args["second"].convert()?;
//
//     Ok(Some((first + second).to_string()))
// }

// help
// ZooKeeper -server host:port cmd args
// addauth scheme auth
// close
// config [-c] [-w] [-s]
// connect host:port
// create [-s] [-e] [-c] [-t ttl] path [data] [acl]
// delete [-v version] path
// deleteall path
// delquota [-n|-b] path
// get [-s] [-w] path
// getAcl [-s] path
// getAllChildrenNumber path
// getEphemerals path
// history
// listquota path
// ls [-s] [-w] [-R] path
// ls2 path [watch]
// printwatches on|off
// quit
// reconfig [-s] [-v version] [[-file path] | [-members serverID=host:port1:port2;port3[,...]*]] | [-add serverId=host:port1:port2;port3[,...]]* [-remove serverId[,...]*]
// redo cmdno
// removewatches path [-c|-d|-a] [-l]
// rmr path
// set [-s] [-v version] path data
// setAcl [-s] [-v version] [-R] path acl
// setquota -n|-b val path
// stat [-w] path
// sync path

pub fn run_repl(cli: &CliArguments, settings: &Settings) {
    let mut rl = Editor::<()>::new();
    // if rl.load_history("history.txt").is_err() {
    //     println!("No previous history.");
    // }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("Line: {}", line);
                let mut args = line.split_ascii_whitespace();
                let cmd = match args.next() {
                    Some(x) => x,
                    None => continue
                };
                match cmd {
                    "help" => println!("zooKeeper -server host:port -client-configuration properties-file cmd args
addWatch [-m mode] path # optional mode is one of [PERSISTENT, PERSISTENT_RECURSIVE] - default is PERSISTENT_RECURSIVE
addauth scheme auth
close
config [-c] [-w] [-s]
connect host:port
create [-s] [-e] [-c] [-t ttl] path [data] [acl]
delete [-v version] path
deleteall path [-b batch size]
delquota [-n|-b|-N|-B] path
get [-s] [-w] path
getAcl [-s] path
getAllChildrenNumber path
getEphemerals path
history
listquota path
ls [-s] [-w] [-R] path
printwatches on|off
quit
reconfig [-s] [-v version] [[-file path] | [-members serverID=host:port1:port2;port3[,...]*]] | [-add serverId=host:port1:port2;port3[,...]]* [-remove serverId[,...]*]
redo cmdno
removewatches path [-c|-d|-a] [-l]
set [-s] [-v version] path data
setAcl [-s] [-v version] [-R] path acl
setquota -n|-b|-N|-B val path
stat [-w] path
sync path
version
whoami"),
                    "zooKeeper" => {
                        // -server
                        args.next();
                        let host_port = args.next().unwrap();
                        // -client-configuration
                        args.next();
                        let properties_file = args.next().unwrap();
                        let command = args.next();
                        println!("--> zookeeper")
                    },
                    "addWatch" => {
                        let mode = match args.next() {
                            Some(x) => {
                                if x != "-m" {
                                    "PERSISTENT_RECURSIVE"
                                } else {
                                    match args.next() {
                                        Some(x) => {
                                            match x {
                                                "PERSISTENT" => "PERSISTENT",
                                                "PERSISTENT_RECURSIVE" => "PERSISTENT_RECURSIVE",
                                                _ => "PERSISTENT_RECURSIVE"
                                            }
                                        },
                                        None => "PERSISTENT_RECURSIVE"
                                    }
                                }
                            },
                            None => "PERSISTENT_RECURSIVE"
                        };
                        let path = args.next().unwrap();
                        println!("--> addWatch {} with path {}", mode, path);
                    },
                    "addauth" => println!("--> addauth"),
                    "close" => println!("--> close"),
                    "config" => println!("--> config"),
                    "connect" => println!("--> connect"),
                    "create" => println!("--> create"),
                    "delete" => println!("--> delete"),
                    "deleteall" => println!("--> deleteall"),
                    "delquota" => println!("--> delquota"),
                    "get" => println!("--> get"),
                    "getAcl" => println!("--> getAcl"),
                    "getAllChildrenNumber" => println!("--> getAllChildrenNumber"),
                    "getEphemerals" => println!("--> getEphemerals"),
                    "history" => println!("--> history"),
                    "listquota" => println!("--> listquota"),
                    "ls" => println!("--> ls"),
                    "printwatches" => println!("--> printwatches"),
                    "quit" => println!("--> quit"),
                    "reconfig" => println!("--> reconfig"),
                    "redo" => println!("--> redo"),
                    "removewatches" => println!("--> removewatches"),
                    "set" => println!("--> set"),
                    "setAcl" => println!("--> setAcl"),
                    "setquota" => println!("--> setquota"),
                    "stat" => println!("--> stat"),
                    "sync" => println!("--> sync"),
                    "version" => println!("--> version"),
                    "whoami" => println!("--> whoami"),
                    _ => println!("Unrecognized command. Use help for a list of available commands.")
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    // rl.save_history("history.txt").unwrap();
}
//
// zooKeeper -server host:port -client-configuration properties-file cmd args
// addWatch [-m mode] path # optional mode is one of [PERSISTENT, PERSISTENT_RECURSIVE] - default is PERSISTENT_RECURSIVE
// addauth scheme auth
// close
// config [-c] [-w] [-s]
// connect host:port
// create [-s] [-e] [-c] [-t ttl] path [data] [acl]
// delete [-v version] path
// deleteall path [-b batch size]
// delquota [-n|-b|-N|-B] path
// get [-s] [-w] path
// getAcl [-s] path
// getAllChildrenNumber path
// getEphemerals path
// history
// listquota path
// ls [-s] [-w] [-R] path
// printwatches on|off
// quit
// reconfig [-s] [-v version] [[-file path] | [-members serverID=host:port1:port2;port3[,...]*]] | [-add serverId=host:port1:port2;port3[,...]]* [-remove serverId[,...]*]
// redo cmdno
// removewatches path [-c|-d|-a] [-l]
// set [-s] [-v version] path data
// setAcl [-s] [-v version] [-R] path acl
// setquota -n|-b|-N|-B val path
// stat [-w] path
// sync path
// version
// whoami