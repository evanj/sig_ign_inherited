use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let num_args = std::env::args().count();
    let mode_name = if num_args <= 1 { "parent" } else { "child" };
    println!("{mode_name} starting (num_args={num_args})");

    unsafe {
        let mut sigint_sigaction: libc::sigaction = std::mem::zeroed();
        let code = libc::sigaction(libc::SIGINT, std::ptr::null(), &mut sigint_sigaction);
        assert_eq!(code, 0);
        println!(
            "{mode_name} before sigint_sigaction.sa_sigaction={} (SIG_IGN={}, SIG_DFL={})",
            sigint_sigaction.sa_sigaction,
            libc::SIG_IGN,
            libc::SIG_DFL
        );
        println!(
            "{mode_name} before sigint_sigaction.sa_flags: {}",
            sigint_sigaction.sa_flags
        );

        sigint_sigaction = std::mem::zeroed();
        sigint_sigaction.sa_sigaction = libc::SIG_IGN;
        let code = libc::sigaction(libc::SIGINT, &sigint_sigaction, std::ptr::null_mut());
        assert_eq!(code, 0);
    }

    if mode_name == "parent" {
        println!("{mode_name} called sigaction to set SIGINT to SIG_IGN; spawning child ...");
        let exe_path = std::env::current_exe()?;
        let exit_status = Command::new(exe_path).arg("child").spawn()?.wait()?;
        assert!(exit_status.success());
    }

    println!("{mode_name} exiting");
    Ok(())
}
