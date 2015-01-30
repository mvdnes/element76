use platform::cpu::cpuid;
use platform::cpu::cpuid::IsFeatureEnum;


pub fn check_pae_support() -> bool {
	if cpuid::check_cpuid_support() {
		cpuid::FeatureEDX::PAE.is_feature_supported()
	} else {
		false
	}
}