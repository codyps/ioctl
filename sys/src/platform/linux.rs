
const WATCHDOG_IOCTL_BASE: u32 = b'W' as u32;
const BASE_VIDIOC_PRIVATE: u32 = 192;
const USBTMC_IOC_NR: u32 = 91;
const UBI_VOL_IOC_MAGIC: u32 = b'O' as u32;
const UBI_IOC_MAGIC: u32 = b'o' as u32;
const UBI_CTRL_IOC_MAGIC: u32 = b'o' as u32;
const SPI_IOC_MAGIC: u32 = b'k' as u32;
const MGSL_MAGIC_IOC: u32 = b'm' as u32;
const DRM_IOCTL_BASE: u32 = b'd' as u32;
const DRM_COMMAND_BASE: u32 = 0x40;
const BTRFS_IOCTL_MAGIC: u32 = 0x94;
const BTRFS_LABEL_SIZE: usize = 256;
//const SIOCPARM_MASK: u32 = 0x1fff;

const ATMIOC_ITF: u32 = 0x80;
const ATMIOC_CLIP: u32 = 0xe0;
const RFKILL_IOC_NOINPUT: u32 = 1;
const RFKILL_IOC_MAGIC: u32 = b'R' as u32;
const IFNAMSIZ: usize = 16;
const DECNET_IOCTL_BASE: u32 = 0x89;
const ABS_MAX: usize = 0x3f;
const ABS_CNT: usize = ABS_MAX+1;
const CXL_MAGIC: u32 = 0xCA;
const CM_IOC_MAGIC: u32 = b'c' as u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct input_event {
    pub time: ::libc::timeval,
    pub _type: u16,
    pub code: u16,
    pub value: i32,
}
impl ::std::default::Default for input_event {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
impl ::std::fmt::Debug for input_event {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "input_event {{ time: {{ tv_sec: {}, tv_usec: {} }}, _type: {}, code: {}, value: {}",
               self.time.tv_sec, self.time.tv_usec, self._type, self.code, self.value)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct input_id {
    pub bustype: u16,
    pub vendor: u16,
    pub product: u16,
    pub version: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ff_effect {
    pub _type: u16,
    pub id: i16,
    pub direction: u16,
    pub trigger: ff_trigger,
    pub replay: ff_replay,
    pub u: Union_Unnamed16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Union_Unnamed16 {
    pub _bindgen_data_: [u64; 4usize],
}
impl Union_Unnamed16 {
    pub unsafe fn constant(&mut self) -> *mut ff_constant_effect {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn ramp(&mut self) -> *mut ff_ramp_effect {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn periodic(&mut self) -> *mut ff_periodic_effect {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn condition(&mut self)
     -> *mut [ff_condition_effect; 2usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn rumble(&mut self) -> *mut ff_rumble_effect {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::default::Default for Union_Unnamed16 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct input_absinfo {
    pub value: i32,
    pub minimum: i32,
    pub maximum: i32,
    pub fuzz: i32,
    pub flat: i32,
    pub resolution: i32,
}
impl ::std::default::Default for input_absinfo {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct input_keymap_entry {
    pub flags: u8,
    pub len: u8,
    pub index: u16,
    pub keycode: u32,
    pub scancode: [u8; 32usize],
}
impl ::std::default::Default for input_keymap_entry {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ff_replay {
    pub length: u16,
    pub delay: u16,
}
impl ::std::default::Default for ff_replay {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ff_trigger {
    pub button: u16,
    pub interval: u16,
}
impl ::std::default::Default for ff_trigger {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ff_envelope {
    pub attack_length: u16,
    pub attack_level: u16,
    pub fade_length: u16,
    pub fade_level: u16,
}
impl ::std::default::Default for ff_envelope {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ff_constant_effect {
    pub level: i16,
    pub envelope: ff_envelope,
}
impl ::std::default::Default for ff_constant_effect {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ff_ramp_effect {
    pub start_level: i16,
    pub end_level: i16,
    pub envelope: ff_envelope,
}
impl ::std::default::Default for ff_ramp_effect {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ff_condition_effect {
    pub right_saturation: u16,
    pub left_saturation: u16,
    pub right_coeff: i16,
    pub left_coeff: i16,
    pub deadband: u16,
    pub center: i16,
}
impl ::std::default::Default for ff_condition_effect {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ff_periodic_effect {
    pub waveform: u16,
    pub period: u16,
    pub magnitude: i16,
    pub offset: i16,
    pub phase: u16,
    pub envelope: ff_envelope,
    pub custom_len: u32,
    pub custom_data: *mut i16,
}
impl ::std::default::Default for ff_periodic_effect {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ff_rumble_effect {
    pub strong_magnitude: u16,
    pub weak_magnitude: u16,
}
impl ::std::default::Default for ff_rumble_effect {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

ioctl!(read buf eviocgname with b'E', 0x06; u8);
ioctl!(read buf eviocgphys with b'E', 0x07; u8);
ioctl!(read buf eviocguniq with b'E', 0x08; u8);
ioctl!(read buf eviocgprop with b'E', 0x09; u8);
ioctl!(read buf eviocgmtslots with b'E', 0x0a; u8);
ioctl!(read buf eviocgkey with b'E', 0x18; u8);
ioctl!(read buf eviocgled with b'E', 0x19; u8);
ioctl!(read buf eviocgsnd with b'E', 0x1a; u8);
ioctl!(read buf eviocgsw with b'E', 0x1b; u8);

ioctl!(write eviocsff with b'E', 0x80; ff_effect);
ioctl!(write eviocgrab with b'E', 0x90; ::std::os::raw::c_int);
ioctl!(write eviocrevoke with b'E', 0x91; ::std::os::raw::c_int);
ioctl!(write eviocsclockid with b'E', 0xa0; ::std::os::raw::c_int);

pub unsafe fn eviocgbit(fd: ::std::os::raw::c_int, ev: u32, len: ::std::os::raw::c_int, buf: *mut u8) -> ::std::os::raw::c_int {
    ::ioctl(fd, ior!(b'E', 0x20 + ev, len) as ::std::os::raw::c_ulong, buf)
}

pub unsafe fn eviocgabs(fd: ::std::os::raw::c_int, abs: u32, buf: *mut input_absinfo) -> ::std::os::raw::c_int {
    ::ioctl(fd, ior!(b'E', 0x40 + abs, ::std::mem::size_of::<input_absinfo>()) as ::std::os::raw::c_ulong, buf)
}

ioctl!(bad write blkroset with io!(0x12, 93); ::std::os::raw::c_int);
ioctl!(bad read blkroget with io!(0x12, 94); ::std::os::raw::c_int);
ioctl!(none blkrrpart with 0x12, 95);
ioctl!(bad read blkgetsize with io!(0x12, 96); ::std::os::raw::c_ulong);
ioctl!(none blkflsbuf with 0x12, 97);
ioctl!(arg  blkraset with 0x12, 98);
ioctl!(bad read blkraget with io!(0x12, 99); ::std::os::raw::c_long);
ioctl!(arg  blkfraset with 0x12, 100);
ioctl!(bad read blkfraget with io!(0x12, 101); ::std::os::raw::c_long);
// ioctl!(bad blksectset with io!(0x12, 102));
ioctl!(bad read blksectget with io!(0x12, 103); ::std::os::raw::c_ushort);
ioctl!(bad read blksszget with io!(0x12, 104); ::std::os::raw::c_int);
ioctl!(bad read blkbszget with ior!(0x12, 112, ::std::mem::size_of::<::libc::size_t>()); ::std::os::raw::c_int);
ioctl!(bad write blkbszset with iow!(0x12, 113, ::std::mem::size_of::<::libc::size_t>()); ::std::os::raw::c_int);
ioctl!(bad read blkgetsize64 with ior!(0x12, 114, ::std::mem::size_of::<::libc::size_t>()); u64);
//ioctl!(readwrite blktracesetup with 0x12, 115; blk_user_trace_setup);
ioctl!(none blktracestart with 0x12, 116);
ioctl!(none blktracestop with 0x12, 117);
ioctl!(none blktraceteardown with 0x12, 118);
ioctl!(bad write blkdiscard with io!(0x12, 119); [u64; 2]);
ioctl!(bad read blkiomin with io!(0x12, 120); ::std::os::raw::c_uint);
ioctl!(bad read blkioopt with io!(0x12, 121); ::std::os::raw::c_uint);
ioctl!(bad read blkalignoff with io!(0x12, 122); ::std::os::raw::c_int);
ioctl!(bad read blkpbszget with io!(0x12, 123); ::std::os::raw::c_uint);
ioctl!(bad read blkdiscardzeros with io!(0x12, 124); ::std::os::raw::c_uint);
ioctl!(bad write blksecdiscard with io!(0x12, 125); [u64; 2]);
ioctl!(bad read blkrotational with io!(0x12, 126); ::std::os::raw::c_ushort);
ioctl!(bad write blkzeroout with io!(0x12, 127); [u64; 2]);

#[cfg(target_arch = "x86_64")]
include!("linux-generated-x86_64.rs");
