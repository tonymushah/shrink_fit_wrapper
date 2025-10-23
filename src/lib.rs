/// An _extremely_ simple, "automatic" `shrink_to_fit` on any mut
use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    hash::{BuildHasher, Hash},
    ops::{Deref, DerefMut},
    time::{Duration, Instant},
};

/// Trait for types that can `shrink_to_fit`
pub trait ShrinkFitable {
    fn shrink_to_fit(&mut self);
}

impl<T> ShrinkFitable for Vec<T> {
    /// Call the [`Vec::shrink_to_fit`] method
    fn shrink_to_fit(&mut self) {
        self.shrink_to_fit();
    }
}

impl<T> ShrinkFitable for VecDeque<T> {
    /// Call the [`VecDeque::shrink_to_fit`] method
    fn shrink_to_fit(&mut self) {
        self.shrink_to_fit();
    }
}

impl<T, S> ShrinkFitable for HashSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    /// Call the [`HashSet::shrink_to_fit`] method
    fn shrink_to_fit(&mut self) {
        self.shrink_to_fit();
    }
}

impl<T> ShrinkFitable for BinaryHeap<T>
where
    T: Ord,
{
    /// Call the [`BinaryHeap::shrink_to_fit`] method
    fn shrink_to_fit(&mut self) {
        self.shrink_to_fit();
    }
}

impl<K, V, S> ShrinkFitable for HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    /// Call the [`HashMap::shrink_to_fit`] method
    fn shrink_to_fit(&mut self) {
        self.shrink_to_fit();
    }
}

impl ShrinkFitable for String {
    /// Call the [`String::shrink_to_fit`] method
    fn shrink_to_fit(&mut self) {
        self.shrink_to_fit();
    }
}

/// The [`ShrinkFitable`] wrapper
///
/// As you might notice, this structure only implement [`Deref`] of `S`.
/// If you want to borrow `S` mutably, use [`ShrinkFitWrapper::as_inner_mut`]
///
/// *Note*: This structure does not call `shrink_to_fit` once it is dropped, only [`ShrinkFitWrapperMutGuard`] does.
///
#[derive(Debug, Clone, Copy, Default)]
pub struct ShrinkFitWrapper<S> {
    container: S,
    duration: Option<Duration>,
    last_shrink: Option<Instant>,
}

impl<S> ShrinkFitWrapper<S> {
    /// create a new wrapper
    pub fn new(container: S) -> Self {
        Self {
            container,
            duration: None,
            last_shrink: None,
        }
    }
    /// Sets the period of time how we shrink the underlying container
    pub fn set_shrink_duration_cycle(mut self, cycle_duration: Duration) -> Self {
        self.duration = Some(cycle_duration);
        self.last_shrink = Some(Instant::now());
        self
    }
    /// Remove the period cycle
    pub fn no_cycle(mut self) -> Self {
        self.duration = None;
        self
    }
    /// Get the last time the underlying container was `shrink_to_fit`-ed
    pub fn last_shrink(&self) -> Option<Instant> {
        self.last_shrink
    }
    /// Get the underlying container
    ///
    /// *Note*: this function will not trigger `shrink_to_fit`
    pub fn into_inner(self) -> S {
        self.container
    }
}

impl<S> ShrinkFitWrapper<S>
where
    S: ShrinkFitable,
{
    /// Borrow the wrapper as mutable guard
    pub fn as_inner_mut(&mut self) -> ShrinkFitWrapperMutGuard<'_, S> {
        ShrinkFitWrapperMutGuard(Some(self))
    }
    /// Call [`ShrinkFitable::shrink_to_fit`] of the underlying container
    pub fn shrink_to_fit(&mut self) {
        self.container.shrink_to_fit();
    }
}

impl<S> Deref for ShrinkFitWrapper<S> {
    type Target = S;
    fn deref(&self) -> &Self::Target {
        &self.container
    }
}

/// A [`ShrinkFitWrapper`] guard that once dropped will trigger [`ShrinkFitable::shrink_to_fit`] of the wrapper container
#[derive(Debug)]
pub struct ShrinkFitWrapperMutGuard<'a, S>(Option<&'a mut ShrinkFitWrapper<S>>)
where
    S: ShrinkFitable;

impl<'a, S> ShrinkFitWrapperMutGuard<'a, S>
where
    S: ShrinkFitable,
{
    /// Drops the guard without triggering `shrink_to_fit`
    pub fn disarm(mut self) {
        self.0.take();
    }
}

impl<'a, S> Deref for ShrinkFitWrapperMutGuard<'a, S>
where
    S: ShrinkFitable,
{
    type Target = S;
    fn deref(&self) -> &Self::Target {
        &self.0.as_ref().unwrap().container
    }
}

impl<'a, S> DerefMut for ShrinkFitWrapperMutGuard<'a, S>
where
    S: ShrinkFitable,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0.as_mut().unwrap().container
    }
}

impl<'a, S> Drop for ShrinkFitWrapperMutGuard<'a, S>
where
    S: ShrinkFitable,
{
    fn drop(&mut self) {
        if let Some(inner) = self.0.as_mut() {
            if let Some(duration) = inner.duration {
                if let Some(last_shrink) = inner.last_shrink {
                    if (Instant::now() - last_shrink) > duration {
                        inner.shrink_to_fit();
                        inner.last_shrink = Some(Instant::now());
                    }
                } else {
                    inner.last_shrink = Some(Instant::now());
                }
            } else {
                inner.shrink_to_fit();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;

    use super::*;

    #[test]
    fn test_shrink_no_cycles() {
        let mut stack = ShrinkFitWrapper::new({
            let mut my_vec = Vec::with_capacity(10);
            my_vec.extend(0..3u32);
            my_vec
        });
        drop(stack.as_inner_mut());
        assert_eq!(stack.len(), stack.capacity());
    }
    #[test]
    fn test_shrink_with_cycles() {
        let mut stack = ShrinkFitWrapper::new({
            let mut my_vec = Vec::with_capacity(10);
            my_vec.extend(0..3u32);
            my_vec
        })
        .set_shrink_duration_cycle(Duration::from_secs(2));
        drop(stack.as_inner_mut());
        assert_ne!(stack.len(), stack.capacity());
        sleep(Duration::from_secs(3));
        drop(stack.as_inner_mut());
        assert_eq!(stack.len(), stack.capacity());
    }
}
