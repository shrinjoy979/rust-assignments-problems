use r#async::hard::rate_limiter::RateLimiter;
use std::time::Instant;

#[tokio::test]
async fn test_limiter_permits() {
    let limiter = RateLimiter::new(2);
    limiter.acquire().await;
    limiter.acquire().await;
    // Third acquire should block if we don't drop the permits, but for this
    // mock we just check that we can acquire.
    assert_eq!(limiter.semaphore.available_permits(), 0);
}

#[tokio::test]
async fn test_new() {
    let limiter = RateLimiter::new(5);
    assert_eq!(limiter.semaphore.available_permits(), 5);
}
