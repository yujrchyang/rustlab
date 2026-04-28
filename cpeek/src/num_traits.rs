#[cfg(test)]
mod tests {
    use num_traits::*;

    #[test]
    fn test_clamp_min() {
        let u: u8 = 1;
        assert_eq!(1, clamp_min(u, 0));
        assert_eq!(1, clamp_min(u, 1));
        assert_eq!(2, clamp_min(u, 2));

        let v: i32 = -1;
        assert_eq!(0, clamp_min(v, 0));
        assert_eq!(-1, clamp_min(v, -2));
        assert_eq!(-1, clamp_min(v, -1));
        assert_eq!(1, clamp_min(v, 1));

        let w: f64 = 1.0;
        assert_eq!(1.0, clamp_min(w, 0.0));
        assert_eq!(1.0, clamp_min(w, 1.0));
        assert_eq!(2.0, clamp_min(w, 2.0));
    }

    #[test]
    fn test_clamp_max() {
        let u: u8 = 1;
        assert_eq!(0, clamp_max(u, 0));
        assert_eq!(1, clamp_max(u, 1));
        assert_eq!(1, clamp_max(u, 2));

        let v: i32 = -1;
        assert_eq!(-1, clamp_max(v, 0));
        assert_eq!(-2, clamp_max(v, -2));
        assert_eq!(-1, clamp_max(v, -1));
        assert_eq!(-1, clamp_max(v, 1));

        let w: f64 = 1.0;
        assert_eq!(0.0, clamp_max(w, 0.0));
        assert_eq!(1.0, clamp_max(w, 1.0));
        assert_eq!(1.0, clamp_max(w, 2.0));
    }

    #[test]
    fn test_clamp() {
        let u: u8 = 5;
        assert_eq!(2, clamp(u, 0, 2));
        assert_eq!(5, clamp(u, 4, 5));
        assert_eq!(6, clamp(u, 6, 7));

        let v: i32 = -1;
        assert_eq!(-2, clamp(v, -3, -2));
        assert_eq!(-1, clamp(v, -2, 0));
        assert_eq!(0, clamp(v, 0, 1));

        let w: f64 = -1.0;
        assert_eq!(-2.0, clamp(w, -3.0, -2.0));
        assert_eq!(-1.0, clamp(w, -2.0, 0.0));
        assert_eq!(0.0, clamp(w, 0.0, 2.0));
    }
}
