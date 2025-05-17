use colored::Colorize;
use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut correct = 0;
    let p_name = &args[1];
    let py_v = python_ver();
    let mins = "Las minusculas siguen siendo minusculas".to_string();
    let mays = "Las mayusculas se convierten en minusculas".to_string();
    let scar = "Los caracteres especiales no han sido cambiados".to_string();
    let bien = " :)";
    let mal = " :(";

    println!("==== Probando {} ====",p_name);

    // prueba mins
    {
        let mut python_process = Command::new(&py_v).arg(p_name).stdin(Stdio::piped())
            .stdout(Stdio::piped()).spawn().unwrap();
        let stdin_p = python_process.stdin.as_mut().unwrap();
        stdin_p.write_all(b"abcdefghijklmnopqrstuvwxyz\n").unwrap();
        let python_out = python_process.wait_with_output().unwrap();
        let python_out_str = String::from_utf8_lossy(&python_out.stdout);
        if python_out_str.trim() == "abcdefghijklmnopqrstuvwxyz" {
            println!("{}", (mins + bien).green());
            correct += 1
        } else {
            println!("{}", (mins + mal).red());
            println!("Se esperaba que la salida fuera {} y fue {}", "abcdefghijklmnopqrstuvwxyz".green(),python_out_str.red())
        }
    }

    // prueba mays
    {
        let mut python_process = Command::new(&py_v).arg(p_name).stdin(Stdio::piped())
            .stdout(Stdio::piped()).spawn().unwrap();
        let stdin_p = python_process.stdin.as_mut().unwrap();
        stdin_p.write_all(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ").unwrap();
        let python_out = python_process.wait_with_output().unwrap();
        let python_out_str = String::from_utf8_lossy(&python_out.stdout);
        if python_out_str.trim() == "abcdefghijklmnopqrstuvwxyz" {
            println!("{}", (mays+bien).green());
            correct += 1;
        } else {
            println!("{}", (mays+mal).red());
            println!("Se esperaba que la salida fuera {} y fue {}", "abcdefghijklmnopqrstuvwxyz".green(),python_out_str.red())
        }
    }
    
    // prueba scar
    {
        let mut python_process = Command::new(py_v).arg(p_name).stdin(Stdio::piped())
            .stdout(Stdio::piped()).spawn().unwrap();
        let stdin_p = python_process.stdin.as_mut().unwrap();
        stdin_p.write_all(b":;,.-_?!1234567890=/$%@#").unwrap();
        let python_out = python_process.wait_with_output().unwrap();
        let python_out_str = String::from_utf8_lossy(&python_out.stdout);
        if python_out_str.trim() == ":;,.-_?!1234567890=/$%@#" {
            println!("{}", (scar+bien).green());
            correct += 1;
        } else {
            println!("{}", (scar+mal).red());
            println!("Se esperaba que la salida fuera {} y fue {}", ":;,.-_?!1234567890=/$%@#".green(),python_out_str.red())
        }
    }
    
    println!("==============================");
    println!("Pruebas correctas: {}/3",correct)
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
