use super::schema::tehtavat;

#[derive(Queryable, Insertable)]
#[table_name = "tehtavat"]
pub struct Tehtava {
    pub id: i32,
    pub paivamaara: chrono::NaiveDate,
    pub tyoaika: f64,  
    pub lyhyt_kuvaus: String,
    pub tarkempi_kuvaus: String,
}
