use colorize::AnsiColor;
use inquire::{validator::Validation, CustomUserError};
use chrono::NaiveDate;

#[derive(Debug)]
struct ConsultaCrypto{
    name : String,
    crypto_list: Vec<String>,
    fecha_creacion: NaiveDate
}
#[warn(unused)]
fn main() {
    if my_prompt_boolean() == true {
      /* let nombre_consulta =  my_prompt_text();
      println!("Nombre de la consulta :{0}", nombre_consulta.green());        
      
      let crypto_name = my_prompt_select();
      println!("Token selecionado :{0}", crypto_name.yellow());        

      let crypto_names = my_prompt_multiselect();
      for i in crypto_names{
        println!("Token selecionado :{:?}", i);      

      }
      let fecha = my_promt_date();
      println!("Token selecionado :{:#?}", fecha.format("%y%m%d").to_string().yellow());      
     */
     let Consulta = ConsultaCrypto{
        name : my_prompt_text(),
        crypto_list: my_prompt_multiselect(),
        fecha_creacion: my_promt_date()
     };   
     println!("Creacion de consulta: {:#?}", Consulta);
    }    
    

    
}
fn my_vec_list_crypto()-> Vec<String>{
    let list = vec![
        "bitcoin".to_string(),
        "polkadot".to_string(),
        "cardano".to_string(),
        "etherum".to_string()
    ];
    list
}

fn my_promt_date ()-> NaiveDate{
    let message = "Seleccione fecha de creacion deConsulta.".yellow();
    let msg_error ="Error al seleccionar fecha creacion".red();

    let selected_date = inquire::DateSelect::new(&message)
    .prompt()
    .expect(&msg_error);
selected_date
}
fn my_prompt_multiselect()-> Vec<String>{
    let cryptos_list = my_vec_list_crypto();
    let promt_error ="Error al seleccionar los tokens".red();
    let promt_message = "Por favor ingrese una/s cripto/s para buscar".yellow();
    let select = inquire::MultiSelect::new(&promt_message, cryptos_list)
    .prompt()
    .expect(&promt_error);

    select
     
}
fn my_prompt_select()-> String{
    let cryptos_list = my_vec_list_crypto();

    let promt_error ="Error al seleccionar un token".red();
    let promt_message = "Por favor ingrese una cripto para buscar".yellow();
    let select = inquire::Select::new(&promt_message,cryptos_list)
    .prompt()
    .expect(&promt_error);
select

}
fn my_prompt_text()-> String{
    let prompt_message = "Nombre de la consulta ...".yellow();
    let validator = |input : &str| if input.chars().count() < 20 {
        let f_chart = input.chars().next().unwrap() as u8 ;
        match f_chart{
            65..=90 => {
                return Ok(Validation::Valid);
            }
             _     => {
                return Ok(Validation::Invalid("Nombre debe empezar con Mayuscula".into()));
            }    
             }
        }else {
            return Ok(Validation::Invalid("Nombre muy largo , solo puede usar 20 caracteres".into()));

        };

    
    let prompt_name = inquire::Text::new(&prompt_message)
         .with_validator(validator)
         .prompt()
         .expect("Error al obtener nombre consulta .");
    
    prompt_name 
}
fn my_prompt_boolean()-> bool {
    let message = "Are you ready to proceed?".yellow();
    let msg_error = "Error  to proceed?".red();
    let proceed = inquire::prompt_confirmation(message);
    //.expect(&msg_error);
    
    if proceed.is_err(){
        println!("{0}", msg_error);
        return false;
    }
    
   proceed.unwrap()

}