use std::collections::HashMap;

fn list(company: &HashMap<String, Vec<String>>, dep: &str) {
    if company.contains_key(dep) {
        let list: Vec<String> = company.get(dep).unwrap().clone();
        println!(
            "The list of employees in the Department of '{}' is as follows:",
            dep
        );
        for emp in list {
            println!("{}", emp);
        }
    } else {
        println!("The Department '{}' does not exist!", dep);
    }
}

fn list_all(company: &HashMap<String, Vec<String>>) {
    println!("Listing all Departments and Employees...");
    let mut depts: Vec<&String> = company.keys().collect();
    depts.sort();
    if company.is_empty() {
        println!("No employees found.");
    }
    for dept in depts {
        println!("{}", dept);
        for emp in &company[dept] {
            println!("\t{}", emp);
        }
    }
}
fn add(map: &mut HashMap<String, Vec<String>>, dep: &str, employee: &str) {
    let employees: &mut Vec<String> = map.entry(dep.to_string()).or_insert(Vec::new());
    employees.push(employee.to_string());
    employees.sort();
    println!("Added employee '{}' to '{}'", employee, dep);
}

fn main() {
    // Add Sally to Engineering
    // Add Amir to Sales
    // Add John to Engineering
    // List Engineering
    // List all
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    let mut cmd = "Add Sally to Engineering".trim().to_lowercase();

    loop {
        let mut cmd = String::new();
        std::io::stdin().read_line(&mut cmd).expect("read error");
        let cmd = cmd.trim().to_lowercase();
        if cmd.starts_with("add") {
            // Analyze departments and employees
            let parts = cmd["Add ".len()..].split(" to ").collect::<Vec<_>>();
            // println!("{:?}", parts);
            add(&mut company, parts[1], parts[0]);
        } else if cmd.starts_with("list") {
            if cmd.contains("all") {
                list_all(&company);
            } else {
                let parts = cmd["list ".len()..].trim();
                list(&company, parts);
            }
        } else if ["quit", "q", "exit"].contains(&&cmd[..]) {
            break;
        }
    }
}
