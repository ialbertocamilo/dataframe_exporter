use mysql::{Pool, Row};
use mysql::prelude::Queryable;
use polars::prelude::*;
use polars_excel_writer::xlsx_writer::PolarsXlsxWriter;

use performance::calculate_delay;
#[derive(Debug)]
struct User {
    id: i32,
    // Adjust data types based on your table schema
    username: String,
    // ... other user data fields
}


#[calculate_delay]
fn get_data() {
    let db_url = "mysql://user:1234@localhost:3307/db";
    let pool = Pool::new(db_url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let query = "
                SELECT
                  COALESCE(UPPER(CONCAT(u.name, ' ', u.lastname, ' ', u.surname)), '') AS 'NOMBRE COMPLETO',
                  UPPER(u.name) AS NOMBRE,
                  UPPER(u.lastname) AS 'APELLIDO PATERNO',
                  COALESCE(u.document,'') AS 'DOCUMENTO',
                  u.phone_number AS 'NUMERO DE TELEFONO',
                  u.person_number AS 'NUMERO DE PERSONA COLABORADOR',
                  CASE u.active
                    WHEN 1 THEN 'ACTIVO'
                    WHEN 0 THEN 'INACTIVO'
                  END AS 'ESTADO',
                  u.email AS 'EMAIL',
                  DATE_FORMAT(u.updated_at, '%d/%m/%Y %H:%i:%S') AS 'ULTIMA SESION',
                  DATE_FORMAT(u.created_at, '%d/%m/%Y %H:%i:%S') AS 'FECHA DE CREACION',
                  w.name as 'MODULO',
                  u.last_summary_updated_at as 'FECHA DE INGRESO'
                FROM users u
                INNER JOIN taxonomies t ON u.type_id = t.id
                INNER JOIN workspaces w ON u.subworkspace_id = w.id
                and u.subworkspace_id in (select workspaces.id from workspaces where workspaces.active=1 and workspaces.parent_id=13)
                AND t.code = 'employee'
                AND u.deleted_at IS NULL";

    let mut result = conn.query_map(query, |row: Row| {
        (row.get::<String, _>("NOMBRE COMPLETO").unwrap(), row.get::<String, _>("DOCUMENTO").unwrap())
    }).unwrap();
    let (id_data, name_data): (Vec<_>, Vec<_>) = result.into_iter().unzip();

    // let mut df: polars_core::frame::DataFrame = df!("id"=>id_data,"name"=>name_data).unwrap();

    let mut df: DataFrame = DataFrame::new(vec![
        Series::new("NOMBRE COMPLETO", id_data),
        Series::new("DOCUMENTO", name_data),
    ]).unwrap();

    // let mut id = Mutex::new(Vec::new());
    // let mut name = Mutex::new(Vec::new());
    // let result = conn.query(query).unwrap();
    // result.par_iter().for_each(|row: &Row| {
    //     id.lock().unwrap().push(row.get::<i32, _>("id").unwrap());
    //     name.lock().unwrap().push(row.get::<String, _>("name").unwrap());
    // });
    // let mut df = DataFrame::new(vec![
    //     Series::new("id", id.lock().unwrap().iter().cloned().collect::<Vec<_>>()),
    //     Series::new("name", name.lock().unwrap().iter().cloned().collect::<Vec<_>>())]).unwrap();

    let mut file = std::fs::File::create("./log.xlsx").unwrap();

    let mut xlsx_writer = PolarsXlsxWriter::new();
    xlsx_writer.write_dataframe(&df).unwrap();
    xlsx_writer.save("log.xlsx").unwrap();
    // CsvWriter::new(&mut file).include_header(false).finish(&mut df).unwrap();


    println!("Function finish");
}


// #[tokio::main]
fn main() -> std::io::Result<()> {
    get_data();
    Ok(())
}
