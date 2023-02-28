use super::{install, is_installed};

pub fn upgrade(pkg: Vec<String>) -> crate::Result<()> {
    println!("Upgrading {:?}", pkg);

    if pkg.len() > 0 {
        let mut installed = vec![];
        let mut to_install = vec![];
        for p in pkg {
            if is_installed(&p) {
                installed.push(p);
            } else {
                to_install.push(p);
            }
        }

        if to_install.len() > 0 {
            println!(
                "Some of those packages are not installed. Installing them now"
            ); // TODO: confirmation needed
        }

        installed.append(&mut to_install);
        install::install(installed)?;
    }

    Ok(())
}
