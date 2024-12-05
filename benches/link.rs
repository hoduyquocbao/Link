use criterion::{criterion_group, criterion_main, Criterion};
use link::core::link::{Link, Settings, Linkable, Movable};

async fn bench_link() {
    let mut link = Link::new(Settings::default());
    link.start().await.unwrap();
    link.stop().await.unwrap();
}

async fn bench_send() {
    let mut link = Link::new(Settings::default());
    link.start().await.unwrap();
    let data = vec![0u8; 1024];
    link.send(&data).await.unwrap();
    link.stop().await.unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("link", |b| {
        b.iter(|| {
            rt.block_on(async {
                bench_link().await;
            })
        })
    });

    c.bench_function("send", |b| {
        b.iter(|| {
            rt.block_on(async {
                bench_send().await;
            })
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches); 