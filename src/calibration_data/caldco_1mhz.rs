#[doc = "Register `CALDCO_1MHZ` reader"]
pub type R = crate::R<Caldco1mhzSpec>;
#[doc = "Register `CALDCO_1MHZ` writer"]
pub type W = crate::W<Caldco1mhzSpec>;
#[doc = "Field `CALDCO_1MHZ` reader - Value for the DCOCTL register for 1 MHz"]
pub type Caldco1mhzR = crate::FieldReader;
#[doc = "Field `CALDCO_1MHZ` writer - Value for the DCOCTL register for 1 MHz"]
pub type Caldco1mhzW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - Value for the DCOCTL register for 1 MHz"]
    #[inline(always)]
    pub fn caldco_1mhz(&self) -> Caldco1mhzR {
        Caldco1mhzR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Value for the DCOCTL register for 1 MHz"]
    #[inline(always)]
    pub fn caldco_1mhz(&mut self) -> Caldco1mhzW<'_, Caldco1mhzSpec> {
        Caldco1mhzW::new(self, 0)
    }
}
#[doc = "DCOCTL Calibration Data for 1MHz\n\nYou can [`read`](crate::Reg::read) this register and get [`caldco_1mhz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caldco_1mhz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Caldco1mhzSpec;
impl crate::RegisterSpec for Caldco1mhzSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`caldco_1mhz::R`](R) reader structure"]
impl crate::Readable for Caldco1mhzSpec {}
#[doc = "`write(|w| ..)` method takes [`caldco_1mhz::W`](W) writer structure"]
impl crate::Writable for Caldco1mhzSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CALDCO_1MHZ to value 0"]
impl crate::Resettable for Caldco1mhzSpec {}
