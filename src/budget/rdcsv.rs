use serde::{ Deserialize};
use std::error::Error;
use std::path::Path;
//use csv::{ReaderBuilder};

use super::nodedata::{BEACat, LKV, Budget, Acct, BKey};
//use ndarray_csv::{ArrayReader, ArrayWriter};
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone)]
struct Record {
    AgencyCode: i32,
    AgencyName: String,
    BureauCode: i32,
    BureauName: String,
    AccountCode: Option<i32>,
    AccountName: String,
    TreasuryAgencyCode: Option<i16>,
    SubfunctionCode: i16,
    SubfunctionTitle: String,
    BEACategory: String,
    OnorOffBudget: String,
    Y1976: i64,	
    TQ: i64,
    Y1977: i64,
    Y1978: i64,
    Y1979: i64,
    Y1980: i64,
    Y1981: i64,
    Y1982: i64,
    Y1983: i64,
    Y1984: i64,
    Y1985: i64,
    Y1986: i64,
    Y1987: i64,
    Y1988: i64,
    Y1989: i64,
    Y1990: i64,
    Y1991: i64,
    Y1992: i64,
    Y1993: i64,
    Y1994: i64,
    Y1995: i64,
    Y1996: i64,
    Y1997: i64,
    Y1998: i64,
    Y1999: i64,
    Y2000: i64,
    Y2001: i64,
    Y2002: i64,
    Y2003: i64,
    Y2004: i64,
    Y2005: i64,
    Y2006: i64,
    Y2007: i64,
    Y2008: i64,
    Y2009: i64,
    Y2010: i64,
    Y2011: i64,
    Y2012: i64,
    Y2013: i64,
    Y2014: i64,
    Y2015: i64,
    Y2016: i64,
    Y2017: i64,
    Y2018: i64,
    Y2019: i64,
    Y2020: i64,
    Y2021: i64,
    Y2022: i64,
    Y2023: i64,
    Y2024: i64,
    idx: Option<i16>
}


fn newleaf(rec: &Record, idx:i32, a: i32, b: i32, s: i32) -> Acct {
    let val: Vec<i64> = vec![
rec.Y1976, rec.TQ, rec.Y1977, rec.Y1978, rec.Y1979, rec.Y1980, rec.Y1981,
rec.Y1982, rec.Y1983, rec.Y1984, rec.Y1985, rec.Y1986, rec.Y1987,
rec.Y1988, rec.Y1989, rec.Y1990, rec.Y1991, rec.Y1992, rec.Y1993,
rec.Y1994, rec.Y1995, rec.Y1996, rec.Y1997, rec.Y1998, rec.Y1999,
rec.Y2000, rec.Y2001, rec.Y2002, rec.Y2003, rec.Y2004, rec.Y2005,
rec.Y2006, rec.Y2007, rec.Y2008, rec.Y2009, rec.Y2010, rec.Y2011,
rec.Y2012, rec.Y2013, rec.Y2014, rec.Y2015, rec.Y2016, rec.Y2017,
rec.Y2018, rec.Y2019, rec.Y2020, rec.Y2021, rec.Y2022, rec.Y2023,
rec.Y2024
    ];
    //let val: Vec<i64> = Vec::new();
    // let val = val.iter().map(|v| {v * 1000000}).collect();
    let ccode = match rec.AccountCode {
        Some(v) => v,
        None => i32::min_value()
    };
    let acct = Acct {
        idx: idx,
        key: BKey::new(rec.AgencyCode as i16, rec.BureauCode as i16, ccode),
        
        name: rec.AccountName.clone(),
        tac: match rec.TreasuryAgencyCode {
            Some(v) => v,
            None => i16::min_value()
        },
        scode: rec.SubfunctionCode,
        bea: match rec.BEACategory.as_ref() {
            "Net interest" => {BEACat::I},
            "Mandatory" => {BEACat::M},
            "Discretionary" => {BEACat::D},
            _ => {println!("Bad BEACat"); BEACat::X}
        },
        onoff: match rec.OnorOffBudget.as_ref() {
            "On-budget" => { true },
            "Off-budget" => { false },
            _ => { false }
        },
        astr: a,
        bstr: b,
        sub: s,
        value: val 
    };
    acct
}

fn idx_find_or_insert(vec: &mut Vec<LKV>, val: String, idx: i32) -> i32 {
    let kv = LKV::new(idx, val);
    if let Some(i) = (0..vec.len()).find(|&i| vec[i] == kv ) {
        i as i32
    } else {
        vec.push(kv);
        (vec.len() - 1)  as i32
    }
}

pub fn rtn_budget(data: String) -> Budget {
//pub fn rtn_budget(data: &[u8]) -> Budget {
//pub fn rtn_budget(data: &[u8]) -> Result<Budget, Box<dyn Error>> {
    //let b = data.as_bytes();
    let d  = data.as_str();
    let mut cnt: i16 = 0;  //d.len() as i16;
    let mut reader = csv::ReaderBuilder::new().has_headers(true).from_reader(d.as_bytes());
 
 //   let reader = ReaderBuilder::new().has_headers(true).from_path(path).unwrap();
    let mut anames: Vec<LKV> = Vec::new();
    let mut bnames: Vec<LKV> = Vec::new();
    let mut snames: Vec<LKV> = Vec::new();
    let mut accts: Vec<Acct> = Vec::new();
    
    //println!("{:?}", hdr.len());
    //let (idx, rslt) = reader.deserialize().enumerate() {
    let iter = reader.deserialize();
    for (idx, rslt) in iter.enumerate() {
        let record: Record = rslt.unwrap();
        cnt = cnt + 1;
        //println!("{:?}", record);
        let a = idx_find_or_insert(anames.as_mut(), record.AgencyName.clone(), record.AgencyCode);
        let b = idx_find_or_insert(bnames.as_mut(), record.BureauName.clone(), record.BureauCode);
        let s = idx_find_or_insert(snames.as_mut(), record.SubfunctionTitle.clone(), record.SubfunctionCode as i32);
        //println!("{:?} {:?}", a, record);
        let acct = newleaf(&record, idx as i32, a, b, s);
 
        accts.push(acct);
    }
    let zot = reader.is_done();

    cnt = accts.len() as i16;
   println!("{:?}", accts);
    let budget = Budget { anames: anames, bnames: bnames, sname: snames, accts: accts};
    //Ok(budget)
    //budget
    budget
}
