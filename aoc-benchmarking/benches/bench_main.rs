use criterion::criterion_main;

use aoc_benchmarking::aoc_benches;
use historian_hysteria::HistorianHysteria;
use mull_it_over::MullItOver;
use red_nosed_reports::RedNosedReports;
// import_marker

criterion_main! {
    benches
}

aoc_benches! {
    5,
    (
        day_001,
        "../day-001-historian-hysteria/input.txt",
        HistorianHysteria,
        "Combined (including parsing)"
    ),
    (
        day_002,
        "../day-002-red-nosed-reports/input.txt",
        RedNosedReports,
        "Part 1",
        "Part 2"
    ),
    (
        day_003,
        "../day-003-mull-it-over/input.txt",
        MullItOver,
        "Part 1",
        "Part 2"
    ),
    // bench_marker
}
