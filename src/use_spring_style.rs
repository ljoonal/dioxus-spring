use crate::{
    use_spring_ref,
    use_spring_signal::{use_spring_signal, UseSpringSignal},
    UseSpringRef,
};
use dioxus::prelude::*;
use interpolation::Lerp;
use std::{rc::Rc, time::Duration};

pub fn use_spring_style<T, V>(
    cx: Scope<T>,
    from: V,
    mut make_style: impl FnMut(V) -> String + 'static,
) -> &UseSpringStyle<V>
where
    V: PartialEq + Lerp<Scalar = f32> + Clone + 'static,
{
    let element_ref = use_ref(cx, || None);

    let from_clone = from.clone();
    use_effect(cx, &element_ref.read().is_some(), |_| {
        set_style(element_ref, &make_style(from_clone));
        async {}
    });

    let element_ref_clone = element_ref.clone();
    let spring_ref = use_spring_signal(cx, from, move |val| {
        set_style(&element_ref_clone, &make_style(val));
    });

    cx.bump().alloc(UseSpringStyle {
        element_ref: element_ref.clone(),
        spring_ref,
    })
}

pub struct UseSpringStyle<V: 'static> {
    element_ref: UseRef<Option<Rc<MountedData>>>,
    spring_ref: UseSpringSignal<V>,
}

impl<V> UseSpringStyle<V> {
    pub fn mount(&self, data: Rc<MountedData>) {
        self.element_ref.set(Some(data));
    }

    pub fn set(&self, to: V) {
        self.spring_ref.set(to)
    }

    pub fn animate(&self, to: V, duration: Duration) {
        self.spring_ref.animate(to, duration);
    }
}

impl<V> Clone for UseSpringStyle<V> {
    fn clone(&self) -> Self {
        Self {
            element_ref: self.element_ref.clone(),
            spring_ref: self.spring_ref.clone(),
        }
    }
}

fn set_style(element_ref: &UseRef<Option<Rc<MountedData>>>, style: &str) {
    if let Some(element) = &*element_ref.read() {
        let raw_elem = element
            .get_raw_element()
            .unwrap()
            .downcast_ref::<web_sys::Element>()
            .unwrap();

        raw_elem.set_attribute("style", style).unwrap();
    }
}
