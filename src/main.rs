use glob::glob;
use scraper::{Html, Selector};
use version_compare::Version;

use std::{cmp::Ordering, fs};
use std::collections::HashMap;

fn compare_versions (a: &String, b: &String) -> Ordering {
    let parts_a = a.split("/").collect::<Vec<_>>();
    let parts_b = b.split("/").collect::<Vec<_>>();
    Version::from(parts_a[1]).unwrap().partial_cmp(&Version::from(parts_b[1]).unwrap()).unwrap()
}

fn main () -> Result<(), Box<dyn std::error::Error>> {
    let mut files: HashMap<String, String> = HashMap::new();

    for entry in glob("./docs/**/*.html").expect("Failed") {
        let full_path = match entry {
            Ok(path) => path.display().to_string(),
            _ => panic!("Could not get path")
        };

        let parts: Vec<&str> = full_path.split("/").collect();
        let file_path = format!("{}/{}", parts[1], parts[2]);

        if parts[3] == "Entities.html" || parts[3] == "Addons.html" {
            if !files.contains_key(&file_path) {
                files.insert(file_path.to_string(), parts[3].to_string());
            }

            // update to entities.html if it exists
            if files.get(&file_path).unwrap() == "Addons.html" && parts[3] == "Entities.html" {
                files.insert(file_path.to_string(), "Entities.html".to_string());
            }
        }
    }

    let mut versions_list = files.keys().collect::<Vec<_>>();
    versions_list.sort_by(|a, b| compare_versions(a, b));

    let latest_version = versions_list[versions_list.len() - 1].split("/").collect::<Vec<_>>()[1];

    let mut versions_map: HashMap<String, String> = HashMap::new();

    let sel = Selector::parse("p[id^=\"minecraft:\"]").unwrap();

    for version_path in &versions_list {
        let filename = files.get(&version_path.to_string()).unwrap();
        let version = version_path.split("/").collect::<Vec<_>>()[1];
        let file_content = fs::read_to_string(format!("./docs/{}/{}", version_path, filename)).unwrap_or_default();
        let document = Html::parse_document(&file_content);

        // println!("{} {}", filename, version);

        for element in document.select(&sel) {
            let id = element.text().collect::<Vec<_>>().join("");
            // println!("id: {}", id);
            versions_map.insert(id.to_string(), version.to_string());
        }
    }

    let mut components_ordered: Vec<_> = versions_map.keys().collect::<Vec<_>>();
    components_ordered.sort();

    println!("{} | {}", "Component", "Version");
    println!("--- | ---");
    for component in components_ordered {
        let last = versions_map.get(component).unwrap();
        let parts = last.split(".").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
        let link = format!("https://bedrock.dev/docs/{}.{}.0.0/{}/Entities#{}", parts[0], parts[1], last, component);
        // ignore 1.8
        if parts[0] == 1 && parts[1] == 8 { continue; }
        if last != latest_version {
            println!("{} | [{}]({})", component, last, link);   
        }
    };

    Ok(())
}
