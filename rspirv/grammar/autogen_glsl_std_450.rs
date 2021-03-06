// AUTOMATICALLY GENERATED from the SPIR-V JSON grammar:
//   external/spirv.core.grammar.json.
// DO NOT MODIFY!

static GLSL_STD_450_INSTRUCTION_TABLE: &[ExtendedInstruction<'static>] = &[
    ext_inst!(Round, 1u32, [], [], [(IdRef, One)]),
    ext_inst!(RoundEven, 2u32, [], [], [(IdRef, One)]),
    ext_inst!(Trunc, 3u32, [], [], [(IdRef, One)]),
    ext_inst!(FAbs, 4u32, [], [], [(IdRef, One)]),
    ext_inst!(SAbs, 5u32, [], [], [(IdRef, One)]),
    ext_inst!(FSign, 6u32, [], [], [(IdRef, One)]),
    ext_inst!(SSign, 7u32, [], [], [(IdRef, One)]),
    ext_inst!(Floor, 8u32, [], [], [(IdRef, One)]),
    ext_inst!(Ceil, 9u32, [], [], [(IdRef, One)]),
    ext_inst!(Fract, 10u32, [], [], [(IdRef, One)]),
    ext_inst!(Radians, 11u32, [], [], [(IdRef, One)]),
    ext_inst!(Degrees, 12u32, [], [], [(IdRef, One)]),
    ext_inst!(Sin, 13u32, [], [], [(IdRef, One)]),
    ext_inst!(Cos, 14u32, [], [], [(IdRef, One)]),
    ext_inst!(Tan, 15u32, [], [], [(IdRef, One)]),
    ext_inst!(Asin, 16u32, [], [], [(IdRef, One)]),
    ext_inst!(Acos, 17u32, [], [], [(IdRef, One)]),
    ext_inst!(Atan, 18u32, [], [], [(IdRef, One)]),
    ext_inst!(Sinh, 19u32, [], [], [(IdRef, One)]),
    ext_inst!(Cosh, 20u32, [], [], [(IdRef, One)]),
    ext_inst!(Tanh, 21u32, [], [], [(IdRef, One)]),
    ext_inst!(Asinh, 22u32, [], [], [(IdRef, One)]),
    ext_inst!(Acosh, 23u32, [], [], [(IdRef, One)]),
    ext_inst!(Atanh, 24u32, [], [], [(IdRef, One)]),
    ext_inst!(Atan2, 25u32, [], [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(Pow, 26u32, [], [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(Exp, 27u32, [], [], [(IdRef, One)]),
    ext_inst!(Log, 28u32, [], [], [(IdRef, One)]),
    ext_inst!(Exp2, 29u32, [], [], [(IdRef, One)]),
    ext_inst!(Log2, 30u32, [], [], [(IdRef, One)]),
    ext_inst!(Sqrt, 31u32, [], [], [(IdRef, One)]),
    ext_inst!(InverseSqrt, 32u32, [], [], [(IdRef, One)]),
    ext_inst!(Determinant, 33u32, [], [], [(IdRef, One)]),
    ext_inst!(MatrixInverse, 34u32, [], [], [(IdRef, One)]),
    ext_inst!(Modf, 35u32, [], [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(ModfStruct, 36u32, [], [], [(IdRef, One)]),
    ext_inst!(FMin, 37u32, [], [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(UMin, 38u32, [], [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(SMin, 39u32, [], [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(FMax, 40u32, [], [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(UMax, 41u32, [], [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(SMax, 42u32, [], [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(
        FClamp,
        43u32,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        UClamp,
        44u32,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        SClamp,
        45u32,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        FMix,
        46u32,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        IMix,
        47u32,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(Step, 48u32, [], [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(
        SmoothStep,
        49u32,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        Fma,
        50u32,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(Frexp, 51u32, [], [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(FrexpStruct, 52u32, [], [], [(IdRef, One)]),
    ext_inst!(Ldexp, 53u32, [], [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(PackSnorm4x8, 54u32, [], [], [(IdRef, One)]),
    ext_inst!(PackUnorm4x8, 55u32, [], [], [(IdRef, One)]),
    ext_inst!(PackSnorm2x16, 56u32, [], [], [(IdRef, One)]),
    ext_inst!(PackUnorm2x16, 57u32, [], [], [(IdRef, One)]),
    ext_inst!(PackHalf2x16, 58u32, [], [], [(IdRef, One)]),
    ext_inst!(PackDouble2x32, 59u32, [Float64], [], [(IdRef, One)]),
    ext_inst!(UnpackSnorm2x16, 60u32, [], [], [(IdRef, One)]),
    ext_inst!(UnpackUnorm2x16, 61u32, [], [], [(IdRef, One)]),
    ext_inst!(UnpackHalf2x16, 62u32, [], [], [(IdRef, One)]),
    ext_inst!(UnpackSnorm4x8, 63u32, [], [], [(IdRef, One)]),
    ext_inst!(UnpackUnorm4x8, 64u32, [], [], [(IdRef, One)]),
    ext_inst!(UnpackDouble2x32, 65u32, [Float64], [], [(IdRef, One)]),
    ext_inst!(Length, 66u32, [], [], [(IdRef, One)]),
    ext_inst!(Distance, 67u32, [], [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(Cross, 68u32, [], [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(Normalize, 69u32, [], [], [(IdRef, One)]),
    ext_inst!(
        FaceForward,
        70u32,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(Reflect, 71u32, [], [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(
        Refract,
        72u32,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
    ext_inst!(FindILsb, 73u32, [], [], [(IdRef, One)]),
    ext_inst!(FindSMsb, 74u32, [], [], [(IdRef, One)]),
    ext_inst!(FindUMsb, 75u32, [], [], [(IdRef, One)]),
    ext_inst!(
        InterpolateAtCentroid,
        76u32,
        [InterpolationFunction],
        [],
        [(IdRef, One)]
    ),
    ext_inst!(
        InterpolateAtSample,
        77u32,
        [InterpolationFunction],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(
        InterpolateAtOffset,
        78u32,
        [InterpolationFunction],
        [],
        [(IdRef, One), (IdRef, One)]
    ),
    ext_inst!(NMin, 79u32, [], [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(NMax, 80u32, [], [], [(IdRef, One), (IdRef, One)]),
    ext_inst!(
        NClamp,
        81u32,
        [],
        [],
        [(IdRef, One), (IdRef, One), (IdRef, One)]
    ),
];
