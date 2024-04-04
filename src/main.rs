use chrono::Duration;
use chrono::NaiveDate;
use std::collections::HashMap;

#[derive(Debug)]
struct LotteryResult {
    animal: String,
    hour: String,
}

fn main() {
    let start_date = NaiveDate::from_ymd_opt(2024, 3, 4).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2024, 4, 25).unwrap();

    for current_date in start_date.iter_weeks().take_while(|d| *d <= end_date) {
        let formatted_date = current_date.format("%Y/%m/%d").to_string();
        let url = format!(
            "https://www.tuazar.com/loteria/animalitos/resultados/{}",
            formatted_date
        );
        println!("Getting the results for the date: {}", url);

        let response = reqwest::blocking::get(url);
        // get the HTML content from the request response
        // and print it
        let html_content = response.unwrap().text().unwrap();

        let document = scraper::Html::parse_document(&html_content);

        let lottery_name = get_the_lottery_name(&document);
        println!("The lottery name is: {}", lottery_name);

        // get_the_lottery_week_results(&document);
    }
}

// get the lottery first date

fn get_the_lottery_week_results(
    document: &scraper::Html,
) -> HashMap<NaiveDate, Vec<LotteryResult>> {
    let initial_date = get_the_lottery_first_date(&document).unwrap();

    // hash map to store the results by date
    let mut results_by_date: HashMap<NaiveDate, Vec<LotteryResult>> = HashMap::new();

    let html_table_body_selector = scraper::Selector::parse(
        "#main > div.tema > div:nth-child(5) > div.col-md-8.resultados.table-responsive > table > tbody > tr > td",
    ).unwrap();

    let table_body = document.select(&html_table_body_selector);
    let mut all_rows = Vec::new();

    for body in table_body {
        let rows = body.text().collect::<Vec<_>>();
        all_rows.push(rows.last().unwrap().to_string());
    }

    let lottery_hours = get_the_lottery_hour_results(&document);

    let chunks = all_rows.chunks(7);

    for (chunk, hour) in chunks.zip(lottery_hours.iter().cloned()) {
        let mut date = initial_date;

        for item in chunk {
            let entry = results_by_date.entry(date).or_insert_with(Vec::new);
            entry.push(LotteryResult {
                animal: item.to_string(),
                hour: hour.clone(),
            });

            date = date + Duration::days(1);
        }
    }

    return results_by_date;
}

fn get_the_lottery_first_date(document: &scraper::Html) -> Option<NaiveDate> {
    let html_table_headers_time_selector = scraper::Selector::parse(
        "#main > div.tema > div:nth-child(5) > div.col-md-8.resultados.table-responsive > table > thead > tr > th > time",
    ).unwrap();

    let mut table_headers = document.select(&html_table_headers_time_selector);

    if let Some(header) = table_headers.next() {
        let headers = header.text().collect::<Vec<_>>();
        let date = headers.last().unwrap();
        if let Ok(date) = NaiveDate::parse_from_str(date, "%d/%m/%Y") {
            return Some(date);
        }
    }

    None
}

// get the lottery hour results
fn get_the_lottery_hour_results(document: &scraper::Html) -> Vec<String> {
    let html_table_body_selector = scraper::Selector::parse("#main > div.tema > div:nth-child(5) > div.col-md-8.resultados.table-responsive > table > tbody > tr").unwrap();

    let table_body = document.select(&html_table_body_selector);

    let mut results: Vec<String> = Vec::new();
    for result in table_body {
        results.push(
            result
                .text()
                .collect::<Vec<_>>()
                .first()
                .unwrap()
                .to_string(),
        );
    }

    return results;
}

fn get_the_lottery_name(document: &scraper::Html) -> String {
    let html_lottery_selector = scraper::Selector::parse(
        "#main > div.tema > div:nth-child(5) > div.col-md-8.resultados.table-responsive > h2",
    )
    .unwrap();

    let lottery_results = document.select(&html_lottery_selector);

    for result in lottery_results {
        return result
            .text()
            .collect::<Vec<_>>()
            .last()
            .unwrap()
            .trim()
            .to_string();
    }

    // Return a default value in case no result is found
    String::new()
}
