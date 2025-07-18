use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::{Connection, params, Row};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorReading {
    pub id: Option<i64>,
    pub timestamp: DateTime<Utc>,
    pub session_id: String,
    pub tgs2600: f64,
    pub tgs2602: f64,
    pub tgs2610: f64,
    pub tgs2611: f64,
    pub mq2: f64,
    pub mq3: f64,
    pub mq7: f64,
    pub mq135: f64,
    pub gas_type: Option<String>,
    pub concentration: Option<f64>,
    pub temperature: Option<f64>,
    pub humidity: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingSession {
    pub id: Option<i64>,
    pub session_id: String,
    pub gas_type: String,
    pub concentration: f64,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub sample_count: i32,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPerformance {
    pub id: Option<i64>,
    pub model_name: String,
    pub timestamp: DateTime<Utc>,
    pub accuracy: f64,
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
    pub confusion_matrix: String,
    pub training_samples: i32,
    pub validation_samples: i32,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: &Path) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        
        // Create tables
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS sensor_readings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                session_id TEXT NOT NULL,
                tgs2600 REAL NOT NULL,
                tgs2602 REAL NOT NULL,
                tgs2610 REAL NOT NULL,
                tgs2611 REAL NOT NULL,
                mq2 REAL NOT NULL,
                mq3 REAL NOT NULL,
                mq7 REAL NOT NULL,
                mq135 REAL NOT NULL,
                gas_type TEXT,
                concentration REAL,
                temperature REAL,
                humidity REAL
            );
            
            CREATE TABLE IF NOT EXISTS training_sessions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id TEXT UNIQUE NOT NULL,
                gas_type TEXT NOT NULL,
                concentration REAL NOT NULL,
                start_time DATETIME NOT NULL,
                end_time DATETIME,
                sample_count INTEGER DEFAULT 0,
                notes TEXT
            );
            
            CREATE TABLE IF NOT EXISTS model_performance (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                model_name TEXT NOT NULL,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                accuracy REAL NOT NULL,
                precision REAL NOT NULL,
                recall REAL NOT NULL,
                f1_score REAL NOT NULL,
                confusion_matrix TEXT,
                training_samples INTEGER,
                validation_samples INTEGER
            );
            
            CREATE INDEX IF NOT EXISTS idx_sensor_readings_timestamp ON sensor_readings(timestamp);
            CREATE INDEX IF NOT EXISTS idx_sensor_readings_session ON sensor_readings(session_id);
            CREATE INDEX IF NOT EXISTS idx_training_sessions_gas_type ON training_sessions(gas_type);"
        )?;
        
        Ok(Self { conn })
    }
    
    pub fn insert_sensor_reading(&self, reading: &SensorReading) -> Result<i64> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO sensor_readings 
             (timestamp, session_id, tgs2600, tgs2602, tgs2610, tgs2611, mq2, mq3, mq7, mq135, 
              gas_type, concentration, temperature, humidity)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)"
        )?;
        
        let id = stmt.insert(params![
            reading.timestamp.format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
            reading.session_id,
            reading.tgs2600,
            reading.tgs2602,
            reading.tgs2610,
            reading.tgs2611,
            reading.mq2,
            reading.mq3,
            reading.mq7,
            reading.mq135,
            reading.gas_type,
            reading.concentration,
            reading.temperature,
            reading.humidity
        ])?;
        
        Ok(id)
    }
    
    pub fn get_training_data(&self, gas_type: Option<&str>) -> Result<Vec<SensorReading>> {
        let mut sql = "SELECT * FROM sensor_readings WHERE gas_type IS NOT NULL".to_string();
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        
        if let Some(gas) = gas_type {
            sql.push_str(" AND gas_type = ?");
            params.push(Box::new(gas.to_string()));
        }
        
        sql.push_str(" ORDER BY timestamp");
        
        let mut stmt = self.conn.prepare(&sql)?;
        let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
        
        let reading_iter = stmt.query_map(&*param_refs, |row| {
            Ok(SensorReading {
                id: Some(row.get(0)?),
                timestamp: row.get::<_, String>(1)?.parse().unwrap_or(Utc::now()),
                session_id: row.get(2)?,
                tgs2600: row.get(3)?,
                tgs2602: row.get(4)?,
                tgs2610: row.get(5)?,
                tgs2611: row.get(6)?,
                mq2: row.get(7)?,
                mq3: row.get(8)?,
                mq7: row.get(9)?,
                mq135: row.get(10)?,
                gas_type: row.get(11)?,
                concentration: row.get(12)?,
                temperature: row.get(13)?,
                humidity: row.get(14)?,
            })
        })?;
        
        let mut readings = Vec::new();
        for reading in reading_iter {
            readings.push(reading?);
        }
        
        Ok(readings)
    }
    
    pub fn get_recent_readings(&self, limit: usize) -> Result<Vec<SensorReading>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM sensor_readings ORDER BY timestamp DESC LIMIT ?"
        )?;
        
        let reading_iter = stmt.query_map([limit], |row| {
            Ok(SensorReading {
                id: Some(row.get(0)?),
                timestamp: row.get::<_, String>(1)?.parse().unwrap_or(Utc::now()),
                session_id: row.get(2)?,
                tgs2600: row.get(3)?,
                tgs2602: row.get(4)?,
                tgs2610: row.get(5)?,
                tgs2611: row.get(6)?,
                mq2: row.get(7)?,
                mq3: row.get(8)?,
                mq7: row.get(9)?,
                mq135: row.get(10)?,
                gas_type: row.get(11)?,
                concentration: row.get(12)?,
                temperature: row.get(13)?,
                humidity: row.get(14)?,
            })
        })?;
        
        let mut readings = Vec::new();
        for reading in reading_iter {
            readings.push(reading?);
        }
        
        Ok(readings)
    }
    
    pub fn save_model_performance(&self, performance: &ModelPerformance) -> Result<i64> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO model_performance 
             (model_name, timestamp, accuracy, precision, recall, f1_score, confusion_matrix, 
              training_samples, validation_samples)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)"
        )?;
        
        let id = stmt.insert(params![
            performance.model_name,
            performance.timestamp.format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
            performance.accuracy,
            performance.precision,
            performance.recall,
            performance.f1_score,
            performance.confusion_matrix,
            performance.training_samples,
            performance.validation_samples
        ])?;
        
        Ok(id)
    }
    
    pub fn create_training_session(&self, session: &TrainingSession) -> Result<i64> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO training_sessions 
             (session_id, gas_type, concentration, start_time, end_time, sample_count, notes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"
        )?;
        
        let id = stmt.insert(params![
            session.session_id,
            session.gas_type,
            session.concentration,
            session.start_time.format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
            session.end_time.map(|t| t.format("%Y-%m-%d %H:%M:%S%.3f").to_string()),
            session.sample_count,
            session.notes
        ])?;
        
        Ok(id)
    }
    
    pub fn update_training_session(&self, session_id: &str, end_time: DateTime<Utc>, sample_count: i32) -> Result<()> {
        let mut stmt = self.conn.prepare(
            "UPDATE training_sessions SET end_time = ?1, sample_count = ?2 WHERE session_id = ?3"
        )?;
        
        stmt.execute(params![
            end_time.format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
            sample_count,
            session_id
        ])?;
        
        Ok(())
    }
    
    pub fn get_training_sessions(&self) -> Result<Vec<TrainingSession>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM training_sessions ORDER BY start_time DESC"
        )?;
        
        let session_iter = stmt.query_map([], |row| {
            Ok(TrainingSession {
                id: Some(row.get(0)?),
                session_id: row.get(1)?,
                gas_type: row.get(2)?,
                concentration: row.get(3)?,
                start_time: row.get::<_, String>(4)?.parse().unwrap_or(Utc::now()),
                end_time: row.get::<_, Option<String>>(5)?.map(|s| s.parse().unwrap_or(Utc::now())),
                sample_count: row.get(6)?,
                notes: row.get(7)?,
            })
        })?;
        
        let mut sessions = Vec::new();
        for session in session_iter {
            sessions.push(session?);
        }
        
        Ok(sessions)
    }
    
    pub fn get_data_statistics(&self) -> Result<DataStatistics> {
        let mut stmt = self.conn.prepare(
            "SELECT 
                COUNT(*) as total_readings,
                COUNT(DISTINCT session_id) as total_sessions,
                COUNT(DISTINCT gas_type) as unique_gases,
                MIN(timestamp) as earliest_reading,
                MAX(timestamp) as latest_reading
             FROM sensor_readings"
        )?;
        
        let stats = stmt.query_row([], |row| {
            Ok(DataStatistics {
                total_readings: row.get(0)?,
                total_sessions: row.get(1)?,
                unique_gases: row.get(2)?,
                earliest_reading: row.get::<_, Option<String>>(3)?.map(|s| s.parse().unwrap_or(Utc::now())),
                latest_reading: row.get::<_, Option<String>>(4)?.map(|s| s.parse().unwrap_or(Utc::now())),
            })
        })?;
        
        Ok(stats)
    }
    
    pub fn cleanup_old_data(&self, days_to_keep: i64) -> Result<usize> {
        let cutoff_date = Utc::now() - chrono::Duration::days(days_to_keep);
        
        let mut stmt = self.conn.prepare(
            "DELETE FROM sensor_readings WHERE timestamp < ? AND gas_type IS NULL"
        )?;
        
        let deleted = stmt.execute([cutoff_date.format("%Y-%m-%d %H:%M:%S%.3f").to_string()])?;
        
        Ok(deleted)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataStatistics {
    pub total_readings: i64,
    pub total_sessions: i64,
    pub unique_gases: i64,
    pub earliest_reading: Option<DateTime<Utc>>,
    pub latest_reading: Option<DateTime<Utc>>,
}