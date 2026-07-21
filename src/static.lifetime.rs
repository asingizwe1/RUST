/*
Since a spawned thread can:
outlive the thread that spawned it (its parent thread)
run until the program exits
it must not borrow any values that might be dropped before the program exits; violating this constraint would expose us to a use-after-free bug.
That's why std::thread::spawn's signature requires that the closure passed to it has the 'static lifetime:
*/