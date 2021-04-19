#[doc = "Register `SCLL` reader"]
pub struct R(crate::R<SCLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SCLL_SPEC>> for R {
    fn from(reader: crate::R<SCLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCLL` writer"]
pub struct W(crate::W<SCLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCLL_SPEC>;
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
impl core::convert::From<crate::W<SCLL_SPEC>> for W {
    fn from(writer: crate::W<SCLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCLL` reader - Count for SCL low time period selection."]
pub struct SCLL_R(crate::FieldReader<u16, u16>);
impl SCLL_R {
    pub(crate) fn new(bits: u16) -> Self {
        SCLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLL` writer - Count for SCL low time period selection."]
pub struct SCLL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Count for SCL low time period selection."]
    #[inline(always)]
    pub fn scll(&self) -> SCLL_R {
        SCLL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Count for SCL low time period selection."]
    #[inline(always)]
    pub fn scll(&mut self) -> SCLL_W {
        SCLL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCL Duty Cycle Register Low Half Word. Determines the low time of the I2C clock. I2nSCLL and I2nSCLH together determine the clock frequency generated by an I2C master and certain times used in slave mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scll](index.html) module"]
pub struct SCLL_SPEC;
impl crate::RegisterSpec for SCLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scll::R](R) reader structure"]
impl crate::Readable for SCLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scll::W](W) writer structure"]
impl crate::Writable for SCLL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCLL to value 0x04"]
impl crate::Resettable for SCLL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
