use std::error::Error;
use ticket_translation::TicketTranslator;

fn main() -> Result<(), Box<dyn Error>> {
    let translator = TicketTranslator::new("input")?;
    println!("Ticket scanning error rate: {}", translator.error_rate());

    println!(
        "Departure number product: {}",
        translator.departure_numbers().iter().product::<usize>()
    );

    Ok(())
}
