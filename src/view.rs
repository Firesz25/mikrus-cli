use anyhow::Result;
use std::io::Write;

pub const HELLO_MSG: &str = r#"
__  __ ___ _  ______  _   _ ____         ____ _     ___
|  \/  |_ _| |/ /  _ \| | | / ___|       / ___| |   |_ _|
| |\/| || || ' /| |_) | | | \___ \ _____| |   | |    | |
| |  | || || . \|  _ <| |_| |___) |_____| |___| |___ | |
|_|  |_|___|_|\_\_| \_.\___/|____/       \____|_____|___|
"#;

pub fn promt() -> Result<String> {
    let mut input = String::new();
    print!("> ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    Ok(input.trim().to_string())
}

pub fn print_help() {
    println!(
        "{}",
    r#"comand line interface to mikr.us api

    Klucz do API uzyskasz tutaj:
    https://mikr.us/panel/?a=api

    Użycie:
        [komenda]

    Dostępne komendy:

    info       - informacje o Twoim serwerze
    serwery    - listuje wszystkie Twoje serwery
    restart    - restartuje Twój serwer
    logs       - podgląd ostatnich logów [10 sztuk]
    log <id>  - podgląd konkretnego wpisu w logach (po ID)
    amfetamina - uruchamia amfetaminę na serwerze (zwiększenie parametrów)
    db         - zwraca dane dostępowe do baz danych
    exec <cmd> - wywołuje polecenie/polecenia
    stats      - statystyki użycia dysku, pamięci, uptime itp.
    porty      - zwraca przypisane do Twojego serwera porty TCP/UDP
    exit       - wyjście z programu
    help       - wyświetla pomoc

    Uwaga:
    Limit czasu dla polecenia 'exec' to 60s. Dłuższe polecenia skończą się niepowodzeniem.
    "#
    )
}
