use std::{env, process::exit};

use crate::file::calculate_hash;

fn env_collector() -> Vec<String> {
    let mut env_values = env::args().collect::<Vec<String>>();
    env_values.remove(0);
    env_values
}

pub async fn user_interaction() {
    println!("\n\n----------------------------------");
    let env_values = env_collector();

    match env_values.get(0).unwrap_or_else(|| exit(0)).as_str() {
        "read_all_packages" => {
            read_all_packages().await;
            return;
        }
        "read_package" => {
            let package_name = match env_values.get(1) {
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
            let package_name = match env_values.get(1) {
                Some(package_name) => package_name,
                None => {
                    eprintln!("Length is not enough");
                    return;
                }
            };
            install_package_with_dependencies(package_name).await;
            return;
        }
        "delete_package" => {
            let package_name = match env_values.get(1) {
                Some(package_name) => package_name,
                None => {
                    eprintln!("Length is not enough");
                    return;
                }
            };
            delete_package(package_name).await;
            return;
        }
        "list_installed_packages" => {
            list_installed_packages().await;
            return;
        }
        "update_package" => {
            let package_name = match env_values.get(1) {
                Some(package_name) => package_name,
                None => {
                    eprintln!("Length is not enough");
                    return;
                }
            };
            update_package(package_name).await;
            return;
        }
        "update_all_packages" => {
            update_all_packages().await;
            return;
        }
        "help" => {
            help().await;
            return;
        }
        "--help" => {
            help().await;
            return;
        }
        _ => {
            eprintln!("Need an Argument");
            return;
        }
    }
}

async fn help() {
    println!("\n\n\n");
    println!("   Arguments                        |  Details");
    println!("---------------------------------------------------------------------------------------------");
    println!("   read_all_packages                |  Shows Details of All Packages from Server");
    println!("   read_package package_name        |  Shows  Details of a Package in from Server  ");
    println!(
        "   install_package package_name          |  Installs a Package from Server with Dependencies "
    );
    println!("   delete_package package_name      |  Deletes an Installed Package from System");
    println!("   list_installed_packages          |  Shows All Installed Packages");
    println!("   update_package package_name      |  Updates an Installed Package");
    println!("   update_all_packages              |  Updates All Installed Packages ");
    println!("   help                             |  Shows Help");
    println!("   --help                           |  Shows Help");
    println!("\n\n\n");
}

async fn read_all_packages() {
    let packages = crate::request::read_all_packages().await;
    match packages {
        Ok(packages) => {
            for package in packages {
                println!("{}", package.get_name(),);
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

async fn install_package_with_dependencies(package_name: &String) {
    let dependencies = match crate::request::read_package(package_name.to_owned()).await {
        Some(package) => package.get_dependencies(),
        None => {
            eprintln!(
                "Error: There is No Package with This Name | {}",
                package_name
            );
            return;
        }
    };
    for dependency in dependencies {
        install_package(&dependency).await;
    }
    install_package(package_name).await;
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

async fn list_installed_packages() {
    match crate::file::list_installed_packages().await {
        Ok(package_names) => {
            if package_names.is_empty() {
                println!("There is no installed package");
                return;
            }
            for package_name in package_names {
                println!("{}", package_name);
            }
        }
        Err(err_val) => eprintln!("Error: List Installed Packages | {}", err_val),
    }
}

async fn update_package(package_name: &String) {
    let target_package_local_hash = match calculate_hash(package_name.to_owned()).await {
        Ok(target_package_local_hash) => match target_package_local_hash {
            Some(target_package_local_hash) => target_package_local_hash,
            None => {
                eprintln!(
                    "Error: No Metadata Found for Local Hash Calculation | {}",
                    package_name
                );
                return;
            }
        },
        Err(err_val) => {
            eprintln!(
                "Error: Local Hash Calculation | {} | {}",
                package_name, err_val
            );
            return;
        }
    };
    match crate::request::read_package(package_name.to_owned()).await {
        Some(package) => match package.get_hash().eq(&target_package_local_hash) {
            true => println!("Package is Already Up to Date"),
            false => {
                println!("New Version is Found, Installing");
                install_package(package_name).await;
            }
        },
        None => eprintln!("Error: Update Package | {}", package_name),
    }
}

async fn update_all_packages() {
    match crate::file::list_installed_packages().await {
        Ok(package_names) => {
            if package_names.is_empty() {
                println!("There is no installed package");
                return;
            }
            for package_name in package_names {
                update_package(&package_name).await;
            }
        }
        Err(err_val) => eprintln!("Error: List Installed Packages | {}", err_val),
    }
}
