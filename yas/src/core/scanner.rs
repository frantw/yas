use super::*;
use crate::common::color::Color;
use crate::core::inference::inference::CRNNModel;
use std::sync::Arc;

use enigo::Enigo;

pub struct YasScanner {
    model: Arc<CRNNModel>,
    enigo: Enigo,

    info: ScanInfo,
    config: YasScannerConfig,

    row: usize,
    col: usize,

    pool: f64,

    initial_color: Color,

    // for scrolls
    scrolled_rows: u32,
    avg_scroll_one_row: f64,

    avg_switch_time: f64,
    scanned_count: u32,

    is_cloud: bool,
}

pub trait Scanner {
    fn scan(&mut self) -> ScanResult;
}

impl YasScanner {}
