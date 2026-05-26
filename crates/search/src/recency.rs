use chrono::{DateTime, Utc};

/// Recency bonus, 14-day half-life exponential decay.
///
/// `last_used == now` → 100.0. After 14 days → ~37.0 (100/e). After 60 days → ~1.4.
/// `None` → 0.0. Future-dated stamps (clock skew or hand-edited frontmatter) are
/// clamped to 100.0 rather than producing a runaway score.
pub fn recency_bonus(now: DateTime<Utc>, last_used: Option<DateTime<Utc>>) -> f64 {
    let Some(ts) = last_used else {
        return 0.0;
    };
    let delta = now - ts;
    let days = delta.num_seconds() as f64 / 86_400.0;
    if days < 0.0 {
        return 100.0;
    }
    100.0 * (-days / 14.0).exp()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn none_returns_zero() {
        let b = recency_bonus(Utc::now(), None);
        assert!(b.abs() < f64::EPSILON);
    }

    #[test]
    fn now_returns_roughly_hundred() {
        let now = Utc::now();
        let b = recency_bonus(now, Some(now));
        assert!((b - 100.0).abs() < 0.01);
    }

    #[test]
    fn fourteen_days_old_decays_to_roughly_thirty_seven() {
        let now = Utc::now();
        let past = now - Duration::days(14);
        let b = recency_bonus(now, Some(past));
        assert!((b - 100.0 / std::f64::consts::E).abs() < 0.1, "got {b}");
    }

    #[test]
    fn future_dated_returns_hundred_not_panic() {
        let now = Utc::now();
        let future = now + Duration::days(1);
        let b = recency_bonus(now, Some(future));
        assert!(b >= 99.0);
    }
}
