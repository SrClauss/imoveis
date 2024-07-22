pub mod xlsx {
    use calamine::{open_workbook_auto, Reader};
    use serde_json::{json, Value};
    use std::collections::HashMap;
    use unicode_normalization::UnicodeNormalization;
    
    

   fn convert_string_in_keys(string: &str) -> Option<String> {
        let mut result = String::new();
        for c in string.chars() {
            if c.is_alphanumeric() {
                result.push(c);
            } else {
                if c == ' ' {
                    result.push('_');
                } else {
                    continue;
                }
            }
        }
        //remove espaços em branco no inicio e no final
        result = result.trim().to_string();

        //substitui espaços em branco por _
        result = result.replace(" ", "_");
        //remover assentos e cedilhas

        return Some(
            result
                .nfd()
                .filter(char::is_ascii)
                .collect::<String>()
                .to_lowercase(),
        );
    }

    enum InputType {
        String(String),
        Integer(i32),
        Float(f32),
        Boolean(bool),
    }

    fn determine_data_type(string: String) -> InputType {
        if string.parse::<i32>().is_ok() {
            return InputType::Integer(string.parse::<i32>().unwrap());
        } else if string.parse::<f32>().is_ok() {
            return InputType::Float(string.parse::<f32>().unwrap());
        } else if string.parse::<bool>().is_ok() {
            return InputType::Boolean(string.parse::<bool>().unwrap());
        } else {
            return InputType::String(string);
        }
    }
   

    #[tauri::command]
    pub fn read_excel_to_hash_vector(path: &str) -> String {
        let mut result: Vec<Vec<HashMap<String, Value>>> = Vec::new();

        let mut workbook: calamine::Sheets<std::io::BufReader<std::fs::File>> =
            match open_workbook_auto(path) {
                Ok(workbook) => workbook,
                Err(e) => return "{\"error\": \"Erro ao ler workbook\"}".to_string(),
            };
        let sheets: Vec<String> = workbook.sheet_names().to_vec();

        for sheet in sheets {
            let sheet_data = match workbook.worksheet_range(&sheet) {
                Ok(data) => data,
                Err(_) => {
                    continue;
                }
            };

            if sheet_data.is_empty() {
                continue;
            }

            let headers = match sheet_data.rows().next() {
                Some(row) => row.iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                None => continue,
            };

            let mut sheet_data_json: Vec<HashMap<String, Value>> = Vec::new();

            for row in sheet_data.rows().skip(1) {
                let mut row_data: HashMap<String, Value> = HashMap::new();
                for (i, cell) in row.iter().enumerate() {
                    if i >= headers.len() {
                        continue;
                    } // Ignora células além do número de cabeçalhos
                    let key = &headers[i];
            
                    let value = cell.to_string();
                    let value = value.trim().to_string();
            
                    // Supondo que `determine_data_type` retorna um Result
            
                    match determine_data_type(value) {
                        InputType::String(value) => {
                            row_data.insert(convert_string_in_keys(key).unwrap(), json!(value));
                        }
                        InputType::Integer(value) => {
                            row_data.insert(convert_string_in_keys(key).unwrap(), json!(value.to_string().parse::<i32>().unwrap()));
                        }
                        InputType::Float(value) => {
                            row_data.insert(convert_string_in_keys(key).unwrap(), json!(value.to_string().parse::<f32>().unwrap()));
                        }
                        InputType::Boolean(value) => {
                            row_data.insert(convert_string_in_keys(key).unwrap(), json!(value.to_string().parse::<bool>().unwrap()));
                        }
                    }
                }
                sheet_data_json.push(row_data)
            }
            result.push(sheet_data_json);
        }
        json!(result).to_string()
    }
    #[tauri::command]
    pub fn read_sheet_to_hash_vector(path: &str, sheet_name: &str) -> String {
        let mut result: Vec<Vec<HashMap<String, Value>>> = Vec::new();

        let mut workbook: calamine::Sheets<std::io::BufReader<std::fs::File>> =
            match open_workbook_auto(path) {
                Ok(workbook) => workbook,
                Err(e) => return "{\"error\": \"Erro ao ler workbook\"}".to_string(),
            };
        let sheet = workbook.worksheet_range(sheet_name);
        if sheet.is_err() {
            return "{\"error\": \"Erro ao ler planilha\"}".to_string();
            
        }
        let sheet_data = sheet.unwrap();

        let headers = match sheet_data.rows().next() {
            Some(row) => row.iter().map(|x| x.to_string()).collect::<Vec<_>>(),
            None => return "{\"error\": \"Erro ao ler cabeçalhos\"}".to_string(),
        };

        let mut sheet_data_json: Vec<HashMap<String, Value>> = Vec::new();


        for row in sheet_data.rows().skip(1) {
            let mut row_data: HashMap<String, Value> = HashMap::new();
            for (i, cell) in row.iter().enumerate() {
                if i >= headers.len() {
                    continue;
                } // Ignora células além do número de cabeçalhos
                let key = &headers[i];
        
                let value = cell.to_string();
                let value = value.trim().to_string();
        
                // Supondo que `determine_data_type` retorna um Result
        
                match determine_data_type(value) {
                    InputType::String(value) => {
                        row_data.insert(convert_string_in_keys(key).unwrap(), json!(value));
                    }
                    InputType::Integer(value) => {
                        row_data.insert(convert_string_in_keys(key).unwrap(), json!(value.to_string().parse::<i32>().unwrap()));
                    }
                    InputType::Float(value) => {
                        row_data.insert(convert_string_in_keys(key).unwrap(), json!(value.to_string().parse::<f32>().unwrap()));
                    }
                    InputType::Boolean(value) => {
                        row_data.insert(convert_string_in_keys(key).unwrap(), json!(value.to_string().parse::<bool>().unwrap()));
                    }
                }
            }
            sheet_data_json.push(row_data)
        }
    
        json!(sheet_data_json).to_string()


    }


    #[tauri::command]
    pub fn get_sheets_names(dir: &str) -> Result<String, String>{


        if dir.is_empty() {
            return Err("{\"error\": \"Diretório não pode ser vazio\"}".to_string());
        }

        if !dir.ends_with(".xlsx") && !dir.ends_with(".xls") && !dir.ends_with(".xlsm") {
            return Err("{\"error\": \"Arquivo não é um xlsx ou xls\"}".to_string());
        }


        
        let mut workbook: calamine::Sheets<std::io::BufReader<std::fs::File>> =
            match open_workbook_auto(dir) {
                Ok(workbook) => workbook,
                Err(e) => return Err(format!("{}", e))
            };
        let sheets: Vec<String> = workbook.sheet_names().to_vec();
        Ok(json!(sheets).to_string())
    }
}

        