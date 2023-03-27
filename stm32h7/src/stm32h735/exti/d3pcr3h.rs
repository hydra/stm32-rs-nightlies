///Register `D3PCR3H` reader
pub struct R(crate::R<D3PCR3H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D3PCR3H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D3PCR3H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D3PCR3H_SPEC>) -> Self {
        R(reader)
    }
}
///Register `D3PCR3H` writer
pub struct W(crate::W<D3PCR3H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D3PCR3H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<D3PCR3H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D3PCR3H_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PCS88` reader - D3 Pending request clear input signal selection on Event input x= truncate N+160/2
pub type PCS88_R = crate::FieldReader<u8, PCS88_A>;
///D3 Pending request clear input signal selection on Event input x= truncate N+160/2
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCS88_A {
    ///0: DMA ch6 event selected as D3 domain pendclear source
    DmaCh6 = 0,
    ///1: DMA ch7 event selected as D3 domain pendclear source
    DmaCh7 = 1,
    ///2: LPTIM4 out selected as D3 domain pendclear source
    Lptim4 = 2,
    ///3: LPTIM5 out selected as D3 domain pendclear source
    Lptim5 = 3,
}
impl From<PCS88_A> for u8 {
    #[inline(always)]
    fn from(variant: PCS88_A) -> Self {
        variant as _
    }
}
impl PCS88_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PCS88_A {
        match self.bits {
            0 => PCS88_A::DmaCh6,
            1 => PCS88_A::DmaCh7,
            2 => PCS88_A::Lptim4,
            3 => PCS88_A::Lptim5,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `DmaCh6`
    #[inline(always)]
    pub fn is_dma_ch6(&self) -> bool {
        *self == PCS88_A::DmaCh6
    }
    ///Checks if the value of the field is `DmaCh7`
    #[inline(always)]
    pub fn is_dma_ch7(&self) -> bool {
        *self == PCS88_A::DmaCh7
    }
    ///Checks if the value of the field is `Lptim4`
    #[inline(always)]
    pub fn is_lptim4(&self) -> bool {
        *self == PCS88_A::Lptim4
    }
    ///Checks if the value of the field is `Lptim5`
    #[inline(always)]
    pub fn is_lptim5(&self) -> bool {
        *self == PCS88_A::Lptim5
    }
}
///Field `PCS88` writer - D3 Pending request clear input signal selection on Event input x= truncate N+160/2
pub type PCS88_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, D3PCR3H_SPEC, u8, PCS88_A, 2, O>;
impl<'a, const O: u8> PCS88_W<'a, O> {
    ///DMA ch6 event selected as D3 domain pendclear source
    #[inline(always)]
    pub fn dma_ch6(self) -> &'a mut W {
        self.variant(PCS88_A::DmaCh6)
    }
    ///DMA ch7 event selected as D3 domain pendclear source
    #[inline(always)]
    pub fn dma_ch7(self) -> &'a mut W {
        self.variant(PCS88_A::DmaCh7)
    }
    ///LPTIM4 out selected as D3 domain pendclear source
    #[inline(always)]
    pub fn lptim4(self) -> &'a mut W {
        self.variant(PCS88_A::Lptim4)
    }
    ///LPTIM5 out selected as D3 domain pendclear source
    #[inline(always)]
    pub fn lptim5(self) -> &'a mut W {
        self.variant(PCS88_A::Lptim5)
    }
}
impl R {
    ///Bits 18:19 - D3 Pending request clear input signal selection on Event input x= truncate N+160/2
    #[inline(always)]
    pub fn pcs88(&self) -> PCS88_R {
        PCS88_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    ///Bits 18:19 - D3 Pending request clear input signal selection on Event input x= truncate N+160/2
    #[inline(always)]
    #[must_use]
    pub fn pcs88(&mut self) -> PCS88_W<18> {
        PCS88_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EXTI D3 pending clear selection register high
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [d3pcr3h](index.html) module
pub struct D3PCR3H_SPEC;
impl crate::RegisterSpec for D3PCR3H_SPEC {
    type Ux = u32;
}
///`read()` method returns [d3pcr3h::R](R) reader structure
impl crate::Readable for D3PCR3H_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [d3pcr3h::W](W) writer structure
impl crate::Writable for D3PCR3H_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets D3PCR3H to value 0
impl crate::Resettable for D3PCR3H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
