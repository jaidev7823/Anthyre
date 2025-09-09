// src/activity/models.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeBlock {
    pub time_range: String,
    pub planned_activities: Vec<Activity>,
    pub actual_activities: Vec<Activity>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Activity {
    pub title: String,
    pub duration_minutes: f64,
    pub category: String,
    pub status: ActivityStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ActivityStatus {
    Completed,
    Partial,
    Overrun,
    Distracted,
    Missed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DailySummary {
    pub total_planned_hours: f64,
    pub total_actual_hours: f64,
    pub productivity_score: f64,
    pub main_categories: Vec<CategorySummary>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategorySummary {
    pub name: String,
    pub planned_hours: f64,
    pub actual_hours: f64,
    pub variance: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealityCheck {
    pub planned_vs_actual: f64,
    pub distractions_detected: usize,
    pub time_overruns: usize,
    pub focus_score: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Suggestion {
    pub title: String,
    pub description: String,
    pub priority: PriorityLevel,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PriorityLevel {
    High,
    Medium,
    Low,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FrontendData {
    pub time_blocks: Vec<TimeBlock>,
    pub daily_summary: DailySummary,
    pub reality_check: RealityCheck,
    pub suggestions: Vec<Suggestion>,
    pub date: String,
}

// For planned events from calendar
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlannedEvent {
    pub title: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub duration_minutes: f64,
    pub category: String,
}