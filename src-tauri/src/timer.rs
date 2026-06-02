use crate::config::{AppConfig, SharedConfig};
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
    Break,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimerSnapshot {
    pub phase: TimerPhase,
    pub remaining_secs: u32,
    pub total_secs: u32,
    pub is_paused: bool,
}

#[derive(Debug)]
pub(crate) struct TimerInner {
    phase: TimerPhase,
    remaining_secs: u32,
    total_secs: u32,
    is_paused: bool,
}

impl Default for TimerInner {
    fn default() -> Self {
        Self {
            phase: TimerPhase::Idle,
            remaining_secs: 0,
            total_secs: 0,
            is_paused: false,
        }
    }
}

pub type SharedTimer = Arc<Mutex<TimerInner>>;

pub fn create_timer() -> SharedTimer {
    Arc::new(Mutex::new(TimerInner::default()))
}

fn read_config(config: &SharedConfig) -> AppConfig {
    config.lock().unwrap().clone()
}

pub fn snapshot(timer: &SharedTimer, _config: &SharedConfig) -> TimerSnapshot {
    let t = timer.lock().unwrap();
    TimerSnapshot {
        phase: t.phase,
        remaining_secs: t.remaining_secs,
        total_secs: t.total_secs,
        is_paused: t.is_paused,
    }
}

pub fn start_work(timer: &SharedTimer, config: &SharedConfig) -> Result<TimerSnapshot, String> {
    {
        let mut t = timer.lock().unwrap();

        if t.phase != TimerPhase::Idle {
            return Err("timer already running".into());
        }

        let cfg = read_config(config);
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

pub fn start_break(timer: &SharedTimer, config: &SharedConfig) -> Result<TimerSnapshot, String> {
    let cfg = read_config(config);
    {
        let mut t = timer.lock().unwrap();
        let secs = cfg.break_minutes * 60;
        t.phase = TimerPhase::Break;
        t.remaining_secs = secs;
        t.total_secs = secs;
        t.is_paused = false;
    }
    Ok(snapshot(timer, config))
}

pub fn skip_break(timer: &SharedTimer, config: &SharedConfig) -> Result<TimerSnapshot, String> {
    {
        let mut t = timer.lock().unwrap();
        t.phase = TimerPhase::Idle;
        t.remaining_secs = 0;
        t.total_secs = 0;
        t.is_paused = false;
    }
    Ok(snapshot(timer, config))
}

pub fn complete_work_cycle(timer: &SharedTimer, config: &SharedConfig) -> TimerSnapshot {
    {
        let mut t = timer.lock().unwrap();
        t.phase = TimerPhase::Idle;
        t.remaining_secs = 0;
        t.total_secs = 0;
        t.is_paused = false;
    }
    snapshot(timer, config)
}

pub fn spawn_timer_loop(app: AppHandle, timer: SharedTimer, config: SharedConfig) {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));

        let mut finished_work = false;
        let mut finished_break = false;

        {
            let mut t = timer.lock().unwrap();

            if t.is_paused || t.phase == TimerPhase::Idle {
                // idle or paused: no countdown
            } else if t.remaining_secs > 0 {
                t.remaining_secs -= 1;
            } else {
                match t.phase {
                    TimerPhase::Working => finished_work = true,
                    TimerPhase::Break => finished_break = true,
                    TimerPhase::Idle => {}
                }
            }
        }

        let snap = snapshot(&timer, &config);
        let _ = app.emit("timer-tick", &snap);

        if finished_work {
            let snap = complete_work_cycle(&timer, &config);
            let _ = app.emit("timer-tick", &snap);
            let _ = app.emit("timer-work-complete", ());
        }

        if finished_break {
            {
                let mut t = timer.lock().unwrap();
                t.phase = TimerPhase::Idle;
                t.remaining_secs = 0;
                t.total_secs = 0;
            }
            let _ = start_work(&timer, &config);
            let snap = snapshot(&timer, &config);
            let _ = app.emit("timer-tick", &snap);
            let _ = app.emit("timer-break-complete", ());
        }
    });
}
