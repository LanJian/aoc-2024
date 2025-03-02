use criterion::criterion_main;

use aoc_benchmarking::aoc_benches;
use bridge_repair::BridgeRepair;
use ceres_search::CeresSearch;
use chronospatial_computer::ChronospatialComputer;
use claw_contraption::ClawContraption;
use disk_fragmenter::DiskFragmenter;
use garden_groups::GardenGroups;
use guard_gallivant::GuardGallivant;
use historian_hysteria::HistorianHysteria;
use hoof_it::HoofIt;
use linen_layout::LinenLayout;
use mull_it_over::MullItOver;
use plutonian_pebbles::PlutonianPebbles;
use print_queue::PrintQueue;
use race_condition::RaceCondition;
use ram_run::RamRun;
use red_nosed_reports::RedNosedReports;
use reindeer_maze::ReindeerMaze;
use resonant_collinearity::ResonantCollinearity;
use restroom_redoubt::RestroomRedoubt;
use warehouse_woes::WarehouseWoes;
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
    (
        day_006,
        "../day-006-guard-gallivant/input.txt",
        GuardGallivant,
        "Part 1",
        "Part 2"
    ),
    (
        day_007,
        "../day-007-bridge-repair/input.txt",
        BridgeRepair,
        "Part 1",
        "Part 2"
    ),
    (
        day_008,
        "../day-008-resonant-collinearity/input.txt",
        ResonantCollinearity,
        "Part 1",
        "Part 2"
    ),
    (
        day_009,
        "../day-009-disk-fragmenter/input.txt",
        DiskFragmenter,
        "Part 1",
        "Part 2"
    ),
    (
        day_010,
        "../day-010-hoof-it/input.txt",
        HoofIt,
        "Part 1",
        "Part 2"
    ),
    (
        day_011,
        "../day-011-plutonian-pebbles/input.txt",
        PlutonianPebbles,
        "Part 1",
        "Part 2"
    ),
    (
        day_012,
        "../day-012-garden-groups/input.txt",
        GardenGroups,
        "Part 1",
        "Part 2"
    ),
    (
        day_013,
        "../day-013-claw-contraption/input.txt",
        ClawContraption,
        "Part 1",
        "Part 2"
    ),
    (
        day_014,
        "../day-014-restroom-redoubt/input.txt",
        RestroomRedoubt,
        "Part 1",
        "Part 2"
    ),
    (
        day_015,
        "../day-015-warehouse-woes/input.txt",
        WarehouseWoes,
        "Combined (including parsing)"
    ),
    (
        day_016,
        "../day-016-reindeer-maze/input.txt",
        ReindeerMaze,
        "Combined (including parsing)"
    ),
    (
        day_017,
        "../day-017-chronospatial-computer/input.txt",
        ChronospatialComputer,
        "Part 1",
        "Part 2"
    ),
    (
        day_018,
        "../day-018-ram-run/input.txt",
        RamRun,
        "Part 1",
        "Part 2"
    ),
    (
        day_019,
        "../day-019-linen-layout/input.txt",
        LinenLayout,
        "Combined (including parsing)"
    ),
    (
        day_020,
        "../day-020-race-condition/input.txt",
        RaceCondition,
        "Combined (including parsing)"
    ),
    // bench_marker
}
