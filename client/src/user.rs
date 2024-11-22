use std::env;

fn env_collector() -> Vec<String> {
    let mut env_values = env::args().collect::<Vec<String>>();
    env_values.remove(0);
    env_values
}

pub async fn user_interaction() {
    let env_values = env_collector();
    for (i, env_value) in env_values.iter().enumerate() {
        match env_value.as_str() {
            "read_all_packages" => {
                read_all_packages().await;
                return;
            }
            "read_package" => {
                let package_name = match env_values.get(i + 1) {
                    Some(package_name) => package_name,
                    None => {
                        eprintln!("Length is not enough");
                        return;
                    }
                };
                read_package(package_name).await;
                return;
            }
            "install_package" => {
                let package_name = match env_values.get(i + 1) {
                    Some(package_name) => package_name,
                    None => {
                        eprintln!("Length is not enough");
                        return;
                    }
                };
                install_package(package_name).await;
                return;
            }
            "delete_package" => {
                let package_name = match env_values.get(i + 1) {
                    Some(package_name) => package_name,
                    None => {
                        eprintln!("Length is not enough");
                        return;
                    }
                };
                delete_package(package_name).await;
                return;
            }
            _ => {
                eprintln!("Need an Argument");
                return;
            }
        }
    }
}

async fn read_all_packages() {
    let packages = crate::request::read_all_packages().await;
    match packages {
        Ok(packages) => {
            for package in packages {
                println!("{}", package.get_name());
            }
        }
        Err(err_val) => eprintln!("Error: Read All Packages | {}", err_val),
    }
}

async fn read_package(package_name: &String) {
    match crate::request::read_package(package_name.to_owned()).await {
        Some(package) => println!("{:#?}", package),
        None => eprintln!("Error: Package Name is Invalid"),
    }
}

async fn install_package(package_name: &String) {
    match crate::request::download_package(package_name.to_owned()).await {
        Some(package_data) => {
            match crate::file::save_package(package_name.to_owned(), &package_data).await {
                Ok(_) => println!("{} is Installed", package_name),
                Err(err_val) => eprintln!("Error: Save Package | {}", err_val),
            }
        }
        None => eprintln!("Error: Download Package"),
    }
}

async fn delete_package(package_name: &String) {
    match crate::file::delete_package(package_name.to_owned()).await {
        Ok(_) => println!("{} is Deleted", package_name),
        Err(err_val) => eprintln!("Error: Delete Package | {}", err_val),
    }
}
