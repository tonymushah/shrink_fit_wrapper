use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    hash::{BuildHasher, Hash},
    ops::{Deref, DerefMut},
    time::{Duration, Instant},
};

pub trait FitShrinkable {
    fn shrink_to_fit(&mut self);
}

impl<T> FitShrinkable for Vec<T> {
    fn shrink_to_fit(&mut self) {
        self.shrink_to_fit();
    }
}

impl<T> FitShrinkable for VecDeque<T> {
    fn shrink_to_fit(&mut self) {
        self.shrink_to_fit();
    }
}

impl<T, S> FitShrinkable for HashSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    fn shrink_to_fit(&mut self) {
        self.shrink_to_fit();
    }
}

impl<T> FitShrinkable for BinaryHeap<T>
where
    T: Ord,
{
    fn shrink_to_fit(&mut self) {
        self.shrink_to_fit();
    }
}

impl<K, V, S> FitShrinkable for HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    fn shrink_to_fit(&mut self) {
        self.shrink_to_fit();
    }
}

impl FitShrinkable for String {
    fn shrink_to_fit(&mut self) {
        self.shrink_to_fit();
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FitShrinkWrapper<S> {
    container: S,
    duration: Option<Duration>,
    last_shrink: Option<Instant>,
}

impl<S> FitShrinkWrapper<S> {
    pub fn new(container: S) -> Self {
        Self {
            container,
            duration: None,
            last_shrink: None,
        }
    }
    pub fn set_shrink_duration_cycle(mut self, cycle_duration: Duration) -> Self {
        self.duration = Some(cycle_duration);
        self
    }
    pub fn no_cycle(mut self) -> Self {
        self.duration = None;
        self
    }
    pub fn into_inner(self) -> S {
        self.container
    }
}

impl<S> FitShrinkWrapper<S>
where
    S: FitShrinkable,
{
    pub fn as_inner_mut(&mut self) -> FitShrinkWrapperMutGuard<'_, S> {
        FitShrinkWrapperMutGuard(self)
    }
}

impl<S> Deref for FitShrinkWrapper<S> {
    type Target = S;
    fn deref(&self) -> &Self::Target {
        &self.container
    }
}

#[derive(Debug)]
pub struct FitShrinkWrapperMutGuard<'a, S>(&'a mut FitShrinkWrapper<S>)
where
    S: FitShrinkable;

impl<'a, S> Deref for FitShrinkWrapperMutGuard<'a, S>
where
    S: FitShrinkable,
{
    type Target = S;
    fn deref(&self) -> &Self::Target {
        &self.0.container
    }
}

impl<'a, S> DerefMut for FitShrinkWrapperMutGuard<'a, S>
where
    S: FitShrinkable,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0.container
    }
}

impl<'a, S> Drop for FitShrinkWrapperMutGuard<'a, S>
where
    S: FitShrinkable,
{
    fn drop(&mut self) {
        if let Some(duration) = self.0.duration {
            if let Some(last_shrink) = self.0.last_shrink {
                if (Instant::now() - last_shrink) > duration {
                    self.shrink_to_fit();
                    self.0.last_shrink = Some(Instant::now());
                }
            } else {
                self.shrink_to_fit();
                self.0.last_shrink = Some(Instant::now());
            }
        } else {
            self.shrink_to_fit();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shrink_no_cycles() {}
}
