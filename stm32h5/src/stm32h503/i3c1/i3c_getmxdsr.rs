///Register `I3C_GETMXDSR` reader
pub struct R(crate::R<I3C_GETMXDSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I3C_GETMXDSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I3C_GETMXDSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I3C_GETMXDSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `I3C_GETMXDSR` writer
pub struct W(crate::W<I3C_GETMXDSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I3C_GETMXDSR_SPEC>;
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
impl From<crate::W<I3C_GETMXDSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I3C_GETMXDSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `HOFFAS` reader - controller hand-off activity state This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates in which initial activity state the (other) current controller should expect the I3C bus after a controller-role hand-off to this controller-capable I3C, when returning the defining byte CRHDLY (0x91) to a GETMXDS CCC. This 2-bit field is used to return the CRHDLY1 byte in response to the GETCAPS CCC format 3, in order to state which is the activity state of this I3C when becoming controller after a controller-role hand-off, and consequently the time the former controller should wait before testing this I3C to be confirmed its ownership.
pub type HOFFAS_R = crate::FieldReader<u8, u8>;
///Field `HOFFAS` writer - controller hand-off activity state This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates in which initial activity state the (other) current controller should expect the I3C bus after a controller-role hand-off to this controller-capable I3C, when returning the defining byte CRHDLY (0x91) to a GETMXDS CCC. This 2-bit field is used to return the CRHDLY1 byte in response to the GETCAPS CCC format 3, in order to state which is the activity state of this I3C when becoming controller after a controller-role hand-off, and consequently the time the former controller should wait before testing this I3C to be confirmed its ownership.
pub type HOFFAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I3C_GETMXDSR_SPEC, u8, u8, 2, O>;
///Field `FMT` reader - GETMXDS CCC format This field is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates how is returned the GETMXDS format 1 (without MaxRdTurn) and format 2 (with MaxRdTurn). This bit is used to return the 2-byte format 1 (MaxWr, MaxRd) or 5-byte format 2 (MaxWr, MaxRd, 3-byte MaxRdTurn) byte in response to the GETCAPS CCC. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=0 and LSB=RDTURN\[7:0\]. - Max read turnaround time is less than 256 �s. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=RDTURN\[7:0\]
///and LSB=0. - Max read turnaround time is between 256 �s and 65535 �s - 3-byte MaxRdTurn is returned with MSB=RDTURN\[7:0\], middle byte=0 and LSB=0. - Max read turnaround time is between 65535 �s and 16 s.
pub type FMT_R = crate::FieldReader<u8, u8>;
///Field `FMT` writer - GETMXDS CCC format This field is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates how is returned the GETMXDS format 1 (without MaxRdTurn) and format 2 (with MaxRdTurn). This bit is used to return the 2-byte format 1 (MaxWr, MaxRd) or 5-byte format 2 (MaxWr, MaxRd, 3-byte MaxRdTurn) byte in response to the GETCAPS CCC. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=0 and LSB=RDTURN\[7:0\]. - Max read turnaround time is less than 256 �s. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=RDTURN\[7:0\]
///and LSB=0. - Max read turnaround time is between 256 �s and 65535 �s - 3-byte MaxRdTurn is returned with MSB=RDTURN\[7:0\], middle byte=0 and LSB=0. - Max read turnaround time is between 65535 �s and 16 s.
pub type FMT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I3C_GETMXDSR_SPEC, u8, u8, 2, O>;
///Field `RDTURN` reader - programmed byte of the 3-byte MaxRdTurn (maximum read turnaround byte) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and writes the value of the selected byte (via the FMT\[1:0\]
///field) of the 3-byte MaxRdTurn which is returned in response to the GETMXDS CCC format 2 to encode the maximum read turnaround time.
pub type RDTURN_R = crate::FieldReader<u8, u8>;
///Field `RDTURN` writer - programmed byte of the 3-byte MaxRdTurn (maximum read turnaround byte) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and writes the value of the selected byte (via the FMT\[1:0\]
///field) of the 3-byte MaxRdTurn which is returned in response to the GETMXDS CCC format 2 to encode the maximum read turnaround time.
pub type RDTURN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I3C_GETMXDSR_SPEC, u8, u8, 8, O>;
///Field `TSCO` reader - clock-to-data turnaround time (tSCO) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and is used to specify the clock-to-data turnaround time tSCO (vs the value of 12 ns). This bit is used by the hardware in response to the GETMXDS CCC to return the encoded clock-to-data turnaround time via the returned MaxRd\[5:3\]
///bits.
pub type TSCO_R = crate::BitReader<bool>;
///Field `TSCO` writer - clock-to-data turnaround time (tSCO) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and is used to specify the clock-to-data turnaround time tSCO (vs the value of 12 ns). This bit is used by the hardware in response to the GETMXDS CCC to return the encoded clock-to-data turnaround time via the returned MaxRd\[5:3\]
///bits.
pub type TSCO_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_GETMXDSR_SPEC, bool, O>;
impl R {
    ///Bits 0:1 - controller hand-off activity state This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates in which initial activity state the (other) current controller should expect the I3C bus after a controller-role hand-off to this controller-capable I3C, when returning the defining byte CRHDLY (0x91) to a GETMXDS CCC. This 2-bit field is used to return the CRHDLY1 byte in response to the GETCAPS CCC format 3, in order to state which is the activity state of this I3C when becoming controller after a controller-role hand-off, and consequently the time the former controller should wait before testing this I3C to be confirmed its ownership.
    #[inline(always)]
    pub fn hoffas(&self) -> HOFFAS_R {
        HOFFAS_R::new((self.bits & 3) as u8)
    }
    ///Bits 8:9 - GETMXDS CCC format This field is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates how is returned the GETMXDS format 1 (without MaxRdTurn) and format 2 (with MaxRdTurn). This bit is used to return the 2-byte format 1 (MaxWr, MaxRd) or 5-byte format 2 (MaxWr, MaxRd, 3-byte MaxRdTurn) byte in response to the GETCAPS CCC. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=0 and LSB=RDTURN\[7:0\]. - Max read turnaround time is less than 256 �s. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=RDTURN\[7:0\]
    ///and LSB=0. - Max read turnaround time is between 256 �s and 65535 �s - 3-byte MaxRdTurn is returned with MSB=RDTURN\[7:0\], middle byte=0 and LSB=0. - Max read turnaround time is between 65535 �s and 16 s.
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:23 - programmed byte of the 3-byte MaxRdTurn (maximum read turnaround byte) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and writes the value of the selected byte (via the FMT\[1:0\]
    ///field) of the 3-byte MaxRdTurn which is returned in response to the GETMXDS CCC format 2 to encode the maximum read turnaround time.
    #[inline(always)]
    pub fn rdturn(&self) -> RDTURN_R {
        RDTURN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - clock-to-data turnaround time (tSCO) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and is used to specify the clock-to-data turnaround time tSCO (vs the value of 12 ns). This bit is used by the hardware in response to the GETMXDS CCC to return the encoded clock-to-data turnaround time via the returned MaxRd\[5:3\]
    ///bits.
    #[inline(always)]
    pub fn tsco(&self) -> TSCO_R {
        TSCO_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - controller hand-off activity state This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates in which initial activity state the (other) current controller should expect the I3C bus after a controller-role hand-off to this controller-capable I3C, when returning the defining byte CRHDLY (0x91) to a GETMXDS CCC. This 2-bit field is used to return the CRHDLY1 byte in response to the GETCAPS CCC format 3, in order to state which is the activity state of this I3C when becoming controller after a controller-role hand-off, and consequently the time the former controller should wait before testing this I3C to be confirmed its ownership.
    #[inline(always)]
    #[must_use]
    pub fn hoffas(&mut self) -> HOFFAS_W<0> {
        HOFFAS_W::new(self)
    }
    ///Bits 8:9 - GETMXDS CCC format This field is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates how is returned the GETMXDS format 1 (without MaxRdTurn) and format 2 (with MaxRdTurn). This bit is used to return the 2-byte format 1 (MaxWr, MaxRd) or 5-byte format 2 (MaxWr, MaxRd, 3-byte MaxRdTurn) byte in response to the GETCAPS CCC. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=0 and LSB=RDTURN\[7:0\]. - Max read turnaround time is less than 256 �s. - 3-byte MaxRdTurn is returned with MSB=0, middle byte=RDTURN\[7:0\]
    ///and LSB=0. - Max read turnaround time is between 256 �s and 65535 �s - 3-byte MaxRdTurn is returned with MSB=RDTURN\[7:0\], middle byte=0 and LSB=0. - Max read turnaround time is between 65535 �s and 16 s.
    #[inline(always)]
    #[must_use]
    pub fn fmt(&mut self) -> FMT_W<8> {
        FMT_W::new(self)
    }
    ///Bits 16:23 - programmed byte of the 3-byte MaxRdTurn (maximum read turnaround byte) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and writes the value of the selected byte (via the FMT\[1:0\]
    ///field) of the 3-byte MaxRdTurn which is returned in response to the GETMXDS CCC format 2 to encode the maximum read turnaround time.
    #[inline(always)]
    #[must_use]
    pub fn rdturn(&mut self) -> RDTURN_W<16> {
        RDTURN_W::new(self)
    }
    ///Bit 24 - clock-to-data turnaround time (tSCO) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and is used to specify the clock-to-data turnaround time tSCO (vs the value of 12 ns). This bit is used by the hardware in response to the GETMXDS CCC to return the encoded clock-to-data turnaround time via the returned MaxRd\[5:3\]
    ///bits.
    #[inline(always)]
    #[must_use]
    pub fn tsco(&mut self) -> TSCO_W<24> {
        TSCO_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///I3C get capability register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [i3c_getmxdsr](index.html) module
pub struct I3C_GETMXDSR_SPEC;
impl crate::RegisterSpec for I3C_GETMXDSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [i3c_getmxdsr::R](R) reader structure
impl crate::Readable for I3C_GETMXDSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [i3c_getmxdsr::W](W) writer structure
impl crate::Writable for I3C_GETMXDSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets I3C_GETMXDSR to value 0
impl crate::Resettable for I3C_GETMXDSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
