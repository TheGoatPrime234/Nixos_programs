use std::process::Command;

pub fn init_git_email(ip: &str) {
    info!("[ RUN ] - Git email wird deklariert");

    let git = Command::new("ssh")
        .arg(get_sshstring(ip, User::Cato))
        .args(["git", "config", "--global", "user.email", "cato.jenisch@gmail.com"])
        .output()
        .unwrap_or_else(|err| { 
            error!("Konnte git nicht starten: {}", err); 
            process::exit(1); 
        });
    if !git.status.success() {
        error!("[ FAILED ] - Konnte in Git die Email nicht festlegen: {}", String::from_utf8_lossy(&git.stderr));
        process::exit(1);
    };
    info!("[ OK ] - Git email deklariert");
}

pub fn init_git_name(ip: &str) {
    info!("[ RUN ] - Git name wird deklariert");

    let git = Command::new("ssh")
        .arg(get_sshstring(ip, User::Cato))
        .args(["git", "config", "--global", "user.name", "Xeravus"])
        .output()
        .unwrap_or_else(|err| { 
            error!("Konnte git nicht starten: {}", err); 
            process::exit(1); 
        });
    if !git.status.success() {
        error!("[ FAILED ] - Konnte in Git den Namen nicht festlegen: {}", String::from_utf8_lossy(&git.stderr));
        process::exit(1);
    };
    info!("[ OK ] - Git name deklariert");
}

pub fn init_github(ip: &str) {
    info!("[ RUN ] - Starte Verbindung zwischen Git und GitHub");

    let gh = Command::new("ssh")
        .arg(get_sshstring(ip, User::Cato))
        .args(["gh", "auth", "login", "--hostname", "github.com", "-w", "-p", "https"])
        .output()
        .unwrap_or_else(|err| { 
            error!("Konnte gh nicht starten: {}", err); 
            process::exit(1); 
        });
    if !gh.status.success() {
        error!("[ Failed ] - Konnte Git nicht mit GitHub verknüpfen: {}", String::from_utf8_lossy(&gh.stderr));
        process::exit(1);
    } else {
        debug!("GH Output: {}", String::from_utf8_lossy(&gh.stdout));
    }
    info!("[ OK ] - GitHub Verknüpfung erfolgreich");
}
