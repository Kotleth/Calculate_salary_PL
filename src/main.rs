use std::io;

const FIRST_THRESHOLD: f32 = 120_000.0;
const TAX_FREE_ALLOWANCE: f32 = 30_000.0;

/// FUNCTIONS ///

fn read_salary() {
    let mut brutto_str = String::new();
    let mut ppk_str = String::new();
    println!("Enter your monthly income [PLN]: ");
    io::stdin().read_line(&mut brutto_str).expect("Not a valid string\n");
    let brutto: f32 = match brutto_str.trim().parse() {
        Ok(value) => value,
        Err(_) => - 1.0,
    };
    println!("Enter ppk contribution [%]: ");
    io::stdin().read_line(&mut ppk_str).expect("Not a valid string\n");
    let ppk_value: f32 = match ppk_str.trim().parse() {
        Ok(value) => value,
        Err(_) => 0.0,
    };
    if brutto < 0.0 {
        println!("Your salary must be a positive, numerical value.\n");
        read_salary();
    } else {
        let monthly_statement = calc_netto_specific(brutto, ppk_value);
        println!("\nJan = {:.2}", monthly_statement[0]);
        println!("Feb = {:.2}", monthly_statement[1]);
        println!("Mar = {:.2}", monthly_statement[2]);
        println!("Apr = {:.2}", monthly_statement[3]);
        println!("May = {:.2}", monthly_statement[4]);
        println!("Jun = {:.2}", monthly_statement[5]);
        println!("Jul = {:.2}", monthly_statement[6]);
        println!("Aug = {:.2}", monthly_statement[7]);
        println!("Sep = {:.2}", monthly_statement[8]);
        println!("Oct = {:.2}", monthly_statement[9]);
        println!("Nov = {:.2}", monthly_statement[10]);
        println!("Dec = {:.2}\n", monthly_statement[11]);
        println!("#####");
    }
}

fn calc_netto_specific(brutto_monthly: f32, ppk_value: f32) -> [f32; 12] {
    let mut monthly_statement: [f32; 12] = [0.0; 12];
    let mut full_brutto: f32 = 0.0;
    let mut netto = 0.0;
    let mut enumerator = 0;

    for income in 0..12 {
        let pension_contr = brutto_monthly * 0.0976;
        let rental_contr = brutto_monthly * 0.015;
        let medical_contr = brutto_monthly * 0.0245;
        let ppk_company = brutto_monthly * 0.015;
        let tax_reduction = 300.0;
        let brutto = (brutto_monthly - pension_contr - rental_contr - medical_contr) * 0.91 - brutto_monthly * ppk_value/100.0 + ppk_company;
        if brutto <= TAX_FREE_ALLOWANCE - full_brutto {
            netto = brutto;
        } else if full_brutto >= TAX_FREE_ALLOWANCE {
            if brutto <= FIRST_THRESHOLD - full_brutto {
                netto = brutto * 0.88;
            } else { // brutto + full_brutto > FIRST_THRESHOLD
                if full_brutto >= FIRST_THRESHOLD {
                    netto = brutto * 0.68;
                } else { // full_brutto < FIRST THRESHOLD
                    netto = (FIRST_THRESHOLD - full_brutto) * 0.88 + (brutto - (FIRST_THRESHOLD - full_brutto)) * 0.68;
                }
            }
        } else { // full_brutto < TAX_FREE_ALLOWANCE 
            if brutto + full_brutto > FIRST_THRESHOLD {
                netto = TAX_FREE_ALLOWANCE - full_brutto + (FIRST_THRESHOLD - TAX_FREE_ALLOWANCE) * 0.88 + (brutto - FIRST_THRESHOLD - TAX_FREE_ALLOWANCE) * 0.68;
            } else { // brutto + full_brutto <= FIRST_THRESHOLD
                netto = TAX_FREE_ALLOWANCE - full_brutto + (brutto - (TAX_FREE_ALLOWANCE - full_brutto)) * 0.88;
            }
        }
        full_brutto += brutto_monthly;
        monthly_statement[enumerator] = netto;
        enumerator += 1;
    }
    let mut annually_netto: f32 = 0.0;
    for income in monthly_statement {
        // println!("{}. month = {}", enumerator, income);
        annually_netto += income;
    };
    println!("\n#####\nAnnually brutto = {:.2}", full_brutto);
    println!("Annually netto = {:.2}", annually_netto);
    println!("Taxes = {:.2}%", (1.0 - annually_netto/full_brutto) * 100.0);
    println!("Average netto = {:.2}", annually_netto/12.0);
    monthly_statement
}

fn main() {    
    read_salary();
}