

use crate::tests::core::{HttpTestHandler, Timer};


pub async fn http_test(host: &str) {

    let handler = HttpTestHandler::new(host);
    let mut timer = Timer::new();

    timer.execute(handler.ping_loop()).await;


}

