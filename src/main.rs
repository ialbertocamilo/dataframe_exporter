use std::borrow::Cow;
use std::fmt::Display;
use std::sync::Mutex;

use mysql::{Pool, Row, Value};
use mysql::prelude::{Queryable, ToValue};
use polars::prelude::*;
use rayon::prelude::*;

#[derive(Debug)]
struct User {
    nombre_completo: String,
    nombre: String,
    apellido_paterno: String,
    documento: String,
    numero_de_telefono: String,
    numero_de_persona_colaborador: String,
    estado: String,
    email: String,
    ultima_sesion: String,
    fecha_de_creacion: String,
    modulo: String,
    fecha_de_ingreso: String,
}


fn null_to_empty_string(value: Value) -> String {
    match value {
        Value::NULL => String::try_from("").unwrap(),
        _ => String::try_from(value.to_value()).unwrap()
    }
}

#[derive(Debug)]
struct GenericValue {
    columns: Vec<String>,
    values: Vec<String>,
}

// #[calculate_delay]
fn process_report(query: String) {
    let db_url = "mysql://user:1234@localhost:3307/db";
    let pool = Pool::new(db_url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    println!("Query {}\n", query);
    let mut series= Mutex::new(vec![]);
    let mut result = conn
        .query(query)
        .unwrap();
    result.par_iter().for_each(|val: &Row| {
        // println!("Columns :{:?}",&val.clone().columns_ref().iter().nth(0).unwrap().name_str());
       let _= &val.clone().columns_ref().into_iter().enumerate().for_each(|(index,col)|{
           
            let val=&val.clone().unwrap().into_iter().nth(index).unwrap();
           series.lock().unwrap().push(Series::new(col.name_str().to_string().as_str(),[null_to_empty_string(val.to_value())]));
            println!("Valor {:?}",&series);
        });
        // println!("Arr {}",&val.clone().unwrap().iter().map(|val|null_to_empty_string(val.to_value())).collect::<String>())
    })
    // let mut df = DataFrame::new(series).unwrap();
    //
    // Escribir el DataFrame en un archivo XLSX
    // let mut xlsx_writer = PolarsXlsxWriter::new();
    // xlsx_writer.write_dataframe(&mut df).unwrap();
    // xlsx_writer.save("log.xlsx").unwrap();
}


// fn get_data() {
//     let db_url = "mysql://user:1234@localhost:3307/db";
//     let pool = Pool::new(db_url).unwrap();
//     let mut conn = pool.get_conn().unwrap();
//     let query = "
//                 SELECT
//                   COALESCE(UPPER(CONCAT(u.name, ' ', u.lastname, ' ', u.surname)), '') AS 'NOMBRE COMPLETO',
//                   UPPER(u.name) AS NOMBRE,
//                   UPPER(u.lastname) AS 'APELLIDO PATERNO',
//                   COALESCE(u.document,'') AS 'DOCUMENTO',
//                   u.phone_number AS 'NUMERO DE TELEFONO',
//                   u.person_number AS 'NUMERO DE PERSONA COLABORADOR',
//                   CASE u.active
//                     WHEN 1 THEN 'ACTIVO'
//                     WHEN 0 THEN 'INACTIVO'
//                   END AS 'ESTADO',
//                   u.email AS 'EMAIL',
//                   DATE_FORMAT(u.updated_at, '%d/%m/%Y %H:%i:%S') AS 'ULTIMA SESION',
//                   DATE_FORMAT(u.created_at, '%d/%m/%Y %H:%i:%S') AS 'FECHA DE CREACION',
//                   w.name as 'MODULO',
//                   COALESCE(u.last_summary_updated_at,'') as 'FECHA DE INGRESO'
//                 FROM users u
//                 INNER JOIN taxonomies t ON u.type_id = t.id
//                 INNER JOIN workspaces w ON u.subworkspace_id = w.id
//                 and u.subworkspace_id in (select workspaces.id from workspaces where workspaces.active=1 and workspaces.parent_id=13)
//                 AND t.code = 'employee'
//                 AND u.deleted_at IS NULL";
//
//     let mut result = conn.query_map(query, |row: Row| {
//         User {
//             nombre_completo: row.get("NOMBRE COMPLETO").unwrap(),
//             nombre: null_to_empty_string::<String>(row.get("NOMBRE").unwrap()),
//             apellido_paterno: null_to_empty_string::<String>(row.get("APELLIDO PATERNO").unwrap()),
//             documento: null_to_empty_string::<String>(row.get("DOCUMENTO").unwrap()),
//             numero_de_telefono: null_to_empty_string::<String>(row.get("NUMERO DE TELEFONO").unwrap()),
//             numero_de_persona_colaborador: null_to_empty_string::<String>(row.get("NUMERO DE PERSONA COLABORADOR").unwrap()),
//             estado: null_to_empty_string::<String>(row.get("ESTADO").unwrap()),
//             email: null_to_empty_string::<String>(row.get("EMAIL").unwrap()),
//             ultima_sesion: null_to_empty_string::<String>(row.get("ULTIMA SESION").unwrap()),
//             fecha_de_creacion: null_to_empty_string::<String>(row.get("FECHA DE CREACION").unwrap()),
//             modulo: null_to_empty_string::<String>(row.get("MODULO").unwrap()),
//             fecha_de_ingreso: null_to_empty_string::<String>(row.get("FECHA DE INGRESO").unwrap()),
//         }
//     }).unwrap();
//     // let (id_data, name_data): (Vec<_>, Vec<_>) = result.into_iter().unzip();
//
//     // let mut df: polars_core::frame::DataFrame = df!("id"=>id_data,"name"=>name_data).unwrap();
//
//
//     let mut df = df!(
//         "NOMBRE COMPLETO"=>result.iter().map(|val| {val.nombre_completo.clone()}).collect::<Vec<_>>(),
//         "APELLIDO PATERNO"=>result.iter().map(|val| {val.apellido_paterno.clone()}).collect::<Vec<_>>(),
//         "DOCUMENTO"=>result.iter().map(|val| {val.documento.clone()}).collect::<Vec<_>>(),
//         "NUMERO DE TELEFONO"=>result.iter().map(|val| {val.numero_de_telefono.clone()}).collect::<Vec<_>>(),
//         "NUMERO DE PERSONA COLABORADOR"=>result.iter().map(|val| {val.numero_de_persona_colaborador.clone()}).collect::<Vec<_>>(),
//         "ESTADO"=>result.iter().map(|val| {val.estado.clone()}).collect::<Vec<_>>(),
//         "EMAIL"=>result.iter().map(|val| {val.email.clone()}).collect::<Vec<_>>(),
//         "ULTIMA SESION"=>result.iter().map(|val| {val.ultima_sesion.clone()}).collect::<Vec<_>>(),
//         "FECHA DE CREACION"=>result.iter().map(|val| {val.fecha_de_creacion.clone()}).collect::<Vec<_>>(),
//         "MODULO"=>result.iter().map(|val| {val.modulo.clone()}).collect::<Vec<_>>(),
//         "FECHA DE INGRESO"=>result.iter().map(|val| {val.fecha_de_ingreso.clone()}).collect::<Vec<_>>()
//     ).unwrap();
//
//
//     // let mut id = Mutex::new(Vec::new());
//     // let mut name = Mutex::new(Vec::new());
//     // let result = conn.query(query).unwrap();
//     // result.par_iter().for_each(|row: &Row| {
//     //     id.lock().unwrap().push(row.get::<i32, _>("id").unwrap());
//     //     name.lock().unwrap().push(row.get::<String, _>("name").unwrap());
//     // });
//     // let mut df = DataFrame::new(vec![
//     //     Series::new("id", id.lock().unwrap().iter().cloned().collect::<Vec<_>>()),
//     //     Series::new("name", name.lock().unwrap().iter().cloned().collect::<Vec<_>>())]).unwrap();
//
//     let mut file = std::fs::File::create("./log.xlsx").unwrap();
//
//     let mut xlsx_writer = PolarsXlsxWriter::new();
//     xlsx_writer.write_dataframe(&df).unwrap();
//     xlsx_writer.save("log.xlsx").unwrap();
//     // CsvWriter::new(&mut file).include_header(false).finish(&mut df).unwrap();
//
//
//     println!("Function finish");
// }


// #[tokio::main]
fn main() -> std::io::Result<()> {
    // get_data();


    process_report(String::from("select * from users"));
    Ok(())
}
