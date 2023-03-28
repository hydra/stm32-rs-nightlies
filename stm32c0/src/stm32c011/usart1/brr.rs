///Register `BRR` reader
pub struct R(crate::R<BRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BRR` writer
pub struct W(crate::W<BRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRR_SPEC>;
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
impl From<crate::W<BRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BRR` reader - USART baud rate BRR\[15:4\]
///BRR\[15:4\]
///= USARTDIV\[15:4\]
///BRR\[3:0\]
///When OVER8 = 0, BRR\[3:0\]
///= USARTDIV\[3:0\]. When OVER8 = 1: BRR\[2:0\]
///= USARTDIV\[3:0\]
///shifted 1 bit to the right. BRR\[3\]
///must be kept cleared.
pub type BRR_R = crate::FieldReader<u16, u16>;
///Field `BRR` writer - USART baud rate BRR\[15:4\]
///BRR\[15:4\]
///= USARTDIV\[15:4\]
///BRR\[3:0\]
///When OVER8 = 0, BRR\[3:0\]
///= USARTDIV\[3:0\]. When OVER8 = 1: BRR\[2:0\]
///= USARTDIV\[3:0\]
///shifted 1 bit to the right. BRR\[3\]
///must be kept cleared.
pub type BRR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BRR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - USART baud rate BRR\[15:4\]
    ///BRR\[15:4\]
    ///= USARTDIV\[15:4\]
    ///BRR\[3:0\]
    ///When OVER8 = 0, BRR\[3:0\]
    ///= USARTDIV\[3:0\]. When OVER8 = 1: BRR\[2:0\]
    ///= USARTDIV\[3:0\]
    ///shifted 1 bit to the right. BRR\[3\]
    ///must be kept cleared.
    #[inline(always)]
    pub fn brr(&self) -> BRR_R {
        BRR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - USART baud rate BRR\[15:4\]
    ///BRR\[15:4\]
    ///= USARTDIV\[15:4\]
    ///BRR\[3:0\]
    ///When OVER8 = 0, BRR\[3:0\]
    ///= USARTDIV\[3:0\]. When OVER8 = 1: BRR\[2:0\]
    ///= USARTDIV\[3:0\]
    ///shifted 1 bit to the right. BRR\[3\]
    ///must be kept cleared.
    #[inline(always)]
    #[must_use]
    pub fn brr(&mut self) -> BRR_W<0> {
        BRR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///USART baud rate register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [brr](index.html) module
pub struct BRR_SPEC;
impl crate::RegisterSpec for BRR_SPEC {
    type Ux = u32;
}
///`read()` method returns [brr::R](R) reader structure
impl crate::Readable for BRR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [brr::W](W) writer structure
impl crate::Writable for BRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BRR to value 0
impl crate::Resettable for BRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
