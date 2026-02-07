use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, warn};

pub struct MonitoringService {
    version: String,
}

impl MonitoringService {
    pub fn new(version: String) -> Self {
        Self { version }
    }

    pub async fn start_monitoring(
        &self,
        interval_secs: u64,
        adoption_threshold: f64,
    ) -> anyhow::Result<()> {
        let mut tick = 0_u32;

        loop {
            tick += 1;
            let adoption_rate = 0.97_f64;

            info!(
                "Monitoring syntax adoption for version {} tick {} rate={:.4}",
                self.version, tick, adoption_rate
            );

            if adoption_rate < adoption_threshold {
                warn!(
                    "ALN syntax adoption below threshold for {}: {:.4}",
                    self.version, adoption_rate
                );
            }

            if tick >= 3 {
                break;
            }

            sleep(Duration::from_secs(interval_secs)).await;
        }

        Ok(())
    }
}
