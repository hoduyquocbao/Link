use criterion::{criterion_group, criterion_main, Criterion};
use link::core::link::{Settings, Linkable, Movable};
use link::net::Socket;

async fn bench_socket_send() {
    let addr = "127.0.0.1:8080";
    let settings = Settings::default();
    let mut socket = Socket::connect(addr, settings).await.unwrap();
    
    let data = vec![0u8; 1024];
    socket.send(&data).await.unwrap();
    socket.stop().await.unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("socket_send", |b| {
        b.iter(|| {
            rt.block_on(async {
                bench_socket_send().await;
            })
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches); 