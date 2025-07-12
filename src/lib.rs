#![feature(test, int_format_into)]

#[cfg(test)]
mod bench {
    extern crate test;
    use core::fmt::NumBuffer;
    use itoa::Buffer;
    use test::{Bencher, black_box};

    mod all_digits {
        use super::*;

        #[bench]
        fn std1(b: &mut Bencher) {
            b.iter(|| {
                for num in (1..=usize::MAX.ilog10())
                    .map(|exp| 10usize.pow(exp) - 1)
                    .chain(std::iter::once(usize::MAX))
                {
                    let _s = black_box(num.to_string());
                }
            });
        }

        #[bench]
        fn std2(b: &mut Bencher) {
            b.iter(|| {
                for num in (1..=usize::MAX.ilog10())
                    .map(|exp| 10usize.pow(exp) - 1)
                    .chain(std::iter::once(usize::MAX))
                {
                    let _s = black_box(format!("{num}"));
                }
            });
        }

        #[bench]
        fn std3(b: &mut Bencher) {
            let mut buff = NumBuffer::new();
            b.iter(|| {
                for num in (1..=usize::MAX.ilog10())
                    .map(|exp| 10usize.pow(exp) - 1)
                    .chain(std::iter::once(usize::MAX))
                {
                    let _s = black_box(num.format_into(&mut buff));
                }
            });
        }

        #[bench]
        fn itoa(b: &mut Bencher) {
            let mut buff = Buffer::new();
            b.iter(|| {
                for num in (1..=usize::MAX.ilog10())
                    .map(|exp| 10usize.pow(exp) - 1)
                    .chain(std::iter::once(usize::MAX))
                {
                    let _s = black_box(buff.format(num));
                }
            });
        }
    }

    mod first_999 {
        use super::*;

        #[bench]
        fn std1(b: &mut Bencher) {
            b.iter(|| {
                for num in 0..1000usize {
                    let _s = black_box(num.to_string());
                }
            });
        }

        #[bench]
        fn std2(b: &mut Bencher) {
            b.iter(|| {
                for num in 0..1000usize {
                    let _s = black_box(format!("{num}"));
                }
            });
        }

        #[bench]
        fn std3(b: &mut Bencher) {
            let mut buff = NumBuffer::new();
            b.iter(|| {
                for num in 0..1000usize {
                    let _s = black_box(num.format_into(&mut buff));
                }
            });
        }

        #[bench]
        fn itoa(b: &mut Bencher) {
            let mut buff = Buffer::new();
            b.iter(|| {
                for num in 0..1000usize {
                    let _s = black_box(buff.format(num));
                }
            });
        }
    }

    mod first_9 {
        use super::*;

        #[bench]
        fn std1(b: &mut Bencher) {
            b.iter(|| {
                for num in 0..10usize {
                    let _s = black_box(num.to_string());
                }
            });
        }

        #[bench]
        fn std2(b: &mut Bencher) {
            b.iter(|| {
                for num in 0..10usize {
                    let _s = black_box(format!("{num}"));
                }
            });
        }

        #[bench]
        fn std3(b: &mut Bencher) {
            let mut buff = NumBuffer::new();
            b.iter(|| {
                for num in 0..10usize {
                    let _s = black_box(num.format_into(&mut buff));
                }
            });
        }

        #[bench]
        fn itoa(b: &mut Bencher) {
            let mut buff: Buffer = Buffer::new();
            b.iter(|| {
                for num in 0..10usize {
                    let _s = black_box(buff.format(num));
                }
            });
        }
    }
}
