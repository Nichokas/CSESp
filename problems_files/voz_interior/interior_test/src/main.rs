use async_trait::async_trait;
use clap::Parser;
use colored::Colorize;
use ezsockets::ClientConfig;
use serde::Deserialize;
use std::io::Write;
use std::process::{Command, Stdio};
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::time::{sleep, Duration, Instant};

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// Archivo que contiene el programa a probar
    #[arg(short, long)]
    archivo: String,

    /// Calificar el ejercicio
    #[arg(short, long, requires_all = &["nombre_de_alumno", "codigo_de_clase"])]
    calificar: bool,

    /// Nombre del alumno (requerido si se usa --calificar)
    #[arg(long, requires = "calificar")]
    nombre_de_alumno: Option<String>,

    /// Código de clase (requerido si se usa --calificar)
    #[arg(long, requires = "calificar")]
    codigo_de_clase: Option<String>,
}

struct Client {
    tx: UnboundedSender<ServerResponse>,
}

#[derive(Debug, Deserialize, Clone)]
struct Message {
    message: String,
}
#[derive(Deserialize, Debug, Clone)]
struct ServerResponse {
    identifier: String,
    message: Message,
}

#[async_trait]
impl ezsockets::ClientExt for Client {
    type Call = ();

    async fn on_text(&mut self, text: ezsockets::Utf8Bytes) -> Result<(), ezsockets::Error> {
        let txt = text.to_string();
        if !txt.contains(r#""type":"ping""#) && txt.contains(r#""identifier":"#) && txt.contains(r#""message":"#) {
            let response: ServerResponse = serde_json::from_str(&txt).unwrap();
            let _ = self.tx.send(response);
        }
        Ok(())
    }

    async fn on_binary(&mut self, bytes: ezsockets::Bytes) -> Result<(), ezsockets::Error> {
        println!("received bytes: {bytes:?}");
        Ok(())
    }

    async fn on_call(&mut self, call: Self::Call) -> Result<(), ezsockets::Error> {
        let () = call;
        Ok(())
    }
}


enum State {
    Initial,
    WaitingFirst,
    WaitingSecond,
    Done,
}


#[tokio::main]
async fn main() {
    let cli = Args::parse();
    let mut correct = 0;
    let p_name = cli.archivo.clone();
    let py_v = python_ver();
    let bien = " :)";
    let mal = " :(";
    if cli.calificar {
        let (tx, mut rx) = mpsc::unbounded_channel::<ServerResponse>();
        let config = ClientConfig::new("wss://csesp.nichokas.eu/submit_cli");
        let (handle, future) = ezsockets::connect(|_client| Client { tx }, config).await;
        tokio::spawn(async move {
            future.await.unwrap();
        });
        sleep(Duration::from_millis(500)).await;
        let start = Instant::now();
        handle.text(r#"{"command": "subscribe","identifier": "{\"channel\":\"SubmitCliChannel\"}"}"#).unwrap();
        sleep(Duration::from_millis(500)).await;
        handle.text(r#"{"command": "message","identifier": "{\"channel\":\"SubmitCliChannel\"}","data": "{\"message\":\"interior\"}"}"#).unwrap();
        let mut messages_received = 0;
        'outer: loop {
            if let Some(response) = rx.recv().await {
                if messages_received == 0 {
                    if response.message.message != "*/**/Connected to server/**/*" {
                        eprintln!("{}","Error al intentar conectarse al servidor, ¿hay una conexion de internet activa?".red());
                        break 'outer;
                    } else {
                        messages_received += 1;
                        sleep(Duration::from_millis(500)).await;
                    }
                } else if messages_received == 1 && response.message.message == "*/**/Problem selected/**/*" {
                    messages_received += 1;
                    sleep(Duration::from_millis(500)).await;
                } else if messages_received == 2 || messages_received == 4 {
                    let resl: String = python_driver(&py_v, &p_name, (response.clone().message.message + "\n").as_bytes());
                    let _ = handle.text(r#"{"command": "message","identifier": "{\"channel\":\"SubmitCliChannel\"}","data": "{\"message\":\""#.to_string() + &resl + r#"\"}"}"#);
                    messages_received += 1;
                    sleep(Duration::from_millis(500)).await;
                } else if messages_received == 3 || messages_received == 5 {
                    if response.message.message != "*/**/Correct/**/*" {
                        eprintln!("{}","El programa no funciona como deberia, para obtener mas informacion, corre este probrama sin el argumento -c o --calificar".red());
                        break 'outer;
                    }
                    println!("{} {}", "El programa respondio de manera correcta a la pregunta".green(), bien.green());
                    messages_received += 1;
                    sleep(Duration::from_millis(500)).await;
                } else if messages_received == 6 {
                    println!("Mandando informacion del alumno...");
                    messages_received += 1;
                    let _ = handle.text(r#"{"command": "message","identifier": "{\"channel\":\"SubmitCliChannel\"}","data": "{\"message\":\""#.to_owned() +
                        format!("{},{},{}",
                                cli.nombre_de_alumno.clone().unwrap(),
                                cli.codigo_de_clase.clone().unwrap(),
                                &p_name.strip_suffix(".py").unwrap_or(&p_name)
                        ).as_str() +
                        r#"\"}"}"#);
                    sleep(Duration::from_millis(500)).await;
                } else if messages_received == 7 {
                println!("{}",response.message.message);
                break 'outer;
            }
            }
        }

    }
    else {
        let mins = "Las minusculas siguen siendo minusculas".to_string();
        let mays = "Las mayusculas se convierten en minusculas".to_string();
        let scar = "Los caracteres especiales no han sido cambiados".to_string();


        println!("==== Probando {} ====",p_name);

        // prueba mins
        {
            let resl:String = python_driver(&py_v,&p_name,b"abcdefghijklmnopqrstuvwxyz\n");
            if resl.contains("abcdefghijklmnopqrstuvwxyz") {
                println!("{}", (mins + bien).green());
                correct += 1
            } else {
                println!("{}", (mins + mal).red());
                println!("Se esperaba que la salida fuera {} y fue {}", "abcdefghijklmnopqrstuvwxyz".green(),resl.red())
            }
        }

        // prueba mays
        {
            let resl:String = python_driver(&py_v,&p_name,b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\n");
            if resl.contains("abcdefghijklmnopqrstuvwxyz") {
                println!("{}", (mays+bien).green());
                correct += 1;
            } else {
                println!("{}", (mays+mal).red());
                println!("Se esperaba que la salida fuera {} y fue {}", "abcdefghijklmnopqrstuvwxyz".green(),resl.red())
            }
        }

        // prueba scar
        {
            let resl:String = python_driver(&py_v,&p_name,b":;,.-_?!1234567890=/$%@#\n");
            if resl.contains(":;,.-_?!1234567890=/$%@#") {
                println!("{}", (scar+bien).green());
                correct += 1;
            } else {
                println!("{}", (scar+mal).red());
                println!("Se esperaba que la salida fuera {} y fue {}", ":;,.-_?!1234567890=/$%@#".green(),resl.red())
            }
        }

        println!("==============================");
        println!("Pruebas correctas: {}/3",correct)
    }
}

fn python_ver() -> String {
    if let Ok(output) = Command::new("python").arg("--version").output() {
        if output.status.success() {
            return "python".to_string();
        }
    } else if let Ok(output) = Command::new("python3").arg("--version").output() {
        if output.status.success() {
            return "python3".to_string();
        }
    }
    panic!("No hay ninguna version de python instalada");
}

fn python_driver(py_v:&String,p_name:&String,input_buf:&[u8]) -> String {
    let mut python_process = Command::new(py_v).arg(p_name).stdin(Stdio::piped())
        .stdout(Stdio::piped()).spawn().unwrap();
    let stdin_p = python_process.stdin.as_mut().unwrap();
    stdin_p.write_all(input_buf).unwrap();
    let python_out = python_process.wait_with_output().unwrap();
    let python_out_str = String::from_utf8_lossy(&python_out.stdout);
    python_out_str.trim().to_string()
}