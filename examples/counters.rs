#![allow(unused)]

use leptos_reactive::{
    create_runtime, create_scope, create_signal, provide_context, ReadSignal, Scope, SignalGet,
    SignalUpdate, WriteSignal,
};
use leptos_tracked::{AddAssign, Extend, TrackedVec};

const MANY_COUNTERS: usize = 1000;

type CounterHolder = Vec<(usize, (ReadSignal<i32>, WriteSignal<i32>))>;

#[derive(Copy, Clone)]
struct CounterUpdater {
    set_counters: WriteSignal<CounterHolder>,
}

fn with_helpers(cx: Scope) {
    let (next_counter_id, set_next_counter_id) = create_signal(cx, 0);
    let (counters, set_counters) = create_signal::<CounterHolder>(cx, vec![]);
    provide_context(cx, CounterUpdater { set_counters });

    let add_counter = move || {
        let id = next_counter_id();
        let sig = create_signal(cx, 0);
        set_counters.tracked_push((id, sig));
        set_next_counter_id.tracked_add(1);
    };

    let add_many_counters = move || {
        let next_id = next_counter_id();
        let new_counters = (next_id..next_id + MANY_COUNTERS).map(|id| {
            let signal = create_signal(cx, 0);
            (id, signal)
        });

        set_counters.tracked_extend(new_counters);
        set_next_counter_id.tracked_add(MANY_COUNTERS);
    };

    let clear_counters = move || {
        set_counters.tracked_clear();
    };
}

fn without_helpers(cx: Scope) {
    let (next_counter_id, set_next_counter_id) = create_signal(cx, 0);
    let (counters, set_counters) = create_signal::<CounterHolder>(cx, vec![]);
    provide_context(cx, CounterUpdater { set_counters });

    let add_counter = move || {
        let id = next_counter_id();
        let sig = create_signal(cx, 0);
        set_counters.update(move |counters| counters.push((id, sig)));
        set_next_counter_id.update(|id| *id += 1);
    };

    let add_many_counters = move || {
        let next_id = next_counter_id();
        let new_counters = (next_id..next_id + MANY_COUNTERS).map(|id| {
            let signal = create_signal(cx, 0);
            (id, signal)
        });

        set_counters.update(move |counters| counters.extend(new_counters));
        set_next_counter_id.update(|id| *id += MANY_COUNTERS);
    };

    let clear_counters = move || {
        set_counters.update(|counters| counters.clear());
    };
}

fn main() {
    create_scope(create_runtime(), |cx| {
        without_helpers(cx);
        with_helpers(cx);
    })
    .dispose();
}
