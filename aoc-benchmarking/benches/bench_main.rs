use criterion::criterion_main;

use aoc_benchmarking::aoc_benches;
use ceres_search::CeresSearch;
use historian_hysteria::HistorianHysteria;
use mull_it_over::MullItOver;
use print_queue::PrintQueue;
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
    (
        day_004,
        "../day-004-ceres-search/input.txt",
        CeresSearch,
        "Part 1",
        "Part 2"
    ),
    (
        day_005,
        "../day-005-print-queue/input.txt",
        PrintQueue,
        "Combined (including parsing)"
    ),
    // bench_marker
}
