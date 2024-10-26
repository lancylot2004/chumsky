use super::*;

impl<'a, T, I, O, E> Parser<'a, I, O, E> for &T
where
    T: ?Sized + Parser<'a, I, O, E>,
    I: Input<'a>,
    E: ParserExtra<'a, I>,
{
    fn go<M: Mode>(&self, inp: &mut InputRef<'a, '_, I, E>) -> PResult<M, O>
    where
        Self: Sized,
    {
        M::invoke(*self, inp)
    }

    go_extra!(O);
}

impl<'a, T, I, O, E> ConfigParser<'a, I, O, E> for &T
where
    T: ?Sized + ConfigParser<'a, I, O, E>,
    I: Input<'a>,
    E: ParserExtra<'a, I>,
{
    type Config = T::Config;

    fn go_cfg<M: Mode>(&self, inp: &mut InputRef<'a, '_, I, E>, cfg: Self::Config) -> PResult<M, O>
    where
        Self: Sized,
    {
        M::invoke_cfg(*self, inp, cfg)
    }

    go_cfg_extra!(O);
}
