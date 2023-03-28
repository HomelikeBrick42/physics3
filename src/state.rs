use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

use crate::{Circle, Number};
use rayon::prelude::*;

#[derive(Default)]
pub struct PhysicsState {
    circles: Vec<Circle>,
    old_circles: Vec<Circle>,
}

impl PhysicsState {
    pub fn add_circle(&mut self, circle: Circle) {
        self.circles.push(circle);
    }

    pub fn update(&mut self, ts: Number) {
        let solved = AtomicBool::new(false);
        while !solved.load(Relaxed) {
            solved.store(true, Relaxed);

            std::mem::swap(&mut self.circles, &mut self.old_circles);
            self.circles.clear();
            self.circles
                .par_extend(self.old_circles.par_iter().copied().enumerate().map(
                    |(circle_index, circle)| {
                        _ = circle_index;
                        circle
                    },
                ));
        }
        self.circles
            .par_iter_mut()
            .for_each(|circle| circle.position += circle.velocity * ts);
    }

    pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> <&mut Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}

impl IntoIterator for PhysicsState {
    type Item = Circle;

    type IntoIter = std::vec::IntoIter<Circle>;

    fn into_iter(self) -> Self::IntoIter {
        self.circles.into_iter()
    }
}

impl<'a> IntoIterator for &'a PhysicsState {
    type Item = &'a Circle;

    type IntoIter = std::slice::Iter<'a, Circle>;

    fn into_iter(self) -> Self::IntoIter {
        self.circles.iter()
    }
}

impl<'a> IntoIterator for &'a mut PhysicsState {
    type Item = &'a mut Circle;

    type IntoIter = std::slice::IterMut<'a, Circle>;

    fn into_iter(self) -> Self::IntoIter {
        self.circles.iter_mut()
    }
}

impl FromIterator<Circle> for PhysicsState {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Circle>,
    {
        Self {
            circles: Vec::from_iter(iter),
            old_circles: vec![],
        }
    }
}
