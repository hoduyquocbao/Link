use criterion::{criterion_group, criterion_main, Criterion};
use link::core::link::{Settings, Movable};
use link::net::{Socket, Group};

async fn bench_group_load() {
    let addr = "127.0.0.1:8080";
    let settings = Settings::default();
    let mut group = Group::new(10, settings.clone());
    
    for _ in 0..10 {
        let socket = Socket::connect(addr, settings.clone()).await.unwrap();
        group.add(socket).await.unwrap();
    }

    for _ in 0..100 {
        let mut holder = group.get().await.unwrap();
        let data = vec![0u8; 1024];
        holder.socket().send(&data).await.unwrap();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("group_load", |b| {
        b.iter(|| {
            rt.block_on(async {
                bench_group_load().await;
            })
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches); 