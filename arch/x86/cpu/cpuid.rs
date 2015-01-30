use core::str::from_utf8_unchecked;
use core::mem;

static VENDOR_OLDAMD:		&'static str = "AMDisbetter!";
static VENDOR_AMD:			&'static str = "AuthenticAMD";
static VENDOR_INTEL:		&'static str = "GenuineIntel";
static VENDOR_VIA:			&'static str = "CentaurHauls";
static VENDOR_OLDTRANSMETA:	&'static str = "TransmetaCPU";
static VENDOR_TRANSMETA:	&'static str = "GenuineTMx86";
static VENDOR_CYRIX:		&'static str = "CyrixInstead";
static VENDOR_CENTAUR:		&'static str = "CentaurHauls";
static VENDOR_NEXGEN:		&'static str = "NexGenDriven";
static VENDOR_UMC:			&'static str = "UMC UMC UMC ";
static VENDOR_SIS:			&'static str = "SiS SiS SiS ";
static VENDOR_NSC:			&'static str = "Geode by NSC";
static VENDOR_RISE:			&'static str = "RiseRiseRise";

pub enum FeatureECX {
	SSE3		= 1 << 0,
	PCLMUL		= 1 << 1,
	DTES64		= 1 << 2,
	MONITOR		= 1 << 3,
	DS_CPL		= 1 << 4,
	VMX			= 1 << 5,
	SMX			= 1 << 6,
	EST			= 1 << 7,
	TM2			= 1 << 8,
	SSSE3		= 1 << 9,
	CID			= 1 << 10,
	FMA			= 1 << 12,
	CX16		= 1 << 13,
	ETPRD		= 1 << 14,
	PDCM		= 1 << 15,
	DCA			= 1 << 18,
	SSE4_1		= 1 << 19,
	SSE4_2		= 1 << 20,
	x2APIC		= 1 << 21,
	MOVBE		= 1 << 22,
	POPCNT		= 1 << 23,
	AES			= 1 << 25,
	XSAVE		= 1 << 26,
	OSXSAVE		= 1 << 27,
	AVX			= 1 << 28,
}

pub enum FeatureEDX {
	FPU		= 1 << 0,
	VME		= 1 << 1,
	DE		= 1 << 2,
	PSE		= 1 << 3,
	TSC		= 1 << 4,
	MSR		= 1 << 5,
	PAE		= 1 << 6,
	MCE		= 1 << 7,
	CX8		= 1 << 8,
	APIC	= 1 << 9,
	SEP		= 1 << 11,
	MTRR	= 1 << 12,
	PGE		= 1 << 13,
	MCA		= 1 << 14,
	CMOV	= 1 << 15,
	PAT		= 1 << 16,
	PSE36	= 1 << 17,
	PSN		= 1 << 18,
	CLF		= 1 << 19,
	DTES	= 1 << 21,
	ACPI	= 1 << 22,
	MMX		= 1 << 23,
	FXSR	= 1 << 24,
	SSE		= 1 << 25,
	SSE2	= 1 << 26,
	SS		= 1 << 27,
	HTT		= 1 << 28,
	TM1		= 1 << 29,
	IA64	= 1 << 30,
	PBE		= 1 << 31,
}

pub fn check_cpuid_support() -> bool {
	let mut res: u32 = 0;
	unsafe {
		asm!("
			pushfd
			pushfd
			xorl $$0x00200000, %esp
			popfd
			pushfd
			pop %eax
			xorl %eax, %esp
			popfd
			andl $$0x00200000, eax
			movl $0, %eax"
			: "=r"(res)
			:
			: "eax", "esp"
		);
	}
	if res == 0 {
		false
	} else {
		true
	}
}

pub fn get_vendor() -> &'static str {
	let mut ebx = 0u32;
	let mut ecx = 0u32;
	let mut edx = 0u32;

	unsafe {
		asm!("
			movl $$0x0, eax
			cpuid
			movl $0, ebx
			movl $1, ecx
			movl $2, edx"
			: "=r"(ebx), "=r"(ecx), "=r"(edx)
			:
			: "eax"
		);
	}

	let bytes: &'static mut [u8; 12] = unsafe { mem::zeroed() };
	bytes[0] 	= ((ebx >> 24) & 0xFF) as u8;
	bytes[1] 	= ((ebx >> 16) & 0xFF) as u8;
	bytes[2] 	= ((ebx >> 8) & 0xFF) as u8;
	bytes[3] 	= (ebx & 0xFF) as u8;
	bytes[4] 	= ((ecx >> 24) & 0xFF) as u8;
	bytes[5] 	= ((ecx >> 16) & 0xFF) as u8;
	bytes[6] 	= ((ecx >> 8) & 0xFF) as u8;
	bytes[7] 	= (ecx & 0xFF) as u8;
	bytes[8] 	= ((edx >> 24) & 0xFF) as u8;
	bytes[9] 	= ((edx >> 16) & 0xFF) as u8;
	bytes[10] 	= ((edx >> 8) & 0xFF) as u8;
	bytes[11] 	= (edx & 0xFF) as u8;

	unsafe {
		from_utf8_unchecked(bytes)
	}
}

pub fn get_features() -> (u32, u32) {
	let mut ecx = 0u32;
	let mut edx = 0u32;
	unsafe {
		asm!("
			movl $$0x1, eax
			cpuid
			movl $0, ecx
			movl $1, edx"
			: "=r"(ecx), "=r"(edx)
			:
			: "eax"
		);
	}

	(ecx, edx)
}

pub trait IsFeatureEnum {
	fn is_feature_supported(self) -> bool;
}

impl IsFeatureEnum for FeatureECX {
	fn is_feature_supported(self) -> bool {
		let (ecx, _) = get_features();

		if ecx & (self as u32) > 0 {
			true
		} else {
			false
		}
	}
}

impl IsFeatureEnum for FeatureEDX {
	fn is_feature_supported(self) -> bool {
		let (_, edx) = get_features();

		if edx & (self as u32) > 0 {
			true
		} else {
			false
		}
	}
}