use anyhow::Result;
use csv_processor::process_quarter;
use csv_processor::process_yearly;

fn main() -> Result<()> {
    // match process_quarter("result.csv") {
    //     Ok(()) => println!("quater csv processed successfully"),
    //     Err(e) => panic!("Error processing quarterly file: {}", e),
    // }
    //
    // match process_yearly("yearly.csv") {
    //     Ok(()) => println!("yearly csv processed successfully"),
    //     Err(e) => panic!("Error processing yearly file: {}", e),
    // }
    Ok(())
}
