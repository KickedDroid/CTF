pub(crate) fn display_name() {
    println!(
        "
        ██████╗░███████╗███╗░░██╗████████╗███████╗░██████╗████████╗
        ██╔══██╗██╔════╝████╗░██║╚══██╔══╝██╔════╝██╔════╝╚══██╔══╝
        ██████╔╝█████╗░░██╔██╗██║░░░██║░░░█████╗░░╚█████╗░░░░██║░░░
        ██╔═══╝░██╔══╝░░██║╚████║░░░██║░░░██╔══╝░░░╚═══██╗░░░██║░░░
        ██║░░░░░███████╗██║░╚███║░░░██║░░░███████╗██████╔╝░░░██║░░░
        ╚═╝░░░░░╚══════╝╚═╝░░╚══╝░░░╚═╝░░░╚══════╝╚═════╝░░░░╚═╝░░░
        
        ░█████╗░██╗░░░██╗████████╗░█████╗░███╗░░░███╗░█████╗░████████╗██╗░█████╗░███╗░░██╗  ░██╗░░░░░░░██╗██╗████████╗██╗░░██╗
        ██╔══██╗██║░░░██║╚══██╔══╝██╔══██╗████╗░████║██╔══██╗╚══██╔══╝██║██╔══██╗████╗░██║  ░██║░░██╗░░██║██║╚══██╔══╝██║░░██║
        ███████║██║░░░██║░░░██║░░░██║░░██║██╔████╔██║███████║░░░██║░░░██║██║░░██║██╔██╗██║  ░╚██╗████╗██╔╝██║░░░██║░░░███████║
        ██╔══██║██║░░░██║░░░██║░░░██║░░██║██║╚██╔╝██║██╔══██║░░░██║░░░██║██║░░██║██║╚████║  ░░████╔═████║░██║░░░██║░░░██╔══██║
        ██║░░██║╚██████╔╝░░░██║░░░╚█████╔╝██║░╚═╝░██║██║░░██║░░░██║░░░██║╚█████╔╝██║░╚███║  ░░╚██╔╝░╚██╔╝░██║░░░██║░░░██║░░██║
        ╚═╝░░╚═╝░╚═════╝░░░░╚═╝░░░░╚════╝░╚═╝░░░░░╚═╝╚═╝░░╚═╝░░░╚═╝░░░╚═╝░╚════╝░╚═╝░░╚══╝  ░░░╚═╝░░░╚═╝░░╚═╝░░░╚═╝░░░╚═╝░░╚═╝
        
        ██████╗░██╗░░░██╗░██████╗████████╗
        ██╔══██╗██║░░░██║██╔════╝╚══██╔══╝
        ██████╔╝██║░░░██║╚█████╗░░░░██║░░░
        ██╔══██╗██║░░░██║░╚═══██╗░░░██║░░░
        ██║░░██║╚██████╔╝██████╔╝░░░██║░░░
        ╚═╝░░╚═╝░╚═════╝░╚═════╝░░░░╚═╝░░░"
    );
}

fn process_nmap_result(res: String)  {
    if res.contains("5432/tcp open  postgresql") {
        println!("Running Postgresql Enumeration")
    }
    if res.contains("pat") {

    }
}