// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod xlsx;
use std::collections::{HashMap, HashSet};
use serde_json::Value;
use tauri::http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS;
use xlsx::xlsx::{get_sheets_names, read_excel_to_hash_vector, read_sheet_to_hash_vector};
use mouse_rs::Mouse;
use chrono::NaiveDate;


struct Peso {

    ativo : f32,
    cont_tipo_bairro: f32,
    cont_bairro: f32,
    diminuiu_preco: f32,
    visualizacoes: f32,
    novo: f32,
    atualizdo: f32,
    contatos: f32,
    visitas: f32,
    visitas_concluidas: f32,
    visitas_canceladas: f32,
    visitas_aguardando: f32,
    interessou: f32,
    interessou_gerou_proposta: f32,
    total_conservacao: f32,
    media_conservacao: f32,
    total_localizacao: f32,
    media_localizacao: f32,
    total_avaliacao: f32,
    media_avaliacao: f32,
    avaliacao_subjetiva: f32,
    
}
impl Peso {
    fn new_from_strignify(json_str: String) -> Self {

        let json: serde_json::Value = serde_json::from_str(json_str.as_str()).unwrap();
        let ativo = json.get("ativo").unwrap();
        let cont_tipo_bairro = json.get("cont_tipo_bairro").unwrap();
        let cont_bairro = json.get("cont_bairro").unwrap();
        let diminuiu_preco = json.get("diminuiu_preco").unwrap();
        let visualizacoes = json.get("visualizacoes").unwrap();
        let novo = json.get("novo").unwrap();
        let atualizdo = json.get("atualizdo").unwrap();
        let contatos = json.get("contatos").unwrap();
        let visitas = json.get("visitas").unwrap();
        let visitas_concluidas = json.get("visitas_concluidas").unwrap();
        let visitas_canceladas = json.get("visitas_canceladas").unwrap();
        let visitas_aguardando = json.get("visitas_aguardando").unwrap();
        let interessou = json.get("interessou").unwrap();
        let interessou_gerou_proposta = json.get("interessou_gerou_proposta").unwrap();
        let total_conservacao = json.get("total_conservacao").unwrap();
        let media_conservacao = json.get("media_conservacao").unwrap();
        let total_localizacao = json.get("total_localizacao").unwrap();
        let media_localizacao = json.get("media_localizacao").unwrap();
        let total_avaliacao = json.get("total_avaliacao").unwrap();
        let media_avaliacao = json.get("media_avaliacao").unwrap();
        let avaliacao_subjetiva = json.get("avaliacao_subjetiva").unwrap();
        return Peso{

            ativo: ativo.as_f64().unwrap() as f32,
            cont_tipo_bairro: cont_tipo_bairro.as_f64().unwrap() as f32,
            cont_bairro: cont_bairro.as_f64().unwrap() as f32,
            diminuiu_preco: diminuiu_preco.as_f64().unwrap() as f32,
            visualizacoes: visualizacoes.as_f64().unwrap() as f32,
            novo: novo.as_f64().unwrap() as f32,
            atualizdo: atualizdo.as_f64().unwrap() as f32,
            contatos: contatos.as_f64().unwrap() as f32,
            visitas: visitas.as_f64().unwrap() as f32,
            visitas_concluidas: visitas_concluidas.as_f64().unwrap() as f32,
            visitas_canceladas: visitas_canceladas.as_f64().unwrap() as f32,
            visitas_aguardando: visitas_aguardando.as_f64().unwrap() as f32,
            interessou: interessou.as_f64().unwrap() as f32,
            interessou_gerou_proposta: interessou_gerou_proposta.as_f64().unwrap() as f32,
            total_conservacao: total_conservacao.as_f64().unwrap() as f32,
            media_conservacao: media_conservacao.as_f64().unwrap() as f32,
            total_localizacao: total_localizacao.as_f64().unwrap() as f32,
            media_localizacao: media_localizacao.as_f64().unwrap() as f32,
            total_avaliacao: total_avaliacao.as_f64().unwrap() as f32,
            media_avaliacao: media_avaliacao.as_f64().unwrap() as f32,
            avaliacao_subjetiva: avaliacao_subjetiva.as_f64().unwrap() as f32,
        };  
        
       

    }

}


struct Configuracao{
    maximo_imovel_novo: i32,
    maximo_imovel_atualizado: i32,
}
 impl Configuracao {
     fn new(minimo_imovel_novo: i32, minimo_imovel_atualizado: i32) -> Self {
         return Configuracao {
             maximo_imovel_novo: minimo_imovel_novo,
             maximo_imovel_atualizado: minimo_imovel_atualizado,
         };
     }
 }





fn get_criterio_bairo(data: &Vec<HashMap<String, Value>>) -> HashMap<String, f32> {
    
    
    //crie um hashmap para armazenar o critério de bairro
    let mut criterio_bairro = HashMap::new();
    //crie um set para armazenar os bairros
    let mut bairros = std::collections::HashSet::new();
    //iterar sobre os dados
    for item in data {
        let bairro = item.get("bairro");
        if bairro.is_none() {
            continue;
        }
        let bairro = bairro.unwrap().as_str();
        if bairro.is_none() {
            continue;
        }
        let bairro = bairro.unwrap();
        bairros.insert(bairro);
    }
    //insira cada elemento do set no hashmap com o valor 0
    for bairro in bairros {
        criterio_bairro.insert(bairro.to_string(), 0.0);
    }
    //iterar sobre os dados
    for item in data {
        //para cada item, incremente o valor do bairro correspondente
        let bairro = item.get("bairro");
        if bairro.is_none() {
            continue;
        }
        let bairro = bairro.unwrap().as_str();
        if bairro.is_none() {
            continue;
        }
        let bairro = bairro.unwrap();

        let contador = criterio_bairro.get_mut(bairro).unwrap();
        *contador += 1.0;     
    }
    

    return criterio_bairro;

    
}
fn get_criterio_bairro_tipo_imovel (data: &Vec<HashMap<String, Value>>) -> HashMap<String, f32> {
    let mut criterio_bairro_tipo_imovel = HashMap::new();
    let mut bairros_tipo: HashSet<String> = std::collections::HashSet::new();
    

    for item in data{
        let bairro = item.get("bairro");
        if bairro.is_none() {
            continue;
        }
        let bairro = bairro.unwrap().as_str();
        if bairro.is_none() {
            continue;
        }
        let bairro = bairro.unwrap();
        let tipo = item.get("tipo");
        if tipo.is_none() {
            continue;
        }
        let tipo = tipo.unwrap().as_str();
        if tipo.is_none() {
            continue;
        }
        let tipo = tipo.unwrap();
        let bairro_tipo = format!("{}_{}", tipo, bairro);
        bairros_tipo.insert(bairro_tipo);
        
    }
    //insira cada elemento do set no hashmap com o valor 0
    for bairro_tipo in bairros_tipo {
        criterio_bairro_tipo_imovel.insert(bairro_tipo.to_string(), 0.0);
    }
    //iterar sobre os dados
    for item in data {
        //para cada item, incremente o valor do bairro correspondente
        let bairro = item.get("bairro");
        if bairro.is_none() {
            continue;
        }
        let bairro = bairro.unwrap().as_str();
        if bairro.is_none() {
            continue;
        }
        let bairro = bairro.unwrap();
        let tipo = item.get("tipo");
        if tipo.is_none() {
            continue;
        }
        let tipo = tipo.unwrap().as_str();
        if tipo.is_none() {
            continue;
        }
        let tipo = tipo.unwrap();
        let bairro_tipo = format!("{}_{}", tipo, bairro);
        let contador = criterio_bairro_tipo_imovel.get_mut(&bairro_tipo).unwrap();
        *contador += 1.0;     
    }    
    return criterio_bairro_tipo_imovel;
}
#[tauri::command]
fn get_mouse_position(wx: i32, wy:i32) -> String {


    let mouse = Mouse::new();
    let position = mouse.get_position().unwrap();
    let x = position.x - wx;
    let y = position.y - wy;
    return format!("{{\"x\": {}, \"y\": {}}}", x, y);


}
#[tauri::command]
fn classify(data: String, old_data:String,  peso:String, max_ativo:i32, max_novo:i32) -> Result<String, String> {
    //parsear os dados de entrada
    let data_json:Result<Vec<HashMap<String, Value>>, serde_json::Error> = serde_json::from_str(data.as_str());
    if data_json.is_err() {
        return Err("Erro ao parsear os dados".to_string());
    }
    let mut data_json = data_json.unwrap();
    let mut data_json_clone = data_json.clone();
    //parsear os dados antigos
    let old_data_json: Vec<HashMap<String, Value>> = serde_json::from_str(old_data.as_str()).unwrap();
    //criar uma instância de Configuração com os valores min_novo e min_ativo
    let configuracao = Configuracao::new(max_novo, max_ativo);
    //criar uma instância de Peso com os valores do json de peso
    let pesos = Peso::new_from_strignify(peso);
    //criar um hashmap para armazenar o critério de bairro
    let criterios_bairros = get_criterio_bairo(&data_json);
    //criar um hashmap para armazenar o critério de bairro e tipo de imóvel
    let criterios_bairros_tipo_imoveis = get_criterio_bairro_tipo_imovel(&data_json);
    for item in &mut data_json {
        let bairro = item.get("bairro");
        if bairro.is_none() {
            continue;
        }
        let bairro = bairro.unwrap().as_str().unwrap();


        let tipo = item.get("tipo");
        if tipo.is_none() {
            continue;
        }
        let tipo = tipo.unwrap().as_str().unwrap();

        let data_cadastro = item.get("criacao");
        if data_cadastro.is_none() {
            continue;
        }
        let data_cadastro = data_cadastro.unwrap().as_str().unwrap();
        let data_cadastro = NaiveDate::parse_from_str(data_cadastro, "%d-%m-%Y");
        if data_cadastro.is_err() {
            continue;
        }
        let data_cadastro = data_cadastro.unwrap();


        let data_atualizacao = item.get("ultimaAtualizacao");
        if data_atualizacao.is_none() {
            continue;
        }
        let data_atualizacao = data_atualizacao.unwrap().as_str().unwrap();
        let data_atualizacao = NaiveDate::parse_from_str(data_atualizacao, "%d-%m-%Y");
        if data_atualizacao.is_err() {
            continue;
        }
        let data_atualizacao = data_atualizacao.unwrap();

        let criterio_bairro = criterios_bairros.get(bairro);
        if criterio_bairro.is_none() {
            continue;
        }
        let criterio_bairro = criterio_bairro.unwrap();


        let tipo_bairro = format!("{}_{}", tipo, bairro);
        let criterio_bairro_tipo_imovel = criterios_bairros_tipo_imoveis.get(&tipo_bairro);
        if criterio_bairro_tipo_imovel.is_none() {
            continue;
        }
        let criterio_bairro_tipo_imovel = criterio_bairro_tipo_imovel.unwrap();
        let today_naive = chrono::Local::now().date_naive();
        let novo = data_cadastro > today_naive - chrono::Duration::days(configuracao.maximo_imovel_novo as i64);
        let atualizado = data_atualizacao > today_naive - chrono::Duration::days(configuracao.maximo_imovel_atualizado as i64);
        item.insert("criterio_bairro".to_string(), Value::Number(serde_json::Number::from(*criterio_bairro as i64)));
        item.insert("criterio_bairro_tipo_imovel".to_string(), Value::Number(serde_json::Number::from(*criterio_bairro_tipo_imovel as i64)));
        item.insert("novo".to_string(), Value::Bool(novo));
        item.insert("atualizado".to_string(), Value::Bool(atualizado));
              
    }

    for mut item in data_json {
        let mut resultado: HashMap<String, f32> = HashMap::new();

        /***************************************************/
        /******************Status***************************/
        let ativo = item.get("status");
        if ativo.is_none() {
            resultado.insert("ativo".to_string(), 0.0);
        }
        let ativo = ativo.unwrap().as_str();
        if ativo.is_none() {
            resultado.insert("ativo".to_string(), 0.0);
        }
        let ativo = ativo.unwrap();
        if ativo.to_lowercase() == "ativo" {
            resultado.insert("ativo".to_string(), pesos.ativo);
        } else {
            resultado.insert("ativo".to_string(), 0.0);
        }
        /***************************************************/
        /******************Tipo e Bairro********************/
        let cont_tipo_bairro = item.get("criterio_bairro_tipo_imovel");
        if cont_tipo_bairro.is_none() {
            resultado.insert("cont_tipo_bairro".to_string(), 0.0);
        }
        let cont_tipo_bairro = cont_tipo_bairro.unwrap().as_f64();
        if cont_tipo_bairro.is_none() {
            resultado.insert("cont_tipo_bairro".to_string(), 0.0);
        }
        let cont_tipo_bairro = cont_tipo_bairro.unwrap();
        resultado.insert("cont_tipo_bairro".to_string(), pesos.cont_tipo_bairro * cont_tipo_bairro as f32);

        /***************************************************/
        /******************Bairro***************************/
        let cont_bairro = item.get("criterio_bairro");
        if cont_bairro.is_none() {
            resultado.insert("cont_bairro".to_string(), 0.0);
        }
        let cont_bairro = cont_bairro.unwrap().as_f64();
        if cont_bairro.is_none() {
            resultado.insert("cont_bairro".to_string(), 0.0);
        }
        let cont_bairro = cont_bairro.unwrap();
        resultado.insert("cont_bairro".to_string(), pesos.cont_bairro * cont_bairro as f32);

        /***************************************************/
        /******************Old Items********************/
        //procurar em old_data_json o item com o mesmo referencia
        let referencia = item.get("referencia");
        if referencia.is_none() {
            resultado.insert("diminuiu_preco".to_string(), 0.0);
        }

        let referencia = referencia.unwrap().as_str();
        if referencia.is_none() {
            resultado.insert("diminuiu_preco".to_string(), 0.0);
        }
        let referencia = referencia.unwrap();
        let old_item = old_data_json.iter().find(|x| 
            match x.get("referencia").and_then(|v| v.as_str()) {
                Some(str) => str == referencia,
                None => false,
            }
        );
        if old_item.is_none() {
            resultado.insert("diminuiu_preco".to_string(), 0.0);
        }
    
        let old_item = old_item.unwrap();
        let old_preco_locacao = old_item.get("locacao");
        if old_preco_locacao.is_none() {
            resultado.insert("diminuiu_preco".to_string(), 0.0);
        }
        let old_preco_locacao = old_preco_locacao.unwrap().as_f64();
        if old_preco_locacao.is_none() {
            resultado.insert("diminuiu_preco".to_string(), 0.0);
        }
        let old_preco_locacao = old_preco_locacao.unwrap() as f32;

        let old_preco_venda = old_item.get("venda");
        if old_preco_venda.is_none() {
            resultado.insert("diminuiu_preco".to_string(), 0.0);
        }
        let old_preco_venda = old_preco_venda.unwrap().as_f64();
        if old_preco_venda.is_none() {
            resultado.insert("diminuiu_preco".to_string(), 0.0);
        }
        let old_preco_venda = old_preco_venda.unwrap() as f32;
        /************************************************************************************/
        /******************************Comparações*******************************************/



        let preco_locacao = item.get("locacao");
        if preco_locacao.is_none() {
            resultado.insert("diminuiu_preco".to_string(), 0.0);
        }
        let preco_locacao = preco_locacao.unwrap().as_f64();
        if preco_locacao.is_none() {
            resultado.insert("diminuiu_preco".to_string(), 0.0);
        }
        let preco_locacao = preco_locacao.unwrap() as f32;

        let preco_venda = item.get("venda");
        if preco_venda.is_none() {
            resultado.insert("diminuiu_preco".to_string(), 0.0);
        }
        let preco_venda = preco_venda.unwrap().as_f64();
        if preco_venda.is_none() {
            resultado.insert("diminuiu_preco".to_string(), 0.0);
        }
        let preco_venda = preco_venda.unwrap() as f32;

        if preco_locacao < old_preco_locacao || preco_venda < old_preco_venda {
            resultado.insert("diminuiu_preco".to_string(), pesos.diminuiu_preco);
        } else {
            resultado.insert("diminuiu_preco".to_string(), 0.0);
        }

        /***************************************************/
        /******************Visualizações********************/
        let visualizacoes = item.get("visualizacoes");
        if visualizacoes.is_none() {
            resultado.insert("visualizacoes".to_string(), 0.0);
        }
        let visualizacoes = visualizacoes.unwrap().as_f64();
        if visualizacoes.is_none() {
            resultado.insert("visualizacoes".to_string(), 0.0);
        }

        let visualizacoes = visualizacoes.unwrap();
        resultado.insert("visualizacoes".to_string(), pesos.visualizacoes * visualizacoes as f32);

        /***************************************************/
        /******************Novo*****************************/
        let novo = item.get("novo");
        if novo.is_none() {
            resultado.insert("novo".to_string(), 0.0);
        }
        let novo = novo.unwrap().as_bool();
        if novo.is_none() {
            resultado.insert("novo".to_string(), 0.0);
        }
        let novo = novo.unwrap();
        if novo {
            resultado.insert("novo".to_string(), pesos.novo);
        } else {
            resultado.insert("novo".to_string(), 0.0);
        }

        /***************************************************/
        /******************Atualizado***********************/
        let atualizado = item.get("atualizado");
        if atualizado.is_none() {
            resultado.insert("atualizado".to_string(), 0.0);
        }
        let atualizado = atualizado.unwrap().as_bool();
        if atualizado.is_none() {
            resultado.insert("atualizado".to_string(), 0.0);
        }
        let atualizado = atualizado.unwrap();

        if atualizado {
            resultado.insert("atualizado".to_string(), pesos.atualizdo);
        } else {
            resultado.insert("atualizado".to_string(), 0.0);
        }

        /***************************************************/
        /******************Contatos*************************/
        let contatos = item.get("contatos");
        if contatos.is_none() {
            resultado.insert("contatos".to_string(), 0.0);
        }
        let contatos = contatos.unwrap().as_f64();
        if contatos.is_none() {
            resultado.insert("contatos".to_string(), 0.0);
        }
        let contatos = contatos.unwrap();
        resultado.insert("contatos".to_string(), pesos.contatos * contatos as f32);


        /***************************************************/
        /******************Visitas*************************/

        let visitas = item.get("numVisitas");
        if visitas.is_none() {
            resultado.insert("visitas".to_string(), 0.0);
        }
        let visitas = visitas.unwrap().as_f64();
        if visitas.is_none() {
            resultado.insert("visitas".to_string(), 0.0);
        }
        let visitas = visitas.unwrap();
        resultado.insert("visitas".to_string(), pesos.visitas * visitas as f32);

        /***************************************************/
        /******************Visitas Concluídas****************/
        

        let visitas_concluidas = item.get("visitasConcluidas");
        if visitas_concluidas.is_none() {
            resultado.insert("visitas_concluidas".to_string(), 0.0);
        }
        let visitas_concluidas = visitas_concluidas.unwrap().as_f64();
        if visitas_concluidas.is_none() {
            resultado.insert("visitas_concluidas".to_string(), 0.0);
        }
        let visitas_concluidas = visitas_concluidas.unwrap();


        resultado.insert("visitas_concluidas".to_string(), pesos.visitas_concluidas * visitas_concluidas as f32);

        /***************************************************/
        /******************Visitas Canceladas****************/
        let visitas_canceladas = item.get("visitasCanceladas");
        if visitas_canceladas.is_none() {
            resultado.insert("visitas_canceladas".to_string(), 0.0);
        }
        let visitas_canceladas = visitas_canceladas.unwrap().as_f64();
        if visitas_canceladas.is_none() {
            resultado.insert("visitas_canceladas".to_string(), 0.0);
        }
        let visitas_canceladas = visitas_canceladas.unwrap();
        resultado.insert("visitas_canceladas".to_string(), pesos.visitas_canceladas * visitas_canceladas as f32);

        /***************************************************/
        /******************Visitas Aguardando****************/
        let visitas_aguardando = item.get("visitasAguardando");
        if visitas_aguardando.is_none() {
            resultado.insert("visitas_aguardando".to_string(), 0.0);
        }
        let visitas_aguardando = visitas_aguardando.unwrap().as_f64();
        if visitas_aguardando.is_none() {
            resultado.insert("visitas_aguardando".to_string(), 0.0);
        }
        let visitas_aguardando = visitas_aguardando.unwrap();
        resultado.insert("visitas_aguardando".to_string(), pesos.visitas_aguardando * visitas_aguardando as f32);


        /***************************************************/
        /******************Interessou***********************/
        let interessou = item.get("interessou");
        if interessou.is_none() {
            resultado.insert("interessou".to_string(), 0.0);
        }
        let interessou = interessou.unwrap().as_f64();
        if interessou.is_none() {
            resultado.insert("interessou".to_string(), 0.0);
        }
        let interessou = interessou.unwrap();
        resultado.insert("interessou".to_string(), pesos.interessou * interessou as f32);

        /***************************************************/
        /******************Interessou Gerou Proposta********/

        let interessou_gerou_proposta = item.get("interessouGerouProposta");
        if interessou_gerou_proposta.is_none() {
            resultado.insert("interessou_gerou_proposta".to_string(), 0.0);
        }
        let interessou_gerou_proposta = interessou_gerou_proposta.unwrap().as_f64();
        if interessou_gerou_proposta.is_none() {
            resultado.insert("interessou_gerou_proposta".to_string(), 0.0);
        }
        let interessou_gerou_proposta = interessou_gerou_proposta.unwrap();
        resultado.insert("interessou_gerou_proposta".to_string(), pesos.interessou_gerou_proposta * interessou_gerou_proposta as f32);


        /***************************************************/
        /******************Total Conservação****************/
        let total_conservacao = item.get("totalConservacao");
        if total_conservacao.is_none() {
            resultado.insert("total_conservacao".to_string(), 0.0);
        }
        let total_conservacao = total_conservacao.unwrap().as_f64();
        if total_conservacao.is_none() {
            resultado.insert("total_conservacao".to_string(), 0.0);
        }
        let total_conservacao = total_conservacao.unwrap();
        resultado.insert("total_conservacao".to_string(), pesos.total_conservacao * total_conservacao as f32);

        /***************************************************/
        /******************Media Conservação****************/
        let media_conservacao = item.get("mediaConservacao");
        if media_conservacao.is_none() {
            resultado.insert("media_conservacao".to_string(), 0.0);
        }
        let media_conservacao = media_conservacao.unwrap().as_f64();
        if media_conservacao.is_none() {
            resultado.insert("media_conservacao".to_string(), 0.0);
        }
        let media_conservacao = media_conservacao.unwrap();
        resultado.insert("media_conservacao".to_string(), pesos.media_conservacao * media_conservacao as f32);

        /***************************************************/
        /******************Total Localização****************/
        let total_localizacao = item.get("totalLocalizacao");
        if total_localizacao.is_none() {
            resultado.insert("total_localizacao".to_string(), 0.0);
        }
        let total_localizacao = total_localizacao.unwrap().as_f64();
        if total_localizacao.is_none() {
            resultado.insert("total_localizacao".to_string(), 0.0);
        }
        let total_localizacao = total_localizacao.unwrap();
        resultado.insert("total_localizacao".to_string(), pesos.total_localizacao * total_localizacao as f32);

        /***************************************************/
        /******************Media Localização****************/
        let media_localizacao = item.get("mediaLocalizacao");
        if media_localizacao.is_none() {
            resultado.insert("media_localizacao".to_string(), 0.0);
        }
        let media_localizacao = media_localizacao.unwrap().as_f64();
        if media_localizacao.is_none() {
            resultado.insert("media_localizacao".to_string(), 0.0);
        }
        let media_localizacao = media_localizacao.unwrap();
        resultado.insert("media_localizacao".to_string(), pesos.media_localizacao * media_localizacao as f32);

        /***************************************************/
        /******************Total Avaliação******************/

        let total_avaliacao = item.get("totalAvaliacao");
        if total_avaliacao.is_none() {
            resultado.insert("total_avaliacao".to_string(), 0.0);
        }
        let total_avaliacao = total_avaliacao.unwrap().as_f64();
        if total_avaliacao.is_none() {
            resultado.insert("total_avaliacao".to_string(), 0.0);
        }
        let total_avaliacao = total_avaliacao.unwrap();
        resultado.insert("total_avaliacao".to_string(), pesos.total_avaliacao * total_avaliacao as f32);

        /***************************************************/
        /******************Media Avaliação******************/
        let media_avaliacao = item.get("mediaAvaliacao");
        if media_avaliacao.is_none() {
            resultado.insert("media_avaliacao".to_string(), 0.0);
        }
                let media_avaliacao = media_avaliacao.unwrap().as_f64();
        if media_avaliacao.is_none() {
            resultado.insert("media_avaliacao".to_string(), 0.0);
        }
        let media_avaliacao = media_avaliacao.unwrap();
        resultado.insert("media_avaliacao".to_string(), pesos.media_avaliacao * media_avaliacao as f32);

        /***************************************************/
        /******************Avaliação Subjetiva**************/
        let avaliacao_subjetiva = item.get("avaliacaoSubjetiva");
        if avaliacao_subjetiva.is_none() {
            resultado.insert("avaliacao_subjetiva".to_string(), 0.0);
        }
        let avaliacao_subjetiva = avaliacao_subjetiva.unwrap().as_f64();
        if avaliacao_subjetiva.is_none() {
            resultado.insert("avaliacao_subjetiva".to_string(), 0.0);
        }
        let avaliacao_subjetiva = avaliacao_subjetiva.unwrap();
        resultado.insert("avaliacao_subjetiva".to_string(), pesos.avaliacao_subjetiva * avaliacao_subjetiva as f32);



        resultado.insert("total".to_string(), resultado.values().sum());

        //insira este total no item iterado

        let resultado_64 = resultado.get("total").unwrap().clone() as f64;
        item.insert("resultado".to_string(), Value::Number(serde_json::Number::from_f64(resultado_64).unwrap()));
    

    }
    //ordene os dados por resultado
    data_json_clone.sort_by(|a, b| {
        let a = a.get("resultado").unwrap().as_f64().unwrap();
        let b = b.get("resultado").unwrap().as_f64().unwrap();
        a.partial_cmp(&b).unwrap()
    });

    //retorne os dados ordenados
    return Ok(serde_json::to_string(&data_json_clone).unwrap());
    
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            read_excel_to_hash_vector,
            read_sheet_to_hash_vector,
            get_sheets_names,
            get_mouse_position, 
            classify
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
