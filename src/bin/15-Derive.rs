#[derive(Debug,Clone, Copy)]
enum Position{
    Manager,
    Supervisor,
    Worker
}
#[derive(Debug,Clone, Copy)]
struct Employee{
    position:Position,
    work_hours:i64,
}
fn printEmployee(emp:Employee){
    println!("{:?}",emp)
}
fn main(){
    let me:Employee = Employee{
        position:Position::Manager,
        work_hours:40
    };

    // Since we have pass Copy Drive
    // We can pass the ownership directly
    printEmployee(me);
    printEmployee(me);
}

