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

pub(crate) fn display_progressbar() -> indicatif::ProgressBar{
    let pb = indicatif::ProgressBar::new_spinner();
    
    pb.inc_length(20);
    return pb; 
}
