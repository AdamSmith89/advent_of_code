use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::{Enumerate, StepBy};
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};

use itertools::Itertools;

// Wrapper around grid::Grid that provides extended functionality
#[derive(Clone, Eq, PartialEq)]
pub struct Grid<T: std::cmp::Eq> {
    inner: grid::Grid<T>,
}

// Extension methods
impl<T: Copy + Default + From<char> + std::cmp::Eq> Grid<T> {
    pub fn enum_row(&self, row: usize) -> Enumerate<StepBy<Iter<'_, T>>> {
        self.inner.iter_row(row).enumerate()
    }

    pub fn from_str_with_order(s: &str, order: grid::Order) -> Self {
        let mut inner = grid::Grid::new_with_order(0, 0, order);

        for line in s.lines() {
            inner.push_row(line.chars().map_into().collect());
        }

        Self { inner }
    }

    pub fn swap(&mut self, x: (usize, usize), y: (usize, usize)) -> color_eyre::Result<()> {
        let temp = self.inner[x];
        self.inner[x] = self.inner[y];
        self.inner[y] = temp;
        Ok(())
    }
}

// grid::Grid interface
impl<T: Clone + Default + std::cmp::Eq> Grid<T> {
    pub fn clear(&mut self) {
        self.inner.clear()
    }

    pub fn cols(&self) -> usize {
        self.inner.cols()
    }

    pub fn fill(&mut self, value: T) {
        self.inner.fill(value)
    }

    pub fn fill_with<F>(&mut self, f: F)
    where
        F: FnMut() -> T,
    {
        self.inner.fill_with(f)
    }

    pub fn flatten(&self) -> &Vec<T> {
        self.inner.flatten()
    }

    pub fn flip_cols(&mut self) {
        self.inner.flip_cols()
    }

    pub fn flip_rows(&mut self) {
        self.inner.flip_rows()
    }

    pub fn from_vec(vec: Vec<T>, cols: usize) -> Self {
        Self {
            inner: grid::Grid::from_vec(vec, cols),
        }
    }

    pub fn from_vec_with_order(vec: Vec<T>, cols: usize, order: grid::Order) -> Self {
        Self {
            inner: grid::Grid::from_vec_with_order(vec, cols, order),
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.inner.get(row, col)
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        self.inner.get_mut(row, col)
    }

    pub unsafe fn get_unchecked(&self, row: usize, col: usize) -> &T {
        self.inner.get_unchecked(row, col)
    }

    pub unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut T {
        self.inner.get_unchecked_mut(row, col)
    }

    pub fn indexed_iter(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        self.inner.indexed_iter()
    }

    pub fn init(rows: usize, cols: usize, data: T) -> Self {
        Self {
            inner: grid::Grid::init(rows, cols, data),
        }
    }

    pub fn init_with_order(rows: usize, cols: usize, order: grid::Order, data: T) -> Self
    where
        T: Clone,
    {
        Self {
            inner: grid::Grid::init_with_order(rows, cols, order, data),
        }
    }

    pub fn insert_col(&mut self, index: usize, col: Vec<T>) {
        self.inner.insert_col(index, col)
    }

    pub fn insert_row(&mut self, index: usize, row: Vec<T>) {
        self.inner.insert_row(index, row)
    }

    pub fn into_vec(self) -> Vec<T> {
        self.inner.into_vec()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn iter(&self) -> Iter<T> {
        self.inner.iter()
    }

    pub fn iter_col(&self, col: usize) -> StepBy<Iter<T>> {
        self.inner.iter_col(col)
    }

    pub fn iter_col_mut(&mut self, col: usize) -> StepBy<IterMut<T>> {
        self.inner.iter_col_mut(col)
    }

    pub fn iter_cols(&self) -> grid::GridColIter<'_, T> {
        self.inner.iter_cols()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.inner.iter_mut()
    }

    pub fn iter_row(&self, row: usize) -> StepBy<Iter<T>> {
        self.inner.iter_row(row)
    }

    pub fn iter_row_mut(&mut self, row: usize) -> StepBy<IterMut<T>> {
        self.inner.iter_row_mut(row)
    }

    pub fn iter_rows(&self) -> grid::GridRowIter<'_, T> {
        self.inner.iter_rows()
    }

    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            inner: grid::Grid::<T>::new(rows, cols),
        }
    }

    pub fn new_with_order(rows: usize, cols: usize, order: grid::Order) -> Self {
        Self {
            inner: grid::Grid::<T>::new_with_order(rows, cols, order),
        }
    }

    pub fn order(&self) -> grid::Order {
        self.inner.order()
    }

    pub fn pop_col(&mut self) -> Option<Vec<T>> {
        self.inner.pop_col()
    }

    pub fn pop_row(&mut self) -> Option<Vec<T>> {
        self.inner.pop_row()
    }

    pub fn push_col(&mut self, col: Vec<T>) {
        self.inner.push_col(col)
    }

    pub fn push_row(&mut self, row: Vec<T>) {
        self.inner.push_row(row)
    }

    pub fn remove_col(&mut self, col_index: usize) -> Option<Vec<T>> {
        self.inner.remove_col(col_index)
    }

    pub fn remove_row(&mut self, row_index: usize) -> Option<Vec<T>> {
        self.inner.remove_row(row_index)
    }

    pub fn rotate_half(&mut self) {
        self.inner.rotate_half()
    }

    pub fn rotate_left(&mut self) {
        self.inner.rotate_left()
    }

    pub fn rotate_right(&mut self) {
        self.inner.rotate_right()
    }

    pub fn rows(&self) -> usize {
        self.inner.rows()
    }

    pub fn size(&self) -> (usize, usize) {
        self.inner.size()
    }

    pub fn transpose(&mut self) {
        self.inner.transpose()
    }
}

impl<T: std::cmp::Eq> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, (row, col): (usize, usize)) -> &T {
        self.inner.index((row, col))
    }
}

impl<T: std::cmp::Eq> IndexMut<(usize, usize)> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut T {
        self.inner.index_mut((row, col))
    }
}

impl<T: Hash + std::cmp::Eq> std::hash::Hash for Grid<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.iter().for_each(|entry| entry.hash(state));
    }
}

impl<T: Display + std::cmp::Eq> Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\n").unwrap();

        self.inner.iter_rows().for_each(|row| {
            row.for_each(|value| {
                let _ = f.write_fmt(format_args!("{}", *value));
            });
            f.write_str("\n").unwrap();
        });

        Ok(())
    }
}

impl<T: Default + From<char> + std::cmp::Eq> From<&str> for Grid<T> {
    fn from(s: &str) -> Self {
        let mut inner = grid::Grid::new(0, 0);

        for line in s.lines() {
            inner.push_row(line.chars().map_into().collect());
        }

        Self { inner }
    }
}

impl<T: std::cmp::Eq> From<grid::Grid<T>> for Grid<T> {
    fn from(value: grid::Grid<T>) -> Self {
        Self { inner: value }
    }
}
