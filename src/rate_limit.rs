use core::num::NonZeroU32;
use std::sync::RwLock;

use governor::clock;
use governor::middleware::NoOpMiddleware;
use governor::state::InMemoryState;
use governor::state::NotKeyed;
use governor::Quota;
use governor::RateLimiter;
use once_cell::sync::Lazy;

/// The rate limiter of the API. By default, it has 5 "Cells", and replenish 1 per second in accordance to the MB API guidelines.
///
/// This allows "bursts" of 5 requests before limiting yourself to the API's classic rate.
/// So you may keep it in mind when designing your apps that you have 5 "free" requests
///
/// This is behind a RwLock, which allows for your own ratelimits. The rate limit will only change after all requests waiting for ratelimits are executed,
/// so prefer changing it in `main()` or somewhere before requests are executed
pub static MB_RATE_LIMITER: Lazy<
    RwLock<RateLimiter<NotKeyed, InMemoryState, clock::DefaultClock, NoOpMiddleware>>,
> = Lazy::new(|| {
    let quota =
        Quota::per_second(NonZeroU32::new(1).unwrap()).allow_burst(NonZeroU32::new(5).unwrap());
    RwLock::new(RateLimiter::direct(quota))
});

/// Wait for a spot in MB's ratelimit
#[allow(clippy::await_holding_lock)]
pub async fn wait_for_mb_ratelimit() {
    //This is fine. Setting the rate limit shouldn't be done at high frequencies, and even if we change it, it's better to finish the requests first
    let rate_limiter = MB_RATE_LIMITER.read().expect("Poisoned ratelimiter!");

    rate_limiter.until_ready().await;
}
