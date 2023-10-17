use crate::use_spring_ref;
use dioxus::prelude::*;
use dioxus_signals::{use_signal, Signal};
use interpolation::Lerp;
use std::time::Duration;

pub fn use_spring_signal<T, V>(
    cx: Scope<T>,
    from: V,
    f: impl FnMut(V) + 'static,
) -> UseSpringSignal<V>
where
    V: PartialEq + Lerp<Scalar = f32> + Clone + 'static,
{
    let spring_ref = use_spring_ref(cx, from, f);
    to_owned![spring_ref];

    let signal: Signal<Option<(V, Option<Duration>)>> = use_signal(cx, || None);

    dioxus_signals::use_effect(cx, move || {
        if let Some((to, duration_cell)) = &*signal.read() {
            if let Some(duration) = duration_cell {
                spring_ref.animate(to.clone(), *duration);
            } else {
                spring_ref.set(to.clone());
            }
        }
    });

    UseSpringSignal { signal }
}

pub struct UseSpringSignal<V: 'static> {
    signal: Signal<Option<(V, Option<Duration>)>>,
}

impl<V> UseSpringSignal<V> {
    pub fn set(&self, to: V) {
        self.signal.set(Some((to, None)))
    }

    pub fn animate(&self, to: V, duration: Duration) {
        self.signal.set(Some((to, Some(duration))))
    }
}

impl<V> Clone for UseSpringSignal<V> {
    fn clone(&self) -> Self {
        Self {
            signal: self.signal.clone(),
        }
    }
}

impl<V> Copy for UseSpringSignal<V> {}
