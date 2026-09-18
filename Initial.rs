use std::collections::BTreeMap;

struct Employee {
    name: String,
    department: String,
    salary: f64,
}

struct Payroll {
    employees: BTreeMap<String, Employee>,
}

impl Payroll {
    fn new() -> Self {
        Self {
            employees: BTreeMap::new(),
        }
    }

    fn add_employee(&mut self, name: &str, department: &str, salary: f64) {
        self.employees.insert(
            name.to_string(),
            Employee {
                name: name.to_string(),
                department: department.to_string(),
                salary,
            },
        );
    }

    fn total_salary(&self) -> f64 {
        self.employees
            .values()
            .map(|employee| employee.salary)
            .sum()
    }

    fn average_salary(&self) -> f64 {
        if self.employees.is_empty() {
            return 0.0;
        }

        self.total_salary() / self.employees.len() as f64
    }

    fn highest_paid(&self) -> Option<&Employee> {
        self.employees.values().max_by(|a, b| {
            a.salary.partial_cmp(&b.salary).unwrap()
        })
    }

    fn print_report(&self) {
        println!("Payroll Report");
        println!("==============");

        for employee in self.employees.values() {
            println!(
                "{} | {} | ${:.2}",
                employee.name,
                employee.department,
                employee.salary
            );
        }

        println!("==============");
        println!("Employees: {}", self.employees.len());
        println!("Total Salary: ${:.2}", self.total_salary());
        println!("Average Salary: ${:.2}", self.average_salary());

        if let Some(employee) = self.highest_paid() {
            println!("Highest Paid: {}", employee.name);
            println!("Highest Salary: ${:.2}", employee.salary);
        }
    }
}

fn main() {
    let mut payroll = Payroll::new();

    payroll.add_employee("Alice", "Engineering", 6200.00);
    payroll.add_employee("Brian", "Marketing", 4700.50);
    payroll.add_employee("Clara", "Design", 5300.75);
    payroll.add_employee("David", "Finance", 5800.25);

    payroll.print_report();
}