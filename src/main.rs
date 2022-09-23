#[tokio::main]

async fn main() -> Result<(), reqwest::Error> {
    loop {
        let mut word = String::new();
        println!("Anlamını öğrenmek istediğiniz kelimeyi girin:");

        std::io::stdin()
            .read_line(&mut word)
            .expect("Kelime okunamadı");

        let url = String::from(format!("https://sozluk.gov.tr/gts?ara={}", word.trim().to_lowercase()));

        let definition: serde_json::Value =
            reqwest::Client::new()
            .get(url)
            .send()
            .await?
            .json()
            .await?;
        let mut definition_count: String = definition[0]["anlam_say"].to_string(); // can't parse the string because it is ""number"" instead "number"
        definition_count.retain(|c| c != '"'); // use retain to remove extra double quotes
        let definition_count: usize = match definition_count.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Kelime bulunamadı"); //fails if get request was unsuccessful
                continue;
            }
        };
        let mut i = 0; // iterate over all the meanings to print them 
        while i < definition_count {
            let meaning = definition[0]["anlamlarListe"][i]["anlam"].to_string();
            println!("{}. {}", i + 1, meaning);
            println!("");

            i = i + 1
        }
    }
}
