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

// NB: this was hand copied. In the future we should use `bindgen` to ensure true compatibility
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DimTypeId {
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

impl Default for DimTypeId {
    fn default() -> Self {
        Self::Unknown
    }
}
