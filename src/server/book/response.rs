use crate::server::book::model::Book;
use crate::server::book::model::Chapter;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BookResponse {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerateBookOutlineResponse {
    pub chapters: Vec<Chapter>,
    pub book: Book,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AnalyticsData {
    pub engagement: EngagementStats,
    pub ai_usage: AIUsageStats,
    pub predictions: PredictiveStats,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct EngagementStats {
    pub total_books: u64,
    pub total_chapters: u64,
    pub avg_chapters_per_book: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AIUsageStats {
    pub total_ai_chapters: u64,
    pub avg_gen_time: f64,
    pub success_rate: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PredictiveStats {
    pub trending_genre: String,
    pub projected_growth: f64,
}
