use zero_inbox::{parse_file, ir, optimizer, scheduler, config::Config};

#[test]
fn test_schedule_respects_time_windows() {
    let ast = parse_file(std::path::Path::new("examples/daily_plan.zbx")).unwrap();
    let metadata = Config::default().to_ir_metadata().unwrap();
    let mut ir = ir::to_ir(&ast, metadata);
    optimizer::optimize(&mut ir);
    let sched = scheduler::create_scheduler(scheduler::ScheduleMode::Naive, None);
    sched.schedule(&mut ir);
    for block in &ir.blocks {
        for task in &block.tasks {
            if let (Some(start), Some(end)) = (task.scheduled_start, task.scheduled_end) {
                assert!(start < end, "Task start must be before end");
            }
        }
    }
}
