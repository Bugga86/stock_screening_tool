use serde;
use std::error::Error;
use std::process;

#[derive(Debug, serde::Deserialize, PartialEq)]
#[allow(non_snake_case)]
struct IssuerDetail {
    Name: String,
    CMP: f64,
    PE: f64,
    Mar_Cap: f64,
    Div_Yld: f64,
    NP_Qtr: f64,
    Sales_Qtr: f64,
    ROCE: f64,
    Sales_Var_5Yrs: f64,
    Free_Cash_Flow: f64,
    Debt_per_Eq: f64,
    ROE: f64,
    EPS_12M: f64,
    Profit_growth: f64,
    OPM: f64,
    OPM_5yrs: f64,
    Piotski_Scr: i32,
}

fn collect_and_sort<F>(
    records: &[IssuerDetail],
    key_extractor: F,
    descending: bool,
) -> Vec<(String, f64)>
where
    F: Fn(&IssuerDetail) -> f64
{
    let mut results: Vec<(String, f64)> = records
        .iter()
        .map(|record| (record.Name
                           .clone(), key_extractor(record))
        )
        .collect();

    if descending {
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    } else {
        results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
    }
    results
}

fn generate_score_for_factor(
    factor_values: Vec<(String, f64)>
) -> Vec<(String, i32)>
{
    factor_values
        .iter()
        .enumerate()
        .map(|(index, item)| {
            (item.0.clone(),(10-index) as i32)
        }).collect()
}

#[allow(unused)]
fn run() -> Result<(), Box<dyn Error>> {
    let mut records = Vec::new();
    let mut rdr = csv::Reader::from_path("resources/Bank_details.csv")?;
    for result in rdr.deserialize() {
        let record: IssuerDetail = result?;
        records.push(record);
    }

    let cmp_vec = collect_and_sort(&records, |r| r.CMP, false);
    let pe_vec = collect_and_sort(&records, |r| r.PE, false);
    let market_cap = collect_and_sort(&records, |r| r.Mar_Cap, true);
    let div_yield_vec = collect_and_sort(&records, |r| r.Div_Yld, true);
    let np_qtr_vec = collect_and_sort(&records, |r| r.NP_Qtr, true);
    let sales_qtr_vec = collect_and_sort(&records,|r| r.Sales_Qtr, true);
    let roce = collect_and_sort(&records, |r| r.ROCE,true);
    let sales_var_5yrs_vec = collect_and_sort(&records, |r| r.Sales_Var_5Yrs, true);
    let debt_per_equity = collect_and_sort(&records,|r| r.Debt_per_Eq, false);
    let roe = collect_and_sort(&records,|r|r.ROE,true);
    let eps_12m_vec = collect_and_sort(&records,|r|r.EPS_12M, true);
    let profit_growth_vec = collect_and_sort(&records, |r| r.Profit_growth, true);
    let opm = collect_and_sort(&records,|r| r.OPM,true);
    let opm_5years = collect_and_sort(&records,|r|r.OPM_5yrs,true);
    let piotski_scr_vec = collect_and_sort(&records, |r| r.Piotski_Scr as f64, true);

    // let cmp_vec_score: Vec<(String, i32)> = cmp_vec
    //     .iter()
    //     .enumerate()
    //     .map(|(index, item)| {
    //         (item.0.clone(), (10 - index as i32))
    //     })
    //     .collect();
    // print!("{:?}",cmp_vec_score);

    let cmp_vec_score: Vec<(String, i32)> = generate_score_for_factor(cmp_vec);

    println!("{:?}",cmp_vec_score);
    let pe_vec_score: Vec<(String, i32)> = pe_vec
        .iter()
        .enumerate()
        .map(|(index, item)| {
            (item.0.clone(),(10 - index as i32))
        })
        .collect();

    println!("{:?}",pe_vec_score);

    let final_score_vec: Vec<(String, i32)> = cmp_vec_score
        .iter()
        .filter_map(|(issuer_name, issuer_score)| {
            pe_vec_score.iter().find_map(|(issuer_name_2, score_2)| {
                if issuer_name == issuer_name_2 {
                    Some((issuer_name.clone(), issuer_score + score_2))
                } else {
                    None
                }
            })
        })
        .collect();

    println!("{:?}",final_score_vec);
    // .filter_map(|(index, item)| {
    //     if market_cap.iter().any(|issuer| issuer.0 == item.0) {
    //         Some((item.0.clone(), index))
    //     } else {
    //         None
    //     }
    // })
    // .collect();
    // println!("{:?}", pe_vec);
    // println!("{:?}", market_cap);
    // println!("{:?}", div_yield_vec);
    // println!("{:?}", np_qtr_vec);
    // println!("{:?}", sales_qtr_vec);
    // println!("{:?}", roce);
    // println!("{:?}", sales_var_5yrs_vec);
    // println!("{:?}", debt_per_equity);
    // println!("{:?}", roe);
    // println!("{:?}", eps_12m_vec);
    // println!("{:?}", profit_growth_vec);
    // println!("{:?}", opm);
    // println!("{:?}", opm_5years);
    // println!("{:?}", piotski_scr_vec);


    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
