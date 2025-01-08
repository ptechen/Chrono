use once_cell::sync::Lazy;
use std::future::Future;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::time::sleep;

pub static SERVER_CLOSE_TAG: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(true));

///执行一次
pub fn start_once<T>(delay_duration: Duration, task: T)
where
    T: Future + Send + 'static,
{
    tokio::spawn(async move {
        sleep(delay_duration).await;
        task.await;
    });
}

///重复执行
pub fn start_loop<F, O>(delay_duration: Duration, loop_duration: Duration, task_fn: F)
where
    F: Fn() -> O + Send + 'static + Copy,
    O: Future + Send,
    <O as Future>::Output: std::fmt::Debug,
{
    tokio::spawn(async move {
        while SERVER_CLOSE_TAG.load(Ordering::Relaxed) {
            if let Err(e) = tokio::spawn(async move {
                let mut sleep_time = true;
                while SERVER_CLOSE_TAG.load(Ordering::Relaxed) {
                    if sleep_time {
                        sleep(delay_duration).await;
                    } else {
                        sleep(loop_duration).await;
                    }
                    let fu = task_fn();
                    let v = format!("{:?}", fu.await);
                    sleep_time = v != "Ok(0)";
                }
            })
            .await
            {
                tracing::error!("{e}");
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });
}

// /// 每天重复执行
// pub fn every_day<F, O>(server_name: &str, task_id: u64, task_fn: F)
//     where
//         F: Fn() -> O + Send + 'static,
//         O: Future + Send,
//         <O as Future>::Output: std::fmt::Debug,
// {
//
//     tokio::spawn(async move {
//         while SERVER_CLOSE_TAG.load(Ordering::Relaxed) {
//             sleep(Duration::new(1, 0)).await;
//             let pre_time = std::fs::read_to_string(format!("task_{}", task_id)).unwrap_or_default();
//             let cur_time = Utc::now().format("%Y-%M-%D").to_string();
//             if NaiveDate::parse_from_str(&pre_time, "%Y-%M-%D") < NaiveDate::parse_from_str(&cur_time, "%Y-%M-%D") {
//
//             }
//             let fu = task_fn();
//             let v = format!("{:?}", fu.await);
//             if v == "Ok(0)" {
//                 sleep_time = false;
//             } else {
//                 sleep_time = true;
//             }
//         }
//     });
// }

///重复执行指定次数
pub fn start_loop_limit<F, O>(
    delay_duration: Duration,
    loop_duration: Duration,
    max_count: u64,
    task_fn: F,
) where
    F: Fn() -> O + Send + 'static,
    O: Future + Send,
{
    tokio::spawn(async move {
        let mut count = 0;
        let mut first_run = true;
        while SERVER_CLOSE_TAG.load(Ordering::Relaxed) {
            if count >= max_count {
                break;
            }
            if first_run {
                sleep(delay_duration).await;
                first_run = false;
            } else {
                sleep(loop_duration).await;
            }
            let fu = task_fn();
            fu.await;
            count += 1;
        }
    });
}

///重复执行指定次数， 当f返回true则提前终止任务
pub fn start_loop_until<F, O>(
    delay_duration: Duration,
    loop_duration: Duration,
    max_count: u64,
    task_fn: F,
) where
    F: Fn() -> O + Send + 'static,
    O: Future<Output = bool> + Send,
{
    tokio::spawn(async move {
        let mut count = 0;
        let mut first_run = true;
        while SERVER_CLOSE_TAG.load(Ordering::Relaxed) {
            if count >= max_count {
                break;
            }
            if first_run {
                sleep(delay_duration).await;
                first_run = false;
            } else {
                sleep(loop_duration).await;
            }
            let fu = task_fn();
            let succ = fu.await;
            if succ {
                break;
            }
            count += 1;
        }
    });
}
