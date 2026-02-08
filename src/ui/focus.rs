use std::{collections::HashSet, iter::Cycle, slice};

type WidgetId = usize;

#[derive(Debug, Clone)]
struct FocusCycle<'a>(Cycle<slice::Iter<'a, WidgetId>>);

impl<'a> std::ops::Deref for FocusCycle<'a> {
    type Target = Cycle<slice::Iter<'a, WidgetId>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> std::ops::DerefMut for FocusCycle<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for FocusCycle<'_> {
    fn default() -> Self {
        Self([0].iter().cycle())
    }
}

impl<'a> FocusCycle<'a> {
    fn new(ids: &'a [WidgetId]) -> Self {
        Self(ids.iter().cycle())
    }
}

#[derive(Debug, Default, Clone)]
pub struct Focus<'a> {
    cycle: FocusCycle<'a>,
}

impl<'a> Focus<'a> {
    pub fn new(ids: &'a [WidgetId]) -> Self {
        let cycle = FocusCycle::new(ids);
        Self { cycle }
    }

    pub fn set_elements(&mut self, ids: &'a [WidgetId]) {
        self.cycle = FocusCycle::new(ids);
    }
}

impl<'a> Iterator for Focus<'a> {
    type Item = WidgetId;

    fn next(&mut self) -> Option<Self::Item> {
        self.cycle.next().copied()
    }
}

pub trait Focusable {
    fn id(&self) -> WidgetId;
    fn set_id(&mut self, id: WidgetId);
}

#[macro_export]
macro_rules! set_elements {
    ($focus:expr, $($id:ident),*) => {
        let mut __idx: usize = 0;

        $(
            $id.set_id(__idx);
            __idx += 1;
        )*
        $focus.set_elements(&[$(< $id as Focusable>::id()),*]);
    }
}
