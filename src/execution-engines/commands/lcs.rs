use std::cmp::max;
use std::error::Error;

use crate::exe_engine::cores::{Command, CommandExecutor, CommandExecutorError, CommandResult};
use crate::storage::cores::Storage;

pub struct Lcs;

impl CommandExecutor for Lcs {
    fn execute(storage: &mut Box<dyn Storage>, cmd: &Command) -> Result<CommandResult, Box<dyn Error>> {
        let key1: String = cmd.get_required("KEY1")?;
        let key2: String = cmd.get_required("KEY2")?;
        let entry1 = storage.read(&key1)?;
        let entry2 = storage.read(&key2)?;

        if !entry1.is_primitive() || !entry2.is_primitive() {
            let err = Box::new(CommandExecutorError::NotSupportedDataType);
            return Err(err);
        }

        if entry1.is_nil() {
            let err = Box::new(CommandExecutorError::KeyNotFound(key1));
            return Err(err);
        }

        if entry2.is_nil() {
            let err = Box::new(CommandExecutorError::KeyNotFound(key2));
            return Err(err);
        }

        let data1 = entry1.get_data().to_string();
        let data2 = entry2.get_data().to_string();

        let lcs_table = Lcs::build_lcs_table(&data1, &data2);
        let length_only: Option<bool> = cmd.get_optional("LENGTH_ONLY")?;
        let length_only = length_only.unwrap_or(false);
        let lcs_value = lcs_table.last().unwrap().last().unwrap();

        if length_only {
            return Ok(CommandResult::from(*lcs_value));
        }

        if *lcs_value == 0 {
            return Ok(CommandResult::from(String::from("")));
        }

        let lcs = Lcs::find_lcs(&lcs_table, &data1).first().unwrap().clone();
        Ok(CommandResult::from(lcs))
    }
}

impl Lcs {
    fn build_lcs_table(s1: &String, s2: &String) -> Vec<Vec<i128>> {
        let l1 = s1.len() + 1;
        let l2 = s2.len() + 1;
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let mut lcs_table = vec!(vec!(0i128; l2); l1);

        for i in 1..l1 {
            for j in 1..l2 {
                if s1[i - 1] != s2[j - 1] {
                    lcs_table[i][j] = max(lcs_table[i - 1][j], lcs_table[i][j - 1]);
                    continue;
                }
                lcs_table[i][j] = 1 + lcs_table[i - 1][j - 1];
            }
        }

        lcs_table
    }

    fn find_lcs(lcs_table: &Vec<Vec<i128>>, s1: &String) -> Vec<String> {
        let l1 = lcs_table.len();
        let l2 = lcs_table[0].len();
        let s1: Vec<char> = s1.chars().collect();
        let mut lcs = vec![];
        let mut result = String::from("");

        let mut i = l1 - 1;
        let mut j = l2 - 1;
        while i >= 1 && j >= 1 {
            if lcs_table[i][j - 1] == lcs_table[i][j] {
                j -= 1;
                continue;
            }
            if lcs_table[i - 1][j] == lcs_table[i][j] {
                i -= 1;
                continue;
            }
            result.push(s1[i - 1]);
            j -= 1;
            i -= 1;
        }

        let result: String = result.chars().rev().collect();
        lcs.push(result);
        lcs
    }
}
