use crate::config::{AppConfig, SharedConfig};
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TimerPhase {
    Idle,
    Working,
    ShortBreak,
    LongBreak,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimerSnapshot {
    pub phase: TimerPhase,
    pub remaining_secs: u32,
    pub total_secs: u32,
    pub is_paused: bool,
    pub sessions_completed_today: u32,
    pub sessions_in_cycle: u32,
    pub daily_goal_count: u32,
    pub daily_focus_goal: bool,
    pub strict_break_mode: bool,
}

#[derive(Debug)]
pub(crate) struct TimerInner {
    phase: TimerPhase,
    remaining_secs: u32,
    total_secs: u32,
    is_paused: bool,
    sessions_completed_today: u32,
    sessions_in_cycle: u32,
    last_date: String,
}

impl Default for TimerInner {
    fn default() -> Self {
        Self {
            phase: TimerPhase::Idle,
            remaining_secs: 0,
            total_secs: 0,
            is_paused: false,
            sessions_completed_today: 0,
            sessions_in_cycle: 0,
            last_date: today_string(),
        }
    }
}

pub type SharedTimer = Arc<Mutex<TimerInner>>;

pub fn create_timer() -> SharedTimer {
    Arc::new(Mutex::new(TimerInner::default()))
}

fn today_string() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

fn reset_daily_if_needed(timer: &mut TimerInner) {
    let today = today_string();
    if timer.last_date != today {
        timer.last_date = today;
        timer.sessions_completed_today = 0;
        timer.sessions_in_cycle = 0;
    }
}

fn read_config(config: &SharedConfig) -> AppConfig {
    config.lock().unwrap().clone()
}

pub fn snapshot(timer: &SharedTimer, config: &SharedConfig) -> TimerSnapshot {
    let t = timer.lock().unwrap();
    let cfg = config.lock().unwrap();
    TimerSnapshot {
        phase: t.phase,
        remaining_secs: t.remaining_secs,
        total_secs: t.total_secs,
        is_paused: t.is_paused,
        sessions_completed_today: t.sessions_completed_today,
        sessions_in_cycle: t.sessions_in_cycle,
        daily_goal_count: cfg.daily_goal_count,
        daily_focus_goal: cfg.daily_focus_goal,
        strict_break_mode: cfg.strict_break_mode,
    }
}

pub fn start_work(timer: &SharedTimer, config: &SharedConfig) -> Result<TimerSnapshot, String> {
    let cfg = read_config(config);
    {
        let mut t = timer.lock().unwrap();
        reset_daily_if_needed(&mut t);

        if t.phase != TimerPhase::Idle {
            return Err("timer already running".into());
        }

        let secs = cfg.work_minutes * 60;
        t.phase = TimerPhase::Working;
        t.remaining_secs = secs;
        t.total_secs = secs;
        t.is_paused = false;
    }
    Ok(snapshot(timer, config))
}

pub fn end_work(timer: &SharedTimer, config: &SharedConfig) -> TimerSnapshot {
    {
        let mut t = timer.lock().unwrap();
        t.phase = TimerPhase::Idle;
        t.remaining_secs = 0;
        t.total_secs = 0;
        t.is_paused = false;
    }
    snapshot(timer, config)
}

pub fn start_break(
    timer: &SharedTimer,
    config: &SharedConfig,
    long: bool,
) -> Result<TimerSnapshot, String> {
    let cfg = read_config(config);
    {
        let mut t = timer.lock().unwrap();

        let (phase, mins) = if long && cfg.long_break_mode {
            (TimerPhase::LongBreak, cfg.long_break_minutes)
        } else {
            (TimerPhase::ShortBreak, cfg.short_break_minutes)
        };

        let secs = mins * 60;
        t.phase = phase;
        t.remaining_secs = secs;
        t.total_secs = secs;
        t.is_paused = false;
    }
    Ok(snapshot(timer, config))
}

pub fn skip_break(timer: &SharedTimer, config: &SharedConfig) -> Result<TimerSnapshot, String> {
    let cfg = read_config(config);
    {
        let mut t = timer.lock().unwrap();

        if cfg.strict_break_mode
            && (t.phase == TimerPhase::ShortBreak || t.phase == TimerPhase::LongBreak)
        {
            return Err("cannot skip break in strict mode".into());
        }

        t.phase = TimerPhase::Idle;
        t.remaining_secs = 0;
        t.total_secs = 0;
        t.is_paused = false;
    }
    Ok(snapshot(timer, config))
}

pub fn complete_work_cycle(timer: &SharedTimer, config: &SharedConfig) -> (TimerSnapshot, bool) {
    let cfg = read_config(config);
    let use_long = {
        let mut t = timer.lock().unwrap();
        reset_daily_if_needed(&mut t);

        t.sessions_completed_today += 1;
        t.sessions_in_cycle += 1;

        let use_long = cfg.long_break_mode && t.sessions_in_cycle >= cfg.long_break_interval;
        if use_long {
            t.sessions_in_cycle = 0;
        }

        t.phase = TimerPhase::Idle;
        t.remaining_secs = 0;
        t.total_secs = 0;
        t.is_paused = false;
        use_long
    };
    (snapshot(timer, config), use_long)
}

pub fn spawn_timer_loop(app: AppHandle, timer: SharedTimer, config: SharedConfig) {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));

        let mut finished_work = false;
        let mut finished_break = false;

        {
            let mut t = timer.lock().unwrap();
            reset_daily_if_needed(&mut t);

            if t.is_paused || t.phase == TimerPhase::Idle {
                // idle or paused: no countdown
            } else if t.remaining_secs > 0 {
                t.remaining_secs -= 1;
            } else {
                match t.phase {
                    TimerPhase::Working => {
                        finished_work = true;
                    }
                    TimerPhase::ShortBreak | TimerPhase::LongBreak => {
                        finished_break = true;
                    }
                    TimerPhase::Idle => {}
                }
            }
        }

        let snap = snapshot(&timer, &config);
        let _ = app.emit("timer-tick", &snap);

        if finished_work {
            let (snap, use_long) = complete_work_cycle(&timer, &config);
            let _ = app.emit("timer-tick", &snap);
            let _ = app.emit("timer-work-complete", serde_json::json!({ "useLongBreak": use_long }));
        }

        if finished_break {
            {
                let mut t = timer.lock().unwrap();
                t.phase = TimerPhase::Idle;
                t.remaining_secs = 0;
                t.total_secs = 0;
            }
            let snap = snapshot(&timer, &config);
            let _ = app.emit("timer-tick", &snap);
            let _ = app.emit("timer-break-complete", ());
        }
    });
}
