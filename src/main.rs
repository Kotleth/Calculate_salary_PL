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
    println!("Enter ppk contribution [%] (typically it's 2):");
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
        let months_names = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
        println!("\n");
        for i in 0..months_names.len() {
            println!("{} = {:.2}", months_names[i], monthly_statement[i]);
        }
        println!("\n#####");
    }
}

fn calc_netto_specific(brutto_monthly: f32, ppk_value: f32) -> [f32; 12] {
    let mut monthly_statement: [f32; 12] = [0.0; 12];
    let mut full_brutto: f32 = 0.0;
    let mut netto: f32;

    for month_number in 0..12 {
        let pension_contr = brutto_monthly * 0.0976;
        let rental_contr = brutto_monthly * 0.015;
        let medical_contr = brutto_monthly * 0.0245;
        let ppk_company = brutto_monthly * 0.015;
        let brutto = (brutto_monthly - pension_contr - rental_contr - medical_contr) * 0.91 - brutto_monthly * ppk_value/100.0 + ppk_company;
        if brutto <= TAX_FREE_ALLOWANCE - full_brutto {
            netto = brutto;
        } else if full_brutto >= TAX_FREE_ALLOWANCE {
            if brutto <= FIRST_THRESHOLD - full_brutto {
                netto = brutto * 0.88;
                if brutto * 0.12 < 300.0 { // ** not sure
                    netto += brutto * 0.12;
                } else {
                    netto += 300.0;
                } // not sure **
            } else { // brutto + full_brutto > FIRST_THRESHOLD
                if full_brutto >= FIRST_THRESHOLD {
                    netto = brutto * 0.68;
                } else { // full_brutto < FIRST THRESHOLD
                    netto = (FIRST_THRESHOLD - full_brutto) * 0.88 + (brutto - (FIRST_THRESHOLD - full_brutto)) * 0.68;
                }
                netto += 300.0; // not sure
            }
        } else { // full_brutto < TAX_FREE_ALLOWANCE 
            if brutto + full_brutto > FIRST_THRESHOLD {
                netto = TAX_FREE_ALLOWANCE - full_brutto + (FIRST_THRESHOLD - TAX_FREE_ALLOWANCE) * 0.88 + (brutto - FIRST_THRESHOLD - TAX_FREE_ALLOWANCE) * 0.68;
                netto += 300.0; // not sure
            } else { // brutto + full_brutto <= FIRST_THRESHOLD
                netto = TAX_FREE_ALLOWANCE - full_brutto + (brutto - (TAX_FREE_ALLOWANCE - full_brutto)) * 0.88;
                if (brutto - (TAX_FREE_ALLOWANCE - full_brutto)) * 0.12 < 300.0 { // ** not sure
                    netto += (brutto - (TAX_FREE_ALLOWANCE - full_brutto)) * 0.12;
                } else {
                    netto += 300.0;
                } // not sure **
            }
        }
        full_brutto += brutto_monthly;
        monthly_statement[month_number] = netto;
    }
    let mut annually_netto: f32 = 0.0;
    for income in monthly_statement {
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
    let mut buffer = String::new();
    println!("Press enter to exit");
    io::stdin().read_line(&mut buffer).expect("Not a valid string\n");
}