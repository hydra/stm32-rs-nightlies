///Register `MAXRLR` reader
pub struct R(crate::R<MAXRLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAXRLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAXRLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAXRLR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MAXRLR` writer
pub struct W(crate::W<MAXRLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAXRLR_SPEC>;
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
impl From<crate::W<MAXRLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAXRLR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MRL` reader - maximum data read length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMRL command (with potentially also updated IBIP\[2:0\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMRL CCC.
pub type MRL_R = crate::FieldReader<u16, u16>;
///Field `MRL` writer - maximum data read length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMRL command (with potentially also updated IBIP\[2:0\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMRL CCC.
pub type MRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAXRLR_SPEC, u16, u16, 16, O>;
///Field `IBIP` reader - IBI payload data size, in bytes (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 to set the number of data bytes to be sent to the controller after an IBI request has been acknowledged.This field may be updated by hardware on the reception of SETMRL command (which potentially also updated IBIP\[2:0\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. others: same as 100
pub type IBIP_R = crate::FieldReader<u8, u8>;
///Field `IBIP` writer - IBI payload data size, in bytes (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 to set the number of data bytes to be sent to the controller after an IBI request has been acknowledged.This field may be updated by hardware on the reception of SETMRL command (which potentially also updated IBIP\[2:0\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. others: same as 100
pub type IBIP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAXRLR_SPEC, u8, u8, 3, O>;
impl R {
    ///Bits 0:15 - maximum data read length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMRL command (with potentially also updated IBIP\[2:0\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMRL CCC.
    #[inline(always)]
    pub fn mrl(&self) -> MRL_R {
        MRL_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:18 - IBI payload data size, in bytes (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 to set the number of data bytes to be sent to the controller after an IBI request has been acknowledged.This field may be updated by hardware on the reception of SETMRL command (which potentially also updated IBIP\[2:0\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. others: same as 100
    #[inline(always)]
    pub fn ibip(&self) -> IBIP_R {
        IBIP_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    ///Bits 0:15 - maximum data read length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMRL command (with potentially also updated IBIP\[2:0\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMRL CCC.
    #[inline(always)]
    #[must_use]
    pub fn mrl(&mut self) -> MRL_W<0> {
        MRL_W::new(self)
    }
    ///Bits 16:18 - IBI payload data size, in bytes (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 to set the number of data bytes to be sent to the controller after an IBI request has been acknowledged.This field may be updated by hardware on the reception of SETMRL command (which potentially also updated IBIP\[2:0\]). Software is notified of a MRL update by the I3C_EVR.MRLUPF and the corresponding interrupt if enabled. others: same as 100
    #[inline(always)]
    #[must_use]
    pub fn ibip(&mut self) -> IBIP_W<16> {
        IBIP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///I3C maximum read length register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [maxrlr](index.html) module
pub struct MAXRLR_SPEC;
impl crate::RegisterSpec for MAXRLR_SPEC {
    type Ux = u32;
}
///`read()` method returns [maxrlr::R](R) reader structure
impl crate::Readable for MAXRLR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [maxrlr::W](W) writer structure
impl crate::Writable for MAXRLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MAXRLR to value 0
impl crate::Resettable for MAXRLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
