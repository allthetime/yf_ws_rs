// Automatically generated rust module for 'yaticker.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::borrow::Cow;
use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Yaticker<'a> {
    pub id: Cow<'a, str>,
    pub price: f32,
    pub time: i64,
    pub currency: Cow<'a, str>,
    pub exchange: Cow<'a, str>,
    pub quoteType: yaticker::mod_Yaticker::QuoteType,
    pub marketHours: yaticker::mod_Yaticker::MarketHoursType,
    pub changePercent: f32,
    pub dayVolume: i64,
    pub dayHigh: f32,
    pub dayLow: f32,
    pub change: f32,
    pub shortName: Cow<'a, str>,
    pub expireDate: i64,
    pub openPrice: f32,
    pub previousClose: f32,
    pub strikePrice: f32,
    pub underlyingSymbol: Cow<'a, str>,
    pub openInterest: i64,
    pub optionsType: yaticker::mod_Yaticker::OptionType,
    pub miniOption: i64,
    pub lastSize: i64,
    pub bid: f32,
    pub bidSize: i64,
    pub ask: f32,
    pub askSize: i64,
    pub priceHint: i64,
    pub vol_24hr: i64,
    pub volAllCurrencies: i64,
    pub fromcurrency: Cow<'a, str>,
    pub lastMarket: Cow<'a, str>,
    pub circulatingSupply: f64,
    pub marketcap: f64,
}

impl<'a> MessageRead<'a> for Yaticker<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            dbg!(r.len());
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(21) => msg.price = r.read_float(bytes)?,
                Ok(24) => msg.time = r.read_sint64(bytes)?,
                Ok(34) => msg.currency = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.exchange = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(48) => msg.quoteType = r.read_enum(bytes)?,
                Ok(56) => msg.marketHours = r.read_enum(bytes)?,
                Ok(69) => msg.changePercent = r.read_float(bytes)?,
                Ok(72) => msg.dayVolume = r.read_sint64(bytes)?,
                Ok(85) => msg.dayHigh = r.read_float(bytes)?,
                Ok(93) => msg.dayLow = r.read_float(bytes)?,
                Ok(101) => msg.change = r.read_float(bytes)?,
                Ok(106) => msg.shortName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(112) => msg.expireDate = r.read_sint64(bytes)?,
                Ok(125) => msg.openPrice = r.read_float(bytes)?,
                Ok(133) => msg.previousClose = r.read_float(bytes)?,
                Ok(141) => msg.strikePrice = r.read_float(bytes)?,
                Ok(146) => msg.underlyingSymbol = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(152) => msg.openInterest = r.read_sint64(bytes)?,
                Ok(160) => msg.optionsType = r.read_enum(bytes)?,
                Ok(168) => msg.miniOption = r.read_sint64(bytes)?,
                Ok(176) => msg.lastSize = r.read_sint64(bytes)?,
                Ok(189) => msg.bid = r.read_float(bytes)?,
                Ok(192) => msg.bidSize = r.read_sint64(bytes)?,
                Ok(205) => msg.ask = r.read_float(bytes)?,
                Ok(208) => msg.askSize = r.read_sint64(bytes)?,
                Ok(216) => msg.priceHint = r.read_sint64(bytes)?,
                Ok(224) => msg.vol_24hr = r.read_sint64(bytes)?,
                Ok(232) => msg.volAllCurrencies = r.read_sint64(bytes)?,
                Ok(242) => msg.fromcurrency = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(250) => msg.lastMarket = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(257) => msg.circulatingSupply = r.read_double(bytes)?,
                Ok(265) => msg.marketcap = r.read_double(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Yaticker<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.price == 0f32 { 0 } else { 1 + 4 }
        + if self.time == 0i64 { 0 } else { 1 + sizeof_sint64(*(&self.time)) }
        + if self.currency == "" { 0 } else { 1 + sizeof_len((&self.currency).len()) }
        + if self.exchange == "" { 0 } else { 1 + sizeof_len((&self.exchange).len()) }
        + if self.quoteType == yaticker::mod_Yaticker::QuoteType::NONE { 0 } else { 1 + sizeof_varint(*(&self.quoteType) as u64) }
        + if self.marketHours == yaticker::mod_Yaticker::MarketHoursType::PRE_MARKET { 0 } else { 1 + sizeof_varint(*(&self.marketHours) as u64) }
        + if self.changePercent == 0f32 { 0 } else { 1 + 4 }
        + if self.dayVolume == 0i64 { 0 } else { 1 + sizeof_sint64(*(&self.dayVolume)) }
        + if self.dayHigh == 0f32 { 0 } else { 1 + 4 }
        + if self.dayLow == 0f32 { 0 } else { 1 + 4 }
        + if self.change == 0f32 { 0 } else { 1 + 4 }
        + if self.shortName == "" { 0 } else { 1 + sizeof_len((&self.shortName).len()) }
        + if self.expireDate == 0i64 { 0 } else { 1 + sizeof_sint64(*(&self.expireDate)) }
        + if self.openPrice == 0f32 { 0 } else { 1 + 4 }
        + if self.previousClose == 0f32 { 0 } else { 2 + 4 }
        + if self.strikePrice == 0f32 { 0 } else { 2 + 4 }
        + if self.underlyingSymbol == "" { 0 } else { 2 + sizeof_len((&self.underlyingSymbol).len()) }
        + if self.openInterest == 0i64 { 0 } else { 2 + sizeof_sint64(*(&self.openInterest)) }
        + if self.optionsType == yaticker::mod_Yaticker::OptionType::CALL { 0 } else { 2 + sizeof_varint(*(&self.optionsType) as u64) }
        + if self.miniOption == 0i64 { 0 } else { 2 + sizeof_sint64(*(&self.miniOption)) }
        + if self.lastSize == 0i64 { 0 } else { 2 + sizeof_sint64(*(&self.lastSize)) }
        + if self.bid == 0f32 { 0 } else { 2 + 4 }
        + if self.bidSize == 0i64 { 0 } else { 2 + sizeof_sint64(*(&self.bidSize)) }
        + if self.ask == 0f32 { 0 } else { 2 + 4 }
        + if self.askSize == 0i64 { 0 } else { 2 + sizeof_sint64(*(&self.askSize)) }
        + if self.priceHint == 0i64 { 0 } else { 2 + sizeof_sint64(*(&self.priceHint)) }
        + if self.vol_24hr == 0i64 { 0 } else { 2 + sizeof_sint64(*(&self.vol_24hr)) }
        + if self.volAllCurrencies == 0i64 { 0 } else { 2 + sizeof_sint64(*(&self.volAllCurrencies)) }
        + if self.fromcurrency == "" { 0 } else { 2 + sizeof_len((&self.fromcurrency).len()) }
        + if self.lastMarket == "" { 0 } else { 2 + sizeof_len((&self.lastMarket).len()) }
        + if self.circulatingSupply == 0f64 { 0 } else { 2 + 8 }
        + if self.marketcap == 0f64 { 0 } else { 2 + 8 }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.price != 0f32 { w.write_with_tag(21, |w| w.write_float(*&self.price))?; }
        if self.time != 0i64 { w.write_with_tag(24, |w| w.write_sint64(*&self.time))?; }
        if self.currency != "" { w.write_with_tag(34, |w| w.write_string(&**&self.currency))?; }
        if self.exchange != "" { w.write_with_tag(42, |w| w.write_string(&**&self.exchange))?; }
        if self.quoteType != yaticker::mod_Yaticker::QuoteType::NONE { w.write_with_tag(48, |w| w.write_enum(*&self.quoteType as i32))?; }
        if self.marketHours != yaticker::mod_Yaticker::MarketHoursType::PRE_MARKET { w.write_with_tag(56, |w| w.write_enum(*&self.marketHours as i32))?; }
        if self.changePercent != 0f32 { w.write_with_tag(69, |w| w.write_float(*&self.changePercent))?; }
        if self.dayVolume != 0i64 { w.write_with_tag(72, |w| w.write_sint64(*&self.dayVolume))?; }
        if self.dayHigh != 0f32 { w.write_with_tag(85, |w| w.write_float(*&self.dayHigh))?; }
        if self.dayLow != 0f32 { w.write_with_tag(93, |w| w.write_float(*&self.dayLow))?; }
        if self.change != 0f32 { w.write_with_tag(101, |w| w.write_float(*&self.change))?; }
        if self.shortName != "" { w.write_with_tag(106, |w| w.write_string(&**&self.shortName))?; }
        if self.expireDate != 0i64 { w.write_with_tag(112, |w| w.write_sint64(*&self.expireDate))?; }
        if self.openPrice != 0f32 { w.write_with_tag(125, |w| w.write_float(*&self.openPrice))?; }
        if self.previousClose != 0f32 { w.write_with_tag(133, |w| w.write_float(*&self.previousClose))?; }
        if self.strikePrice != 0f32 { w.write_with_tag(141, |w| w.write_float(*&self.strikePrice))?; }
        if self.underlyingSymbol != "" { w.write_with_tag(146, |w| w.write_string(&**&self.underlyingSymbol))?; }
        if self.openInterest != 0i64 { w.write_with_tag(152, |w| w.write_sint64(*&self.openInterest))?; }
        if self.optionsType != yaticker::mod_Yaticker::OptionType::CALL { w.write_with_tag(160, |w| w.write_enum(*&self.optionsType as i32))?; }
        if self.miniOption != 0i64 { w.write_with_tag(168, |w| w.write_sint64(*&self.miniOption))?; }
        if self.lastSize != 0i64 { w.write_with_tag(176, |w| w.write_sint64(*&self.lastSize))?; }
        if self.bid != 0f32 { w.write_with_tag(189, |w| w.write_float(*&self.bid))?; }
        if self.bidSize != 0i64 { w.write_with_tag(192, |w| w.write_sint64(*&self.bidSize))?; }
        if self.ask != 0f32 { w.write_with_tag(205, |w| w.write_float(*&self.ask))?; }
        if self.askSize != 0i64 { w.write_with_tag(208, |w| w.write_sint64(*&self.askSize))?; }
        if self.priceHint != 0i64 { w.write_with_tag(216, |w| w.write_sint64(*&self.priceHint))?; }
        if self.vol_24hr != 0i64 { w.write_with_tag(224, |w| w.write_sint64(*&self.vol_24hr))?; }
        if self.volAllCurrencies != 0i64 { w.write_with_tag(232, |w| w.write_sint64(*&self.volAllCurrencies))?; }
        if self.fromcurrency != "" { w.write_with_tag(242, |w| w.write_string(&**&self.fromcurrency))?; }
        if self.lastMarket != "" { w.write_with_tag(250, |w| w.write_string(&**&self.lastMarket))?; }
        if self.circulatingSupply != 0f64 { w.write_with_tag(257, |w| w.write_double(*&self.circulatingSupply))?; }
        if self.marketcap != 0f64 { w.write_with_tag(265, |w| w.write_double(*&self.marketcap))?; }
        Ok(())
    }
}

pub mod mod_Yaticker {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum QuoteType {
    NONE = 0,
    ALTSYMBOL = 5,
    HEARTBEAT = 7,
    EQUITY = 8,
    INDEX = 9,
    MUTUALFUND = 11,
    MONEYMARKET = 12,
    OPTION = 13,
    CURRENCY = 14,
    WARRANT = 15,
    BOND = 17,
    FUTURE = 18,
    ETF = 20,
    COMMODITY = 23,
    ECNQUOTE = 28,
    CRYPTOCURRENCY = 41,
    INDICATOR = 42,
    INDUSTRY = 1000,
}

impl Default for QuoteType {
    fn default() -> Self {
        QuoteType::NONE
    }
}

impl From<i32> for QuoteType {
    fn from(i: i32) -> Self {
        match i {
            0 => QuoteType::NONE,
            5 => QuoteType::ALTSYMBOL,
            7 => QuoteType::HEARTBEAT,
            8 => QuoteType::EQUITY,
            9 => QuoteType::INDEX,
            11 => QuoteType::MUTUALFUND,
            12 => QuoteType::MONEYMARKET,
            13 => QuoteType::OPTION,
            14 => QuoteType::CURRENCY,
            15 => QuoteType::WARRANT,
            17 => QuoteType::BOND,
            18 => QuoteType::FUTURE,
            20 => QuoteType::ETF,
            23 => QuoteType::COMMODITY,
            28 => QuoteType::ECNQUOTE,
            41 => QuoteType::CRYPTOCURRENCY,
            42 => QuoteType::INDICATOR,
            1000 => QuoteType::INDUSTRY,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for QuoteType {
    fn from(s: &'a str) -> Self {
        match s {
            "NONE" => QuoteType::NONE,
            "ALTSYMBOL" => QuoteType::ALTSYMBOL,
            "HEARTBEAT" => QuoteType::HEARTBEAT,
            "EQUITY" => QuoteType::EQUITY,
            "INDEX" => QuoteType::INDEX,
            "MUTUALFUND" => QuoteType::MUTUALFUND,
            "MONEYMARKET" => QuoteType::MONEYMARKET,
            "OPTION" => QuoteType::OPTION,
            "CURRENCY" => QuoteType::CURRENCY,
            "WARRANT" => QuoteType::WARRANT,
            "BOND" => QuoteType::BOND,
            "FUTURE" => QuoteType::FUTURE,
            "ETF" => QuoteType::ETF,
            "COMMODITY" => QuoteType::COMMODITY,
            "ECNQUOTE" => QuoteType::ECNQUOTE,
            "CRYPTOCURRENCY" => QuoteType::CRYPTOCURRENCY,
            "INDICATOR" => QuoteType::INDICATOR,
            "INDUSTRY" => QuoteType::INDUSTRY,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OptionType {
    CALL = 0,
    PUT = 1,
}

impl Default for OptionType {
    fn default() -> Self {
        OptionType::CALL
    }
}

impl From<i32> for OptionType {
    fn from(i: i32) -> Self {
        match i {
            0 => OptionType::CALL,
            1 => OptionType::PUT,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for OptionType {
    fn from(s: &'a str) -> Self {
        match s {
            "CALL" => OptionType::CALL,
            "PUT" => OptionType::PUT,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MarketHoursType {
    PRE_MARKET = 0,
    REGULAR_MARKET = 1,
    POST_MARKET = 2,
    EXTENDED_HOURS_MARKET = 3,
}

impl Default for MarketHoursType {
    fn default() -> Self {
        MarketHoursType::PRE_MARKET
    }
}

impl From<i32> for MarketHoursType {
    fn from(i: i32) -> Self {
        match i {
            0 => MarketHoursType::PRE_MARKET,
            1 => MarketHoursType::REGULAR_MARKET,
            2 => MarketHoursType::POST_MARKET,
            3 => MarketHoursType::EXTENDED_HOURS_MARKET,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for MarketHoursType {
    fn from(s: &'a str) -> Self {
        match s {
            "PRE_MARKET" => MarketHoursType::PRE_MARKET,
            "REGULAR_MARKET" => MarketHoursType::REGULAR_MARKET,
            "POST_MARKET" => MarketHoursType::POST_MARKET,
            "EXTENDED_HOURS_MARKET" => MarketHoursType::EXTENDED_HOURS_MARKET,
            _ => Self::default(),
        }
    }
}

}

