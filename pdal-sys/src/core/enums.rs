// MIT License
//
// Copyright (c) 2024 NUVIEW, Inc. <simeon.fitch@nuview.space>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge, publish, distribute,
// sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT
// OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

// NB: this was hand copied. In the future we should use `bindgen` to ensure future compatibility
#[derive(Clone, Copy, PartialEq, Eq, Default, Hash)]
#[repr(C)]
#[non_exhaustive]
pub enum DimTypeId {
    #[default]
    Unknown,
    X,
    Y,
    Z,
    W,
    Intensity,
    Amplitude,
    Reflectance,
    ReturnNumber,
    NumberOfReturns,
    ScanDirectionFlag,
    EdgeOfFlightLine,
    Classification,
    ScanAngleRank,
    UserData,
    PointSourceId,
    Red,
    Green,
    Blue,
    GpsTime,
    InternalTime,
    OffsetTime,
    IsPpsLocked,
    StartPulse,
    ReflectedPulse,
    Pdop,
    Pitch,
    Roll,
    PulseWidth,
    Deviation,
    PassiveSignal,
    BackgroundRadiation,
    PassiveX,
    PassiveY,
    PassiveZ,
    XVelocity,
    YVelocity,
    ZVelocity,
    Azimuth,
    WanderAngle,
    XBodyAccel,
    YBodyAccel,
    ZBodyAccel,
    XBodyAngRate,
    YBodyAngRate,
    ZBodyAngRate,
    Flag,
    Mark,
    Alpha,
    EchoRange,
    ScanChannel,
    Infrared,
    HeightAboveGround,
    ClassFlags,
    Synthetic,
    KeyPoint,
    Withheld,
    Overlap,
    LvisLfid,
    ShotNumber,
    LongitudeCentroid,
    LatitudeCentroid,
    ElevationCentroid,
    LongitudeLow,
    LatitudeLow,
    ElevationLow,
    LongitudeHigh,
    LatitudeHigh,
    ElevationHigh,
    PointId,
    OriginId,
    NormalX,
    NormalY,
    NormalZ,
    Curvature,
    Density,
    Omit,
    ClusterID,
    NNDistance,
    TextureU,
    TextureV,
    TextureW,
    Linearity,
    Planarity,
    Scattering,
    Verticality,
    Omnivariance,
    Anisotropy,
    Eigenentropy,
    EigenvalueSum,
    SurfaceVariation,
    DemantkeVerticality,
    OptimalKNN,
    OptimalRadius,
    Coplanar,
    LocalReachabilityDistance,
    LocalOutlierFactor,
    Miniball,
    Reciprocity,
    Rank,
    Eigenvalue0,
    Eigenvalue1,
    Eigenvalue2,
    PlaneFit,
    RadialDensity,
    BeamOriginX,
    BeamOriginY,
    BeamOriginZ,
    BeamDirectionX,
    BeamDirectionY,
    BeamDirectionZ,
    NorthPositionRMS,
    EastPositionRMS,
    DownPositionRMS,
    NorthVelocityRMS,
    EastVelocityRMS,
    DownVelocityRMS,
    RollRMS,
    PitchRMS,
    HeadingRMS,
    Reliability,
    EchoPos,
    EchoNorm,
    ImgNbr,
    Image,
    Dimension,
    SphericalRange,
    SphericalAzimuth,
    SphericalElevation,
}

unsafe impl cxx::ExternType for DimTypeId {
    type Id = cxx::type_id!("pdal_sys::core::DimTypeId");
    type Kind = cxx::kind::Trivial;
}

// NB: this was hand copied. Ordinals are from pdal::Dimension::Type, and are
// compile-time computed.
// In the future we should use `bindgen` to ensure stable compatibility
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)]
pub enum DimTypeEncoding {
    #[default]
    None = 0,
    Unsigned8 = 513,
    Signed8 = 257,
    Unsigned16 = 514,
    Signed16 = 258,
    Unsigned32 = 516,
    Signed32 = 260,
    Unsigned64 = 520,
    Signed64 = 264,
    Float = 1028,
    Double = 1032,
}

/// Enables copy by value
unsafe impl cxx::ExternType for DimTypeEncoding {
    type Id = cxx::type_id!("pdal_sys::core::DimTypeEncoding");
    type Kind = cxx::kind::Trivial;
}

/// Discriminated union of instances of PDAL primitive types.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PdalValue {
    None,
    Unsigned8(u8),
    Signed8(i8),
    Unsigned16(u16),
    Signed16(i16),
    Unsigned32(u32),
    Signed32(i32),
    Unsigned64(u64),
    Signed64(i64),
    Float(f32),
    Double(f64),
}

pub trait PdalType: Sized {
    /// Get the corresponding PDAL datatype encoding to this type.
    fn encoding() -> DimTypeEncoding;

    /// Convert into a wrapped PdalValue
    fn into_pdal_value(self) -> PdalValue;

    /// Convert dynamic type to static type when logically known.
    /// Returns `None` if given value isn't actually the <u>exact</u> same
    /// type as encoding.
    fn static_cast<T: PdalType>(value: T) -> Option<Self> {
        if Self::encoding() == T::encoding() {
            Some(unsafe { std::mem::transmute_copy::<T, Self>(&value) })
        } else {
            None
        }
    }
}

impl PdalType for PdalValue {
    fn encoding() -> DimTypeEncoding {
        DimTypeEncoding::None
    }

    fn into_pdal_value(self) -> PdalValue {
        self
    }
}

macro_rules! impl_pdal_type {
    ($t:ty, $enc:ident) => {
        impl PdalType for $t {
            fn encoding() -> DimTypeEncoding {
                DimTypeEncoding::$enc
            }
            fn into_pdal_value(self) -> PdalValue {
                PdalValue::$enc(self)
            }
        }
    };
    () => {};
}

impl_pdal_type!(i8, Signed8);
impl_pdal_type!(u8, Unsigned8);
impl_pdal_type!(i16, Signed16);
impl_pdal_type!(u16, Unsigned16);
impl_pdal_type!(i32, Signed32);
impl_pdal_type!(u32, Unsigned32);
impl_pdal_type!(i64, Signed64);
impl_pdal_type!(u64, Unsigned64);
impl_pdal_type!(f32, Float);
impl_pdal_type!(f64, Double);
