extern crate fixedbitset;
extern crate rand;
extern crate uuid;

pub mod stats;
pub mod trace;
pub mod tags;
pub mod context;
pub mod common;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
