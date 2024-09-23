use chrono::prelude::*;
use chrono::TimeZone;
use std::time::SystemTime;
use std::{fs, time::UNIX_EPOCH};

use ratatui::{
    layout::{Constraint, Layout},
    style::{Style, Stylize},
    widgets::{Block, Paragraph, Row, Table, TableState},
    Frame,
};

use crate::app::App;
use crate::theme;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame, theme: theme::Theme) {
}
