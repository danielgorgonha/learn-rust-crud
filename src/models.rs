// Este struct representa um registro de dados do nosso CRUD.
// Ele será convertido automaticamente para JSON usando Serde.
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataEntry {
    pub data1: Vec<String>, // Lista de textos
    pub data2: Vec<u8>,     // Lista de números inteiros (bytes)
}