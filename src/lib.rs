mod ffi;
pub mod usd;
pub mod usd_geom;
pub mod usd_shade;
pub mod tf;
pub mod sdf;
pub mod cpp;
pub mod vt;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
