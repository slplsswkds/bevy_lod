use bevy::prelude::Component;

#[derive(Component, Clone)]
pub struct LODDistances {
    pub l1: f32,
    pub l2: f32,
    pub l3: f32, // if l3 = 0 then l3 LOD will be visible always
}

impl LODDistances {
    pub fn new<T: Into<f32> + PartialOrd>(l1: T, l2: T, l3: T) -> Self {
        let l1 = l1.into();
        let l2 = l2.into();
        let l3 = l3.into();

        assert!(l1 < l2);
        assert!(l2 < l3 || l3 == 0.0);

        Self { l1, l2, l3 }
    }

    #[inline]
    pub fn get_tupple(&self) -> (f32, f32, f32) {
        let l1_distance = self.l1.clone();
        let l2_distance = self.l2.clone();
        let l3_distance = self.l3.clone();

        return (l1_distance, l2_distance, l3_distance);
    }
}
