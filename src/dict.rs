use core::fmt;

pub struct Word {
    pub name: String,
    pub definitions: Vec<String>,
    pub def_count: usize,
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Word {
    pub fn new() -> Word {
        Word {
            name: String::new(),
            definitions: Vec::new(),
            def_count: 0,
        }
    }

    pub fn get_def(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "https://sozluk.gov.tr/gts?ara={}",
            self.name.trim().to_lowercase()
        );
        let res = reqwest::blocking::get(url)?;

        let body = res.json::<serde_json::Value>()?;
        let dif_defs = body.to_string().matches("anlamlarListe").count();
        for j in 0..dif_defs {
            let mut definition_count: String = body[0]["anlam_say"].to_string(); // can't parse the string because it is ""number"" instead "number"
            definition_count.retain(|c| c != '"'); // use retain to remove extra double quotes
            self.def_count = definition_count.parse()?;
            for i in 0..self.def_count {
                let meaning = body[j]["anlamlarListe"][i]["anlam"].to_string();
                if meaning != "null" {
                    self.definitions.push(meaning);
                }
            }
        }

        Ok(())
    }

    pub fn clear(&mut self) {
        self.definitions.clear();
        self.def_count = 0;
    }
}
