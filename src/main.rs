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
) -> Vec<(String, i32)>
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
    generate_score_for_factor(results.clone())
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

fn run() -> Result<(), Box<dyn Error>> {
    // let mut records = Vec::new();
    let mut rdr = csv::Reader::from_path("resources/Bank_details.csv")?;
    let records: Vec<IssuerDetail> = rdr.deserialize()
        .map(|result| result.expect("Failed to deserialize record"))
        .collect();

    let cmp_vec_score = collect_and_sort(&records, |r| r.CMP, false);
    let pe_vec_score = collect_and_sort(&records, |r| r.PE, false);
    let market_cap_score = collect_and_sort(&records, |r| r.Mar_Cap, true);
    let div_yield_score = collect_and_sort(&records, |r| r.Div_Yld, true);
    let np_qtr_score = collect_and_sort(&records, |r| r.NP_Qtr, true);
    let sales_qtr_score = collect_and_sort(&records,|r| r.Sales_Qtr, true);
    let roce_score = collect_and_sort(&records, |r| r.ROCE,true);
    let sales_var_5yrs_score = collect_and_sort(&records, |r| r.Sales_Var_5Yrs, true);
    let debt_per_equity_score = collect_and_sort(&records,|r| r.Debt_per_Eq, false);
    let roe_score = collect_and_sort(&records,|r|r.ROE,true);
    let eps_12m_score = collect_and_sort(&records,|r|r.EPS_12M, true);
    let profit_growth_score = collect_and_sort(&records, |r| r.Profit_growth, true);
    let opm_score = collect_and_sort(&records,|r| r.OPM,true);
    let opm_5years_score = collect_and_sort(&records,|r|r.OPM_5yrs,true);
    let piotski_scr_score = collect_and_sort(&records, |r| r.Piotski_Scr as f64, true);

    let mut final_score_map: std::collections::HashMap<String, i32> = std::collections::HashMap::new();

    let all_scores = vec![
        cmp_vec_score,
        pe_vec_score,
        market_cap_score,
        div_yield_score,
        np_qtr_score,
        sales_qtr_score,
        roce_score,
        sales_var_5yrs_score,
        debt_per_equity_score,
        roe_score,
        eps_12m_score,
        profit_growth_score,
        opm_score,
        opm_5years_score,
        piotski_scr_score,
    ];

    for scores in all_scores {
        for (name, score) in scores {
            *final_score_map.entry(name).or_insert(0) += score;
        }
    }
    let mut final_score_vec: Vec<(String, i32)> = final_score_map.into_iter().collect();
    final_score_vec.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    println!("{:?}",final_score_vec);
    Ok(())
}

#[actix_rt::main]
async fn main() {

    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
