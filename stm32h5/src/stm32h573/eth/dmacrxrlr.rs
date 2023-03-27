///Register `DMACRXRLR` reader
pub struct R(crate::R<DMACRXRLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACRXRLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACRXRLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACRXRLR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DMACRXRLR` writer
pub struct W(crate::W<DMACRXRLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACRXRLR_SPEC>;
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
impl From<crate::W<DMACRXRLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACRXRLR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RDRL` reader - Receive Descriptor Ring Length This register sets the maximum number of Rx descriptors in the circular descriptor ring. The maximum number of descriptors is limited to 1K descriptors. For example, you can program any value up to 0x3FF in this field. This field is 10-bit wide. If you program 0x3FF, you can have 1024 descriptors. If you want to have 10 descriptors, program it to a value of 0x9.
pub type RDRL_R = crate::FieldReader<u16, u16>;
///Field `RDRL` writer - Receive Descriptor Ring Length This register sets the maximum number of Rx descriptors in the circular descriptor ring. The maximum number of descriptors is limited to 1K descriptors. For example, you can program any value up to 0x3FF in this field. This field is 10-bit wide. If you program 0x3FF, you can have 1024 descriptors. If you want to have 10 descriptors, program it to a value of 0x9.
pub type RDRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMACRXRLR_SPEC, u16, u16, 10, O>;
///Field `ARBS` reader - Alternate Receive Buffer Size Indicates size in bytes for Buffer 1 when ARBS\[7:0\]
///is programmed to a non-zero value. When ARBS\[7:0\]�=�0, Rx Buffer1 and Rx Buffer2 sizes are based on RBSZ\[13:0\]
///field of Channel receive control register (ETH_DMACRXCR).
pub type ARBS_R = crate::FieldReader<u8, u8>;
///Field `ARBS` writer - Alternate Receive Buffer Size Indicates size in bytes for Buffer 1 when ARBS\[7:0\]
///is programmed to a non-zero value. When ARBS\[7:0\]�=�0, Rx Buffer1 and Rx Buffer2 sizes are based on RBSZ\[13:0\]
///field of Channel receive control register (ETH_DMACRXCR).
pub type ARBS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMACRXRLR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:9 - Receive Descriptor Ring Length This register sets the maximum number of Rx descriptors in the circular descriptor ring. The maximum number of descriptors is limited to 1K descriptors. For example, you can program any value up to 0x3FF in this field. This field is 10-bit wide. If you program 0x3FF, you can have 1024 descriptors. If you want to have 10 descriptors, program it to a value of 0x9.
    #[inline(always)]
    pub fn rdrl(&self) -> RDRL_R {
        RDRL_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 16:23 - Alternate Receive Buffer Size Indicates size in bytes for Buffer 1 when ARBS\[7:0\]
    ///is programmed to a non-zero value. When ARBS\[7:0\]�=�0, Rx Buffer1 and Rx Buffer2 sizes are based on RBSZ\[13:0\]
    ///field of Channel receive control register (ETH_DMACRXCR).
    #[inline(always)]
    pub fn arbs(&self) -> ARBS_R {
        ARBS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:9 - Receive Descriptor Ring Length This register sets the maximum number of Rx descriptors in the circular descriptor ring. The maximum number of descriptors is limited to 1K descriptors. For example, you can program any value up to 0x3FF in this field. This field is 10-bit wide. If you program 0x3FF, you can have 1024 descriptors. If you want to have 10 descriptors, program it to a value of 0x9.
    #[inline(always)]
    #[must_use]
    pub fn rdrl(&mut self) -> RDRL_W<0> {
        RDRL_W::new(self)
    }
    ///Bits 16:23 - Alternate Receive Buffer Size Indicates size in bytes for Buffer 1 when ARBS\[7:0\]
    ///is programmed to a non-zero value. When ARBS\[7:0\]�=�0, Rx Buffer1 and Rx Buffer2 sizes are based on RBSZ\[13:0\]
    ///field of Channel receive control register (ETH_DMACRXCR).
    #[inline(always)]
    #[must_use]
    pub fn arbs(&mut self) -> ARBS_W<16> {
        ARBS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Channel Rx descriptor ring length register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmacrxrlr](index.html) module
pub struct DMACRXRLR_SPEC;
impl crate::RegisterSpec for DMACRXRLR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmacrxrlr::R](R) reader structure
impl crate::Readable for DMACRXRLR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dmacrxrlr::W](W) writer structure
impl crate::Writable for DMACRXRLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DMACRXRLR to value 0
impl crate::Resettable for DMACRXRLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
