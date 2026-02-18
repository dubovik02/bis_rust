use std::io::{BufReader, Read};

use serde::{Serialize, Deserialize};
use serde_json::Result;
use strum_macros::Display;

const CVS_HEADER: &str = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n";
const MAGIC: &str = "YPBN";
const BIN_BODY_LEN: u32 = 46; 

#[derive(Display, Debug, Serialize, Deserialize)]
pub enum TransactionType {
   DEPOSIT = 0, 
   TRANSFER = 1, 
   WITHDRAWAL = 2,
   EMPTY = 3,
}

#[derive(Display, Debug, Serialize, Deserialize)]
pub enum TransactionStatus {
   SUCCESS = 0,
   FAILURE = 1,
   PENDING = 2,
   EMPTY = 3,
}

#[derive(PartialEq, Debug)]
pub enum TransactionsFormatType {
    TXT = 0,
    CSV = 1,
    BIN = 2,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub tx_id: u64,
    pub tx_type: TransactionType,
    pub from_user_id: u64,
    pub to_user_id: u64,
    pub amount: u64,
    pub timestamp: u64,
    pub status: TransactionStatus,
    pub description: String, 
}

impl Transaction {
    pub fn new() -> Self {
        Self {
            tx_id: 0,
            tx_type: TransactionType::EMPTY,
            from_user_id: 0,
            to_user_id: 0,
            amount: 0,
            timestamp: 0,
            status: TransactionStatus::EMPTY,
            description: String::new(),  
        }
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self::new()
    }
}

pub trait TransactionsParser {

    fn get_using_format_type(&self) -> TransactionsFormatType;

    fn from_read<R: std::io::Read>(&self, source: &mut R) -> Result<Vec<Transaction>>;

    fn write_to<W: std::io::Write>(&self, target: &mut W, data: Vec<Transaction>) -> Result<()>;
}

#[derive(Default)]
pub struct TxtParser {
    
}

impl TransactionsParser for TxtParser {
    fn get_using_format_type(&self) -> TransactionsFormatType {
        TransactionsFormatType::TXT
    }

    fn from_read<R: std::io::Read>(&self, source: &mut R) -> Result<Vec<Transaction>> {

        let mut result: Vec<Transaction> = Vec::new();

        let mut str_records = String::new();
        source.read_to_string(&mut str_records);

        let str_arr: Vec<&str> = str_records
            .split("\n\n")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();


        for stx in str_arr {

            let tmp_vec: Vec<&str> = stx
            .split("\n")
            .map(|s| s.trim())
            .filter(|s| !s.contains("#"))
            .collect();

            let json_vec: Vec<String> = tmp_vec  
            .into_iter()  
            .map(|s| txt_to_json_str(s))
            .collect();

            let str_tx = "{".to_owned() + &json_vec.join(",") + "}";
            let tx: Transaction = serde_json::from_str(&str_tx)?;
            result.push(tx);
        }
        Ok(result)
    }

    fn write_to<W: std::io::Write>(&self, target: &mut W, data: Vec<Transaction>) -> Result<()> {

        let mut result_str = String::new();

        for (index, tx) in data.iter().enumerate() {
            result_str = result_str.to_owned() + String::from(
                "# Record".to_owned() +  " " 
                    + (index + 1).to_string().as_str() + " " + "(" + tx.tx_type.to_string().as_str() + ")" + "\n" +
                "TX_ID: " + tx.tx_id.to_string().as_str() + "\n" +
                "TX_TYPE: " + tx.tx_type.to_string().as_str() + "\n" +
                "TO_USER_ID: " + tx.to_user_id.to_string().as_str() + "\n" +
                "FROM_USER_ID: " + tx.from_user_id.to_string().as_str() + "\n" +
                "AMOUNT: " + tx.amount.to_string().as_str() + "\n" +
                "TIMESTAMP: " + tx.timestamp.to_string().as_str() + "\n" +
                "STATUS: " + tx.status.to_string().as_str() + "\n" +  
                "DESCRIPTION: " + "\"" + tx.description.to_string().as_str() + "\"" + "\n" +
                "\n"

            ).as_str();
        }
        target.write_all(result_str.as_bytes());
        Ok(())
    }
}

fn txt_to_json_str(tx: &str) -> String {
    let tx_json = &tx.replace(": ", ":");

    let json_vec: Vec<&str> = tx_json.split(":").into_iter().collect();

    let type_val; 
    let status_val; 

    let key_str = "\"".to_owned() + json_vec[0].to_ascii_lowercase().as_str() + "\"" + ":";

    if json_vec[0] == "TX_TYPE" {
        type_val = json_vec[1];
        return key_str + "\"" + type_val.to_string().as_str() + "\"";
    };

    if json_vec[0] == "STATUS" {
        status_val = json_vec[1];
        return key_str + "\"" + status_val.to_string().as_str() + "\"";
    }  

    let key_val = "".to_owned() + json_vec[1];
    key_str + key_val.as_str()
}


#[derive(Default)]
pub struct CsvParser {
    
}

impl TransactionsParser for CsvParser {
    fn get_using_format_type(&self) -> TransactionsFormatType {
        TransactionsFormatType::CSV
    }

    fn from_read<R: std::io::Read>(&self, source: &mut R) -> Result<Vec<Transaction>> {
        let mut result: Vec<Transaction> = Vec::new();

        let mut str_records = String::new();
        source.read_to_string(&mut str_records);

        let str_arr: Vec<&str> = str_records
            .split("\n")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .filter(|s| !s.contains("TX_ID"))
            .collect();

        for stx in str_arr {
            let tmp_vec: Vec<&str> = stx
            .split(",")
            .map(|s| s.trim())
            .collect();

            let str_tx = 
                "{".to_owned() + 
                "\"tx_id\":" + &tmp_vec[0] + (",") + 
                "\"tx_type\":" + "\"" + &tmp_vec[1] + "\"" + (",") + 
                "\"from_user_id\":" + &tmp_vec[2] + (",") + 
                "\"to_user_id\":" + &tmp_vec[3] + (",") + 
                "\"amount\":" + &tmp_vec[4] + (",") + 
                "\"timestamp\":" + &tmp_vec[5] + (",") + 
                "\"status\":" + "\"" + &tmp_vec[6] + "\"" + (",") + 
                "\"description\":" + &tmp_vec[7] + 
                "}";
            let tx: Transaction = serde_json::from_str(&str_tx)?;
            result.push(tx);
        }
        Ok(result)
    }

    fn write_to<W: std::io::Write>(&self, target: &mut W, data: Vec<Transaction>) -> Result<()> {
        let mut result_str = String::from(CVS_HEADER);

        for tx in data {
            result_str = result_str.to_owned() + String::from(
                tx.tx_id.to_string() + "," +
                tx.tx_type.to_string().as_str() + "," +
                tx.from_user_id.to_string().as_str() + "," +
                tx.to_user_id.to_string().as_str() + "," +
                tx.amount.to_string().as_str() + "," +
                tx.timestamp.to_string().as_str() + "," +
                tx.status.to_string().as_str() + "," +  
                "\"" + tx.description.to_string().as_str() + "\"" +
                "\n"

            ).as_str();
        }
        target.write_all(result_str.as_bytes());
        Ok(())
    }
}

#[derive(Default)]
pub struct BinParser {
    
}

impl TransactionsParser for BinParser {
    fn get_using_format_type(&self) -> TransactionsFormatType {
        TransactionsFormatType::BIN
    }

    fn from_read<R: std::io::Read>(&self, source: &mut R) -> Result<Vec<Transaction>> {

        let mut result: Vec<Transaction> = Vec::new();
        
        let mut reader = BufReader::new(source);
        let mut is_eof = false;

        loop {

            let mut tx = Transaction::new();
            let mut buf4 = [0u8; 4];

            reader.read_exact(&mut buf4);
            if String::from(MAGIC) != String::from_utf8_lossy(&buf4).into_owned() {
                break;
            }

            reader.read_exact(&mut buf4);

            let mut buf8 = [0u8; 8];
            reader.read_exact(&mut buf8);
            tx.tx_id = u64::from_be_bytes(buf8);

            let mut buf1 = [0u8; 1];
            reader.read_exact(&mut buf1);
            tx.tx_type = match u8::from_be_bytes(buf1) 
                {
                    0 => TransactionType::DEPOSIT, 
                    1 => TransactionType::TRANSFER, 
                    2 => TransactionType::WITHDRAWAL, 
                    _ => TransactionType::EMPTY
                };

            reader.read_exact(&mut buf8);
            tx.from_user_id = u64::from_be_bytes(buf8);

            //let mut buf_to_user = [0u8; 8];
            reader.read_exact(&mut buf8);
            tx.to_user_id = u64::from_be_bytes(buf8);

            //let mut buf_amount = [0u8; 8];
            reader.read_exact(&mut buf8);
            tx.amount = u64::from_be_bytes(buf8);

            reader.read_exact(&mut buf8);
            tx.timestamp = u64::from_be_bytes(buf8);

            //let mut buf_status = [0u8; 1];
            reader.read_exact(&mut buf1);
            tx.status = match u8::from_be_bytes(buf1)
            {
                0 => TransactionStatus::SUCCESS,
                1 => TransactionStatus::FAILURE,
                2 => TransactionStatus::PENDING,
                _ => TransactionStatus::EMPTY
            };

            let mut buf_desc_len: [u8; 4] = [0u8; 4];
            reader.read_exact(&mut buf_desc_len);

            let desc_len: usize = u32::from_be_bytes(buf_desc_len) as usize;
            let mut buf_desc = vec![0u8; desc_len];
            reader.read_exact(&mut buf_desc);
            tx.description = String::from_utf8_lossy(&buf_desc).into_owned();

            result.push(tx);
        }
        Ok(result)
    }

    fn write_to<W: std::io::Write>(&self, target: &mut W, data: Vec<Transaction>) -> Result<()> {

        for tx in data {
            target.write(MAGIC.as_bytes());

            let desc_len = tx.description.len();
            let body_len = BIN_BODY_LEN + (desc_len as u32); 
            target.write(&(body_len).to_be_bytes());
            
            target.write(&tx.tx_id.to_be_bytes());
            target.write(
                match tx.tx_type {
                    TransactionType::DEPOSIT => &[0],
                    TransactionType::TRANSFER => &[1],
                    TransactionType::WITHDRAWAL => &[2],
                    _ => &[3]
                }
            );
            target.write(&tx.from_user_id.to_be_bytes());
            target.write(&tx.to_user_id.to_be_bytes());
            target.write(&tx.amount.to_be_bytes());
            target.write(&tx.timestamp.to_be_bytes());
            target.write(
                match tx.status {
                    TransactionStatus::SUCCESS => &[0],
                    TransactionStatus::FAILURE => &[1],
                    TransactionStatus::PENDING => &[2],
                    _ => &[3]
                }
            );
            
            target.write(&(desc_len as u32).to_be_bytes());
            if desc_len != 0 {
                target.write(tx.description.as_bytes());
            }
        }
        Ok(())
    }
}