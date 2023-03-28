///Register `TGTTDR` reader
pub struct R(crate::R<TGTTDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TGTTDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TGTTDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TGTTDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TGTTDR` writer
pub struct W(crate::W<TGTTDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TGTTDR_SPEC>;
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
impl From<crate::W<TGTTDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TGTTDR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TGTTDCNT` reader - transmit data counter, in bytes (when I3C is configured as target) This field must be written by software in the same access when is asserted PRELOAD, in order to define the number of bytes to preload and to transmit. This field is updated by hardware and reports, when read, the remaining number of bytes to be loaded into the TX-FIFO.
pub type TGTTDCNT_R = crate::FieldReader<u16, u16>;
///Field `TGTTDCNT` writer - transmit data counter, in bytes (when I3C is configured as target) This field must be written by software in the same access when is asserted PRELOAD, in order to define the number of bytes to preload and to transmit. This field is updated by hardware and reports, when read, the remaining number of bytes to be loaded into the TX-FIFO.
pub type TGTTDCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TGTTDR_SPEC, u16, u16, 16, O>;
///Field `PRELOAD` reader - preload of the TX-FIFO (when I3C is configured as target) This bit must be written and asserted by software in the same access when is written and defined the number of bytes to preload into the TX-FIFO and to transmit. This bit is cleared by hardware when all the data bytes to transmit are loaded into the TX-FIFO.
pub type PRELOAD_R = crate::BitReader<bool>;
///Field `PRELOAD` writer - preload of the TX-FIFO (when I3C is configured as target) This bit must be written and asserted by software in the same access when is written and defined the number of bytes to preload into the TX-FIFO and to transmit. This bit is cleared by hardware when all the data bytes to transmit are loaded into the TX-FIFO.
pub type PRELOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, TGTTDR_SPEC, bool, O>;
impl R {
    ///Bits 0:15 - transmit data counter, in bytes (when I3C is configured as target) This field must be written by software in the same access when is asserted PRELOAD, in order to define the number of bytes to preload and to transmit. This field is updated by hardware and reports, when read, the remaining number of bytes to be loaded into the TX-FIFO.
    #[inline(always)]
    pub fn tgttdcnt(&self) -> TGTTDCNT_R {
        TGTTDCNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - preload of the TX-FIFO (when I3C is configured as target) This bit must be written and asserted by software in the same access when is written and defined the number of bytes to preload into the TX-FIFO and to transmit. This bit is cleared by hardware when all the data bytes to transmit are loaded into the TX-FIFO.
    #[inline(always)]
    pub fn preload(&self) -> PRELOAD_R {
        PRELOAD_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bits 0:15 - transmit data counter, in bytes (when I3C is configured as target) This field must be written by software in the same access when is asserted PRELOAD, in order to define the number of bytes to preload and to transmit. This field is updated by hardware and reports, when read, the remaining number of bytes to be loaded into the TX-FIFO.
    #[inline(always)]
    #[must_use]
    pub fn tgttdcnt(&mut self) -> TGTTDCNT_W<0> {
        TGTTDCNT_W::new(self)
    }
    ///Bit 16 - preload of the TX-FIFO (when I3C is configured as target) This bit must be written and asserted by software in the same access when is written and defined the number of bytes to preload into the TX-FIFO and to transmit. This bit is cleared by hardware when all the data bytes to transmit are loaded into the TX-FIFO.
    #[inline(always)]
    #[must_use]
    pub fn preload(&mut self) -> PRELOAD_W<16> {
        PRELOAD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///I3C target transmit configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tgttdr](index.html) module
pub struct TGTTDR_SPEC;
impl crate::RegisterSpec for TGTTDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [tgttdr::R](R) reader structure
impl crate::Readable for TGTTDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tgttdr::W](W) writer structure
impl crate::Writable for TGTTDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TGTTDR to value 0
impl crate::Resettable for TGTTDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
