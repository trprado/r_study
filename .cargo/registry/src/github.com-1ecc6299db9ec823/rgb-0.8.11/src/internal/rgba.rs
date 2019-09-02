use core;
use core::fmt;
use super::pixel::*;
use RGB;
use RGBA;
use alt::BGR;
use alt::BGRA;

impl<T> RGBA<T> {
    #[inline(always)]
    pub fn new(r: T, g: T, b: T, a: T) -> Self {
        Self {r,g,b,a}
    }
}

impl<T, A> RGBA<T,A> {
    #[inline(always)]
    pub fn new_alpha(r: T, g: T, b: T, a: A) -> Self {
        Self {r,g,b,a}
    }
}

macro_rules! impl_rgba {
    ($RGBA:ident, $RGB:ident, $BGRA:ident) => {
        impl<T: Clone> $RGBA<T> {
            /// Iterate over all components (length=4)
            #[inline(always)]
            pub fn iter(&self) -> core::iter::Cloned<core::slice::Iter<T>> {
                self.as_slice().iter().cloned()
            }
        }

        impl<T: Clone, A> $RGBA<T, A> {
            /// Copy RGB components out of the RGBA struct
            #[inline(always)]
            pub fn rgb(&self) -> $RGB<T> {
                $RGB {r:self.r.clone(), g:self.g.clone(), b:self.b.clone()}
            }
        }

        impl<T, A> $RGBA<T, A> {
            /// Provide a mutable view of only RGB components (leaving out alpha). Useful to change color without changing opacity.
            #[inline(always)]
            pub fn rgb_mut(&mut self) -> &mut $RGB<T> {
                unsafe {
                    core::mem::transmute(self)
                }
            }
        }

        impl<T: Copy, A: Clone> $RGBA<T, A> {
            /// Create new RGBA with the same alpha value, but different RGB values
            #[inline(always)]
            pub fn map_rgb<F, U, B>(&self, f: F) -> $RGBA<U, B>
                where F: FnMut(T) -> U, U: Clone, B: From<A> + Clone {
                self.rgb().map(f).new_alpha(self.a.clone().into())
            }
        }

        impl<T: Copy, B> ComponentMap<$RGBA<B>, T, B> for $RGBA<T> {
            #[inline(always)]
            fn map<F>(&self, mut f: F) -> $RGBA<B>
            where
                F: FnMut(T) -> B,
            {
                $RGBA {
                    r: f(self.r),
                    g: f(self.g),
                    b: f(self.b),
                    a: f(self.a),
                }
            }
        }

        impl<T> ComponentSlice<T> for $RGBA<T> {
            #[inline(always)]
            fn as_slice(&self) -> &[T] {
                unsafe {
                    core::slice::from_raw_parts(self as *const Self as *const T, 4)
                }
            }

            #[inline(always)]
            fn as_mut_slice(&mut self) -> &mut [T] {
                unsafe {
                    core::slice::from_raw_parts_mut(self as *mut Self as *mut T, 4)
                }
            }
        }

        impl<T> ComponentSlice<T> for [$RGBA<T>] {
            #[inline]
            fn as_slice(&self) -> &[T] {
                unsafe {
                    core::slice::from_raw_parts(self.as_ptr() as *const _, self.len() * 4)
                }
            }
            #[inline]
            fn as_mut_slice(&mut self) -> &mut [T] {
                unsafe {
                    core::slice::from_raw_parts_mut(self.as_ptr() as *mut _, self.len() * 4)
                }
            }
        }

        impl<T: Copy + Send + Sync + 'static> ComponentBytes<T> for [$RGBA<T>] {}

        /// Assumes 255 is opaque
        impl<T: Copy> From<$RGB<T>> for $RGBA<T, u8> {
            fn from(other: $RGB<T>) -> Self {
                Self {
                    r: other.r,
                    g: other.g,
                    b: other.b,
                    a: 0xFF,
                }
            }
        }

        /// Assumes 255 is opaque
        impl<T: Copy> From<$RGB<T>> for $BGRA<T, u8> {
            fn from(other: $RGB<T>) -> Self {
                Self {
                    r: other.r,
                    g: other.g,
                    b: other.b,
                    a: 0xFF,
                }
            }
        }

        /// Assumes 65535 is opaque
        impl<T: Copy> From<$RGB<T>> for $RGBA<T, u16> {
            fn from(other: $RGB<T>) -> Self {
                Self {
                    r: other.r,
                    g: other.g,
                    b: other.b,
                    a: 0xFFFF,
                }
            }
        }

        /// Assumes 255 is opaque
        impl<T: Copy> From<$RGB<T>> for $BGRA<T, u16> {
            fn from(other: $RGB<T>) -> Self {
                Self {
                    r: other.r,
                    g: other.g,
                    b: other.b,
                    a: 0xFFFF,
                }
            }
        }
    }
}

impl<T> core::iter::FromIterator<T> for RGBA<T> {
    #[inline(always)]
    /// Takes exactly 4 elements from the iterator and creates a new instance.
    /// Panics if there are fewer elements in the iterator.
    fn from_iter<I: IntoIterator<Item = T>>(into_iter: I) -> Self {
        let mut iter = into_iter.into_iter();
        Self {
            r: iter.next().unwrap(),
            g: iter.next().unwrap(),
            b: iter.next().unwrap(),
            a: iter.next().unwrap(),
        }
    }
}

impl_rgba! {RGBA, RGB, BGRA}
impl_rgba! {BGRA, BGR, RGBA}

impl<T: fmt::Display, A: fmt::Display> fmt::Display for RGBA<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "rgba({},{},{},{})", self.r, self.g, self.b, self.a)
    }
}

impl<T: fmt::Display, A: fmt::Display> fmt::Display for BGRA<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bgra({},{},{},{})", self.r, self.g, self.b, self.a)
    }
}

#[test]
fn rgba_test() {
    let neg = RGBA::new(1,2,3i32,1000).map(|x| -x);
    assert_eq!(neg.r, -1);
    assert_eq!(neg.rgb().r, -1);
    assert_eq!(neg.g, -2);
    assert_eq!(neg.rgb().g, -2);
    assert_eq!(neg.b, -3);
    assert_eq!(neg.rgb().b, -3);
    assert_eq!(neg.a, -1000);
    assert_eq!(neg, neg.as_slice().iter().cloned().collect());
    assert!(neg < RGBA::new(0,0,0,0));

    let neg = RGBA::new(1u8,2,3,4).map_rgb(|c| -(c as i16));
    assert_eq!(-1i16, neg.r);
    assert_eq!(4i16, neg.a);

    let mut px = RGBA{r:1,g:2,b:3,a:4};
    px.as_mut_slice()[3] = 100;
    assert_eq!(1, px.rgb_mut().r);
    assert_eq!(2, px.rgb_mut().g);
    px.rgb_mut().b = 4;
    assert_eq!(4, px.rgb_mut().b);
    assert_eq!(100, px.a);

    let v = vec![RGBA::new(1u8,2,3,4), RGBA::new(5,6,7,8)];
    assert_eq!(&[1,2,3,4,5,6,7,8], v.as_bytes());
}

#[test]
fn bgra_test() {
    let neg = BGRA{r:1,g:2,b:3i32,a:1000}.map(|x| -x);
    assert_eq!(neg.r, -1);
    assert_eq!(neg.rgb().r, -1);
    assert_eq!(neg.g, -2);
    assert_eq!(neg.rgb().g, -2);
    assert_eq!(neg.b, -3);
    assert_eq!(neg.rgb().b, -3);
    assert_eq!(neg.a, -1000);
    assert_eq!(&[-3,-2,-1,-1000], neg.as_slice());
    assert!(neg < BGRA{r:0,g:0,b:0,a:0});

    let neg = BGRA{r:1u8,g:2u8,b:3u8,a:4u8}.map_rgb(|c| -(c as i16));
    assert_eq!(-1i16, neg.r);
    assert_eq!(4i16, neg.a);

    let mut px = BGRA{r:1,g:2,b:3,a:4};
    px.as_mut_slice()[3] = 100;
    assert_eq!(1, px.rgb_mut().r);
    assert_eq!(2, px.rgb_mut().g);
    px.rgb_mut().b = 4;
    assert_eq!(4, px.rgb_mut().b);
    assert_eq!(100, px.a);

    let v = vec![BGRA{b:1u8,g:2,r:3,a:4}, BGRA{b:5,g:6,r:7,a:8}];
    assert_eq!(&[1,2,3,4,5,6,7,8], v.as_bytes());
}
