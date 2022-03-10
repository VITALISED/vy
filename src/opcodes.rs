#[derive(PartialEq)]
pub enum VyToken {
    TkIdentifier = 0,

    TkBoolSig = 1,
    TkByteSig = 2,
    TkIntSig = 3,
    TkLongSig = 4, // unused
    TkFloatSig = 5,
    TkDoubleSig = 6,
    TkCharSig = 7,
    TkVoidSig = 8,
    TkClosureSig = 9,
    TkClassSig = 10,
    TkArraySig = 11,
    TkTableSig = 12,
    TkEnumSig = 13,

    TkIf = 14,
    TkElse = 15,
    TkWhile = 16,
    TkFor = 17,
    TkIn = 18,
    TkNotIn = 19,
    TkForEach = 20,
    TkReturn = 21,
    TkBreak = 22,
    TkContinue = 23,
    TkImport = 24,
    TkTypeof = 25,
    TkNew = 26,
    TkDelete = 27,
    TkNull = 28,
    TkYield = 29,
    TkThis = 30,
    TkSuper = 31,
    TkResume = 32,
    TkThrow = 33,
    TkTry = 34,
    TkCatch = 35,
    TkSwitch = 36,
    TkCase = 37,
    TkDefault = 38,
    TkInstanceof = 39,
    TkConstructor = 40,
    TkStatic = 41,
    TkPublic = 42,
    TkPrivate = 43,
    TkExtends = 44,
    TkConst = 45,
    TkJSON = 46,
    TkAsync = 47,
    TkAwait = 48,
    TkThread = 49,
    TkGenerator = 50,

    TkPlus = 51,
    TkMinus = 52,
    TkMultiply = 53,
    TkDivide = 54,
    TkModulo = 55,
    TkPlusPlus = 56,
    TkMinusMinus = 57,
    TkEquals = 58,
    TkGreaterEquals = 59,
    TkLesserEquals = 60,
    TkNewSlot = 61,
    TkLesser = 62,
    TkGreater = 63,
    TkEqualsEquals = 64,
    TkPlusEq = 65,
    TkMinusEq = 66,
    TkMultiplyEq = 67,
    TkDivideEq = 68,
    TkModuloEq = 69,
    TkAnd = 70,
    TkOr = 71,
    TkShiftRight = 72,
    TkShiftLeft = 73,
    TkNotEquals = 74,

    TkString = 75,
    TkInteger = 76,
    TkFloat = 77,
    TkTrue = 78,
    TkFalse = 79,
    TkEOF = 80,
}

pub enum VyInstructions {

}

pub enum VyCompilerFlag {

}

