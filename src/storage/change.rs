#[derive(Clone, Debug)]
pub enum Change<T> {
    Added(T),
    Modified(T, T),
    Removed(T),
}

pub trait ChangeSplit {
    type Output;
    fn into_added(self) -> Self::Output;
    fn into_modified(old: Self, new: Self) -> Self::Output;
    fn into_removed(self) -> Self::Output;
}

impl<T> Change<T> {
    pub fn map<F, U>(self, mut func: F) -> Change<U>
    where
        F: FnMut(T) -> U,
    {
        match self {
            Self::Added(new) => Change::Added(func(new)),
            Self::Modified(old, new) => Change::Modified(func(old), func(new)),
            Self::Removed(old) => Change::Removed(func(old)),
        }
    }

    pub fn split<U>(self) -> U
    where
        T: ChangeSplit<Output = U>,
    {
        match self {
            Self::Added(new) => ChangeSplit::into_added(new),
            Self::Modified(old, new) => ChangeSplit::into_modified(old, new),
            Self::Removed(old) => ChangeSplit::into_removed(old),
        }
    }

    pub fn check_modified(self) -> Option<Self>
    where
        T: PartialEq,
    {
        match self {
            Self::Added(new) => Some(Self::Added(new)),
            Self::Modified(old, new) => {
                if old == new {
                    None
                } else {
                    Some(Self::Modified(old, new))
                }
            }
            Self::Removed(old) => Some(Self::Removed(old)),
        }
    }
}

impl<T: Clone> Change<&T> {
    pub fn cloned(&self) -> Change<T> {
        match self {
            Self::Added(value) => Change::Added((*value).clone()),
            Self::Modified(old, new) => Change::Modified((*old).clone(), (*new).clone()),
            Self::Removed(value) => Change::Removed((*value).clone()),
        }
    }
}

impl ChangeSplit for () {
    type Output = ();

    fn into_added(self) -> Self::Output {
        ()
    }

    fn into_modified(old: Self, new: Self) -> Self::Output {
        ()
    }

    fn into_removed(self) -> Self::Output {
        ()
    }
}

impl<T1> ChangeSplit for (T1,) {
    type Output = (Change<T1>,);

    fn into_added(self) -> Self::Output {
        (Change::Added(self.0),)
    }

    fn into_modified(old: Self, new: Self) -> Self::Output {
        (Change::Modified(old.0, new.0),)
    }

    fn into_removed(self) -> Self::Output {
        (Change::Removed(self.0),)
    }
}

impl<T1, T2> ChangeSplit for (T1, T2) {
    type Output = (Change<T1>, Change<T2>);

    fn into_added(self) -> Self::Output {
        (Change::Added(self.0), Change::Added(self.1))
    }

    fn into_modified(old: Self, new: Self) -> Self::Output {
        (
            Change::Modified(old.0, new.0),
            Change::Modified(old.1, new.1),
        )
    }

    fn into_removed(self) -> Self::Output {
        (Change::Removed(self.0), Change::Removed(self.1))
    }
}

impl<T1, T2, T3> ChangeSplit for (T1, T2, T3) {
    type Output = (Change<T1>, Change<T2>, Change<T3>);

    fn into_added(self) -> Self::Output {
        (
            Change::Added(self.0),
            Change::Added(self.1),
            Change::Added(self.2),
        )
    }

    fn into_modified(old: Self, new: Self) -> Self::Output {
        (
            Change::Modified(old.0, new.0),
            Change::Modified(old.1, new.1),
            Change::Modified(old.2, new.2),
        )
    }

    fn into_removed(self) -> Self::Output {
        (
            Change::Removed(self.0),
            Change::Removed(self.1),
            Change::Removed(self.2),
        )
    }
}

impl<T1, T2, T3, T4> ChangeSplit for (T1, T2, T3, T4) {
    type Output = (Change<T1>, Change<T2>, Change<T3>, Change<T4>);

    fn into_added(self) -> Self::Output {
        (
            Change::Added(self.0),
            Change::Added(self.1),
            Change::Added(self.2),
            Change::Added(self.3),
        )
    }

    fn into_modified(old: Self, new: Self) -> Self::Output {
        (
            Change::Modified(old.0, new.0),
            Change::Modified(old.1, new.1),
            Change::Modified(old.2, new.2),
            Change::Modified(old.3, new.3),
        )
    }

    fn into_removed(self) -> Self::Output {
        (
            Change::Removed(self.0),
            Change::Removed(self.1),
            Change::Removed(self.2),
            Change::Removed(self.3),
        )
    }
}
