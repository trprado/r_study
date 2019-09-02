use core::ops::*;
use super::pixel::*;
use RGB;
use RGBA;

/// `px + px`
impl<T: Add> Add for RGB<T> {
    type Output = RGB<<T as Add>::Output>;

    #[inline(always)]
    fn add(self, other: RGB<T>) -> Self::Output {
        RGB {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

/// `px + px`
impl<T: Add, A: Add> Add<RGBA<T, A>> for RGBA<T, A> {
    type Output = RGBA<<T as Add>::Output, <A as Add>::Output>;

    #[inline(always)]
    fn add(self, other: RGBA<T, A>) -> Self::Output {
        RGBA {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a,
        }
    }
}

/// `px - px`
impl<T: Sub> Sub for RGB<T> {
    type Output = RGB<<T as Sub>::Output>;

    #[inline(always)]
    fn sub(self, other: RGB<T>) -> Self::Output {
        RGB {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

/// `px - px`
impl<T: Sub, A: Sub> Sub<RGBA<T, A>> for RGBA<T, A> {
    type Output = RGBA<<T as Sub>::Output, <A as Sub>::Output>;

    #[inline(always)]
    fn sub(self, other: RGBA<T, A>) -> Self::Output {
        RGBA {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
            a: self.a - other.a,
        }
    }
}

/// `px + 1`
impl<T: Clone + Copy + Add> Add<T> for RGB<T>
    where T: Add<Output=T> {
    type Output = RGB<T>;

    #[inline(always)]
    fn add(self, r: T) -> Self::Output {
        self.map(|l|l+r)
    }
}

/// `px + 1`
impl<T: Clone + Copy + Add> Add<T> for RGBA<T>
    where T: Add<Output=T> {
    type Output = RGBA<T>;

    #[inline(always)]
    fn add(self, r: T) -> Self::Output {
        self.map(|l|l+r)
    }
}

/// `px * 1`
impl<T: Clone + Copy + Mul> Mul<T> for RGB<T>
    where T: Mul<Output=T> {
    type Output = RGB<T>;

    #[inline(always)]
    fn mul(self, r: T) -> Self::Output {
        self.map(|l|l*r)
    }
}

/// `px * 1`
impl<T: Clone + Copy + Mul> Mul<T> for RGBA<T>
    where T: Mul<Output=T> {
    type Output = RGBA<T>;

    #[inline(always)]
    fn mul(self, r: T) -> Self::Output {
        self.map(|l|l*r)
    }
}

#[test]
fn test_math() {
    assert_eq!(RGB::new(2,4,6), RGB::new(1,2,3) + RGB{r:1,g:2,b:3});
    assert_eq!(RGB::new(2.,4.,6.), RGB::new(1.,3.,5.) + 1.);
    assert_eq!(RGB::new(0.5,1.5,2.5), RGB::new(1.,3.,5.) * 0.5);

    assert_eq!(RGBA::new_alpha(2u8,4,6,8u16), RGBA::new_alpha(1u8,2,3,4u16) + RGBA{r:1u8,g:2,b:3,a:4u16});
    assert_eq!(RGBA::new(2i16,4,6,8), RGBA::new(1,3,5,7) + 1);
    assert_eq!(RGBA::new(2,4,6,8), RGBA::new(1,2,3,4) * 2);
}
