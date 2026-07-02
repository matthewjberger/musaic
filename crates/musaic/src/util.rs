use leptos::prelude::*;

/// The half-open range of item indices to render for a virtualized, fixed-row-height
/// list, given the current scroll offset and viewport. Shared by every windowed view
/// (`VirtualList`, `Table`, `CodeSurface`, `MultiEditor`).
pub fn visible_range(
    scroll_top: f64,
    viewport_height: f64,
    item_height: f64,
    overscan: usize,
    total: usize,
) -> (usize, usize) {
    let view = viewport_height.max(item_height);
    let first = ((scroll_top / item_height).floor() as usize).saturating_sub(overscan);
    let count = (view / item_height).ceil() as usize + overscan * 2 + 1;
    let start = first.min(total);
    let end = (start + count).min(total);
    (start, end)
}

/// A per-owner store that keeps values (typically `Closure`s) alive and drops them
/// on cleanup. Use this instead of `Closure::forget` so one-shot JS callbacks are
/// reclaimed when the component unmounts. Create it once at component init with
/// [`use_retained_closures`], then `retain` from any later handler; the handle is
/// `Copy`, so callbacks can capture it freely.
#[derive(Clone, Copy)]
pub struct RetainedClosures(StoredValue<Vec<Box<dyn std::any::Any>>, LocalStorage>);

impl RetainedClosures {
    pub fn retain<T: 'static>(&self, value: T) {
        self.0.update_value(|held| held.push(Box::new(value)));
    }
}

/// Creates an owner-scoped [`RetainedClosures`] store. Call this in a component body.
pub fn use_retained_closures() -> RetainedClosures {
    RetainedClosures(StoredValue::new_local(Vec::new()))
}
