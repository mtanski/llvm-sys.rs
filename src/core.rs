//! The LLVM intermediate representation.

use super::*;
use super::prelude::*;

// Core
extern "C" {
    pub fn LLVMShutdown() -> ();
    pub fn LLVMCreateMessage(Message: *const ::libc::c_char) -> *mut ::libc::c_char;
    pub fn LLVMDisposeMessage(Message: *mut ::libc::c_char) -> ();
    pub fn LLVMInstallFatalErrorHandler(Handler: LLVMFatalErrorHandler) -> ();
    pub fn LLVMResetFatalErrorHandler() -> ();
    pub fn LLVMEnablePrettyStackTrace() -> ();
}

// Core->Contexts
extern "C" {
    pub fn LLVMContextCreate() -> LLVMContextRef;
    pub fn LLVMGetGlobalContext() -> LLVMContextRef;
    pub fn LLVMContextSetDiagnosticHandler(C: LLVMContextRef,
                                           Handler: LLVMDiagnosticHandler,
                                           DiagnosticContext:
                                               *mut ::libc::c_void) -> ();
    pub fn LLVMContextSetYieldCallback(C: LLVMContextRef,
                                       Callback: LLVMYieldCallback,
                                       OpaqueHandle: *mut ::libc::c_void) -> ();
    pub fn LLVMContextDispose(C: LLVMContextRef) -> ();
    pub fn LLVMGetDiagInfoDescription(DI: LLVMDiagnosticInfoRef)
     -> *mut ::libc::c_char;
    pub fn LLVMGetDiagInfoSeverity(DI: LLVMDiagnosticInfoRef)
     -> LLVMDiagnosticSeverity;
    pub fn LLVMGetMDKindIDInContext(C: LLVMContextRef,
                                    Name: *const ::libc::c_char,
                                    SLen: ::libc::c_uint) -> ::libc::c_uint;
    pub fn LLVMGetMDKindID(Name: *const ::libc::c_char, SLen: ::libc::c_uint) -> ::libc::c_uint;
}

// Core->Modules
extern "C" {
    pub fn LLVMModuleCreateWithName(ModuleID: *const ::libc::c_char) -> LLVMModuleRef;
    pub fn LLVMModuleCreateWithNameInContext(ModuleID: *const ::libc::c_char,
                                             C: LLVMContextRef) -> LLVMModuleRef;
    pub fn LLVMCloneModule(M: LLVMModuleRef) -> LLVMModuleRef;
    pub fn LLVMDisposeModule(M: LLVMModuleRef) -> ();
    pub fn LLVMGetDataLayout(M: LLVMModuleRef) -> *const ::libc::c_char;
    pub fn LLVMSetDataLayout(M: LLVMModuleRef, Triple: *const ::libc::c_char) -> ();
    pub fn LLVMGetTarget(M: LLVMModuleRef) -> *const ::libc::c_char;
    pub fn LLVMSetTarget(M: LLVMModuleRef, Triple: *const ::libc::c_char) -> ();
    pub fn LLVMDumpModule(M: LLVMModuleRef) -> ();
    pub fn LLVMPrintModuleToFile(M: LLVMModuleRef,
                                 Filename: *const ::libc::c_char,
                                 ErrorMessage: *mut *mut ::libc::c_char) -> LLVMBool;
    pub fn LLVMPrintModuleToString(M: LLVMModuleRef) -> *mut ::libc::c_char;
    pub fn LLVMSetModuleInlineAsm(M: LLVMModuleRef,
                                  Asm: *const ::libc::c_char) -> ();
    pub fn LLVMGetModuleContext(M: LLVMModuleRef) -> LLVMContextRef;
    pub fn LLVMGetTypeByName(M: LLVMModuleRef, Name: *const ::libc::c_char) -> LLVMTypeRef;
    pub fn LLVMGetNamedMetadataNumOperands(M: LLVMModuleRef,
                                           name: *const ::libc::c_char) -> ::libc::c_uint;
    pub fn LLVMGetNamedMetadataOperands(M: LLVMModuleRef,
                                        name: *const ::libc::c_char,
                                        Dest: *mut LLVMValueRef) -> ();
    pub fn LLVMAddNamedMetadataOperand(M: LLVMModuleRef,
                                       name: *const ::libc::c_char,
                                       Val: LLVMValueRef) -> ();
    pub fn LLVMAddFunction(M: LLVMModuleRef, Name: *const ::libc::c_char,
                           FunctionTy: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMGetNamedFunction(M: LLVMModuleRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMGetFirstFunction(M: LLVMModuleRef) -> LLVMValueRef;
    pub fn LLVMGetLastFunction(M: LLVMModuleRef) -> LLVMValueRef;
    pub fn LLVMGetNextFunction(Fn: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMGetPreviousFunction(Fn: LLVMValueRef) -> LLVMValueRef;
}

// Core->Types
extern "C" {
    pub fn LLVMGetTypeKind(Ty: LLVMTypeRef) -> LLVMTypeKind;
    pub fn LLVMTypeIsSized(Ty: LLVMTypeRef) -> LLVMBool;
    pub fn LLVMGetTypeContext(Ty: LLVMTypeRef) -> LLVMContextRef;
    pub fn LLVMDumpType(Val: LLVMTypeRef) -> ();
    pub fn LLVMPrintTypeToString(Val: LLVMTypeRef) -> *mut ::libc::c_char;

    // Core->Types->Integer
    pub fn LLVMInt1TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMInt8TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMInt16TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMInt32TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMInt64TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMIntTypeInContext(C: LLVMContextRef, NumBits: ::libc::c_uint) -> LLVMTypeRef;
    pub fn LLVMInt1Type() -> LLVMTypeRef;
    pub fn LLVMInt8Type() -> LLVMTypeRef;
    pub fn LLVMInt16Type() -> LLVMTypeRef;
    pub fn LLVMInt32Type() -> LLVMTypeRef;
    pub fn LLVMInt64Type() -> LLVMTypeRef;
    pub fn LLVMIntType(NumBits: ::libc::c_uint) -> LLVMTypeRef;
    pub fn LLVMGetIntTypeWidth(IntegerTy: LLVMTypeRef) -> ::libc::c_uint;

    // Core->Types->Floating-Point
    pub fn LLVMHalfTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMFloatTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMDoubleTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMX86FP80TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMFP128TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMPPCFP128TypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMHalfType() -> LLVMTypeRef;
    pub fn LLVMFloatType() -> LLVMTypeRef;
    pub fn LLVMDoubleType() -> LLVMTypeRef;
    pub fn LLVMX86FP80Type() -> LLVMTypeRef;
    pub fn LLVMFP128Type() -> LLVMTypeRef;
    pub fn LLVMPPCFP128Type() -> LLVMTypeRef;

    // Core->Types->Function
    pub fn LLVMFunctionType(ReturnType: LLVMTypeRef,
                            ParamTypes: *mut LLVMTypeRef,
                            ParamCount: ::libc::c_uint, IsVarArg: LLVMBool) -> LLVMTypeRef;
    pub fn LLVMIsFunctionVarArg(FunctionTy: LLVMTypeRef) -> LLVMBool;
    pub fn LLVMGetReturnType(FunctionTy: LLVMTypeRef) -> LLVMTypeRef;
    pub fn LLVMCountParamTypes(FunctionTy: LLVMTypeRef) -> ::libc::c_uint;
    pub fn LLVMGetParamTypes(FunctionTy: LLVMTypeRef, Dest: *mut LLVMTypeRef) -> ();

    // Core->Types->Struct
    pub fn LLVMStructTypeInContext(C: LLVMContextRef,
                                   ElementTypes: *mut LLVMTypeRef,
                                   ElementCount: ::libc::c_uint,
                                   Packed: LLVMBool) -> LLVMTypeRef;
    pub fn LLVMStructType(ElementTypes: *mut LLVMTypeRef,
                          ElementCount: ::libc::c_uint, Packed: LLVMBool) -> LLVMTypeRef;
    pub fn LLVMStructCreateNamed(C: LLVMContextRef,
                                 Name: *const ::libc::c_char) -> LLVMTypeRef;
    pub fn LLVMGetStructName(Ty: LLVMTypeRef) -> *const ::libc::c_char;
    pub fn LLVMStructSetBody(StructTy: LLVMTypeRef,
                             ElementTypes: *mut LLVMTypeRef,
                             ElementCount: ::libc::c_uint, Packed: LLVMBool) -> ();
    pub fn LLVMCountStructElementTypes(StructTy: LLVMTypeRef) -> ::libc::c_uint;
    pub fn LLVMGetStructElementTypes(StructTy: LLVMTypeRef,
                                     Dest: *mut LLVMTypeRef) -> ();
    pub fn LLVMIsPackedStruct(StructTy: LLVMTypeRef) -> LLVMBool;
    pub fn LLVMIsOpaqueStruct(StructTy: LLVMTypeRef) -> LLVMBool;

    // Core->Types->Sequential
    pub fn LLVMGetElementType(Ty: LLVMTypeRef) -> LLVMTypeRef;
    pub fn LLVMArrayType(ElementType: LLVMTypeRef,
                         ElementCount: ::libc::c_uint) -> LLVMTypeRef;
    pub fn LLVMGetArrayLength(ArrayTy: LLVMTypeRef) -> ::libc::c_uint;
    pub fn LLVMPointerType(ElementType: LLVMTypeRef,
                           AddressSpace: ::libc::c_uint) -> LLVMTypeRef;
    pub fn LLVMGetPointerAddressSpace(PointerTy: LLVMTypeRef) -> ::libc::c_uint;
    pub fn LLVMVectorType(ElementType: LLVMTypeRef,
                          ElementCount: ::libc::c_uint) -> LLVMTypeRef;
    pub fn LLVMGetVectorSize(VectorTy: LLVMTypeRef) -> ::libc::c_uint;
    
    // Core->Types->Other
    pub fn LLVMVoidTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMLabelTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMX86MMXTypeInContext(C: LLVMContextRef) -> LLVMTypeRef;
    pub fn LLVMVoidType() -> LLVMTypeRef;
    pub fn LLVMLabelType() -> LLVMTypeRef;
    pub fn LLVMX86MMXType() -> LLVMTypeRef;
}

// Core->Values
extern "C" {
    // Core->Values->General
    pub fn LLVMTypeOf(Val: LLVMValueRef) -> LLVMTypeRef;
    pub fn LLVMGetValueName(Val: LLVMValueRef) -> *const ::libc::c_char;
    pub fn LLVMSetValueName(Val: LLVMValueRef, Name: *const ::libc::c_char)
     -> ();
    pub fn LLVMDumpValue(Val: LLVMValueRef) -> ();
    pub fn LLVMPrintValueToString(Val: LLVMValueRef) -> *mut ::libc::c_char;
    pub fn LLVMReplaceAllUsesWith(OldVal: LLVMValueRef, NewVal: LLVMValueRef)
     -> ();
    pub fn LLVMIsConstant(Val: LLVMValueRef) -> LLVMBool;
    pub fn LLVMIsUndef(Val: LLVMValueRef) -> LLVMBool;
    pub fn LLVMIsAMDNode(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAMDString(Val: LLVMValueRef) -> LLVMValueRef;

    // Core->Values->Usage
    pub fn LLVMGetFirstUse(Val: LLVMValueRef) -> LLVMUseRef;
    pub fn LLVMGetNextUse(U: LLVMUseRef) -> LLVMUseRef;
    pub fn LLVMGetUser(U: LLVMUseRef) -> LLVMValueRef;
    pub fn LLVMGetUsedValue(U: LLVMUseRef) -> LLVMValueRef;

    // Core->Values->User value
    pub fn LLVMGetOperand(Val: LLVMValueRef, Index: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMGetOperandUse(Val: LLVMValueRef, Index: ::libc::c_uint) -> LLVMUseRef;
    pub fn LLVMSetOperand(User: LLVMValueRef, Index: ::libc::c_uint,
                          Val: LLVMValueRef) -> ();
    pub fn LLVMGetNumOperands(Val: LLVMValueRef) -> ::libc::c_int;

    // Core->Values->Constants
    pub fn LLVMConstNull(Ty: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstAllOnes(Ty: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMGetUndef(Ty: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMIsNull(Val: LLVMValueRef) -> LLVMBool;
    pub fn LLVMConstPointerNull(Ty: LLVMTypeRef) -> LLVMValueRef;

    // Core->Values->Constants->Scalar
    pub fn LLVMConstInt(IntTy: LLVMTypeRef, N: ::libc::c_ulonglong,
                        SignExtend: LLVMBool) -> LLVMValueRef;
    pub fn LLVMConstIntOfArbitraryPrecision(IntTy: LLVMTypeRef,
                                            NumWords: ::libc::c_uint,
                                            Words: *const u64) -> LLVMValueRef;
    pub fn LLVMConstIntOfString(IntTy: LLVMTypeRef,
                                Text: *const ::libc::c_char, Radix: u8) -> LLVMValueRef;
    pub fn LLVMConstIntOfStringAndSize(IntTy: LLVMTypeRef,
                                       Text: *const ::libc::c_char,
                                       SLen: ::libc::c_uint, Radix: u8) -> LLVMValueRef;
    pub fn LLVMConstReal(RealTy: LLVMTypeRef, N: ::libc::c_double) -> LLVMValueRef;
    pub fn LLVMConstRealOfString(RealTy: LLVMTypeRef,
                                 Text: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMConstRealOfStringAndSize(RealTy: LLVMTypeRef,
                                        Text: *const ::libc::c_char,
                                        SLen: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMConstIntGetZExtValue(ConstantVal: LLVMValueRef) -> ::libc::c_ulonglong;
    pub fn LLVMConstIntGetSExtValue(ConstantVal: LLVMValueRef) -> ::libc::c_longlong;
    pub fn LLVMConstRealGetDouble(ConstantVal: LLVMValueRef,
                                  losesInfo: *mut LLVMBool) -> ::libc::c_double;

    // Core->Values->Constants->Composite
    pub fn LLVMConstStringInContext(C: LLVMContextRef,
                                    Str: *const ::libc::c_char,
                                    Length: ::libc::c_uint,
                                    DontNullTerminate: LLVMBool) -> LLVMValueRef;
    pub fn LLVMConstString(Str: *const ::libc::c_char, Length: ::libc::c_uint,
                           DontNullTerminate: LLVMBool) -> LLVMValueRef;
    pub fn LLVMIsConstantString(c: LLVMValueRef) -> LLVMBool;
    pub fn LLVMGetAsString(c: LLVMValueRef, out: *mut ::libc::size_t) -> *const ::libc::c_char;
    pub fn LLVMConstStructInContext(C: LLVMContextRef,
                                    ConstantVals: *mut LLVMValueRef,
                                    Count: ::libc::c_uint, Packed: LLVMBool) -> LLVMValueRef;
    pub fn LLVMConstStruct(ConstantVals: *mut LLVMValueRef,
                           Count: ::libc::c_uint, Packed: LLVMBool) -> LLVMValueRef;
    pub fn LLVMConstArray(ElementTy: LLVMTypeRef,
                          ConstantVals: *mut LLVMValueRef,
                          Length: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMConstNamedStruct(StructTy: LLVMTypeRef,
                                ConstantVals: *mut LLVMValueRef,
                                Count: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMGetElementAsConstant(c: LLVMValueRef, idx: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMConstVector(ScalarConstantVals: *mut LLVMValueRef,
                           Size: ::libc::c_uint) -> LLVMValueRef;
    
    // Core->Values->Constants->Constant expressions
    pub fn LLVMGetConstOpcode(ConstantVal: LLVMValueRef) -> LLVMOpcode;
    pub fn LLVMAlignOf(Ty: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMSizeOf(Ty: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstNeg(ConstantVal: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstNSWNeg(ConstantVal: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstNUWNeg(ConstantVal: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstFNeg(ConstantVal: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstNot(ConstantVal: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstAdd(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstNSWAdd(LHSConstant: LLVMValueRef,
                           RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstNUWAdd(LHSConstant: LLVMValueRef,
                           RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstFAdd(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstSub(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstNSWSub(LHSConstant: LLVMValueRef,
                           RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstNUWSub(LHSConstant: LLVMValueRef,
                           RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstFSub(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstMul(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstNSWMul(LHSConstant: LLVMValueRef,
                           RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstNUWMul(LHSConstant: LLVMValueRef,
                           RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstFMul(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstUDiv(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstSDiv(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstExactSDiv(LHSConstant: LLVMValueRef,
                              RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstFDiv(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstURem(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstSRem(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstFRem(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstAnd(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstOr(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstXor(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstICmp(Predicate: LLVMIntPredicate,
                         LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstFCmp(Predicate: LLVMRealPredicate,
                         LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstShl(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstLShr(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstAShr(LHSConstant: LLVMValueRef, RHSConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstGEP(ConstantVal: LLVMValueRef,
                        ConstantIndices: *mut LLVMValueRef,
                        NumIndices: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMConstInBoundsGEP(ConstantVal: LLVMValueRef,
                                ConstantIndices: *mut LLVMValueRef,
                                NumIndices: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMConstTrunc(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstSExt(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstZExt(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstFPTrunc(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstFPExt(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstUIToFP(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstSIToFP(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstFPToUI(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstFPToSI(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstPtrToInt(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstIntToPtr(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstBitCast(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstAddrSpaceCast(ConstantVal: LLVMValueRef,
                                  ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstZExtOrBitCast(ConstantVal: LLVMValueRef,
                                  ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstSExtOrBitCast(ConstantVal: LLVMValueRef,
                                  ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstTruncOrBitCast(ConstantVal: LLVMValueRef,
                                   ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstPointerCast(ConstantVal: LLVMValueRef,
                                ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstIntCast(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef,
                            isSigned: LLVMBool) -> LLVMValueRef;
    pub fn LLVMConstFPCast(ConstantVal: LLVMValueRef, ToType: LLVMTypeRef) -> LLVMValueRef;
    pub fn LLVMConstSelect(ConstantCondition: LLVMValueRef,
                           ConstantIfTrue: LLVMValueRef,
                           ConstantIfFalse: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstExtractElement(VectorConstant: LLVMValueRef,
                                   IndexConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstInsertElement(VectorConstant: LLVMValueRef,
                                  ElementValueConstant: LLVMValueRef,
                                  IndexConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstShuffleVector(VectorAConstant: LLVMValueRef,
                                  VectorBConstant: LLVMValueRef,
                                  MaskConstant: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMConstExtractValue(AggConstant: LLVMValueRef,
                                 IdxList: *mut ::libc::c_uint,
                                 NumIdx: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMConstInsertValue(AggConstant: LLVMValueRef,
                                ElementValueConstant: LLVMValueRef,
                                IdxList: *mut ::libc::c_uint,
                                NumIdx: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMConstInlineAsm(Ty: LLVMTypeRef,
                              AsmString: *const ::libc::c_char,
                              Constraints: *const ::libc::c_char,
                              HasSideEffects: LLVMBool,
                              IsAlignStack: LLVMBool) -> LLVMValueRef;
    pub fn LLVMBlockAddress(F: LLVMValueRef, BB: LLVMBasicBlockRef) -> LLVMValueRef;

    // Core->Values->Constants->Global Values
    pub fn LLVMGetGlobalParent(Global: LLVMValueRef) -> LLVMModuleRef;
    pub fn LLVMIsDeclaration(Global: LLVMValueRef) -> LLVMBool;
    pub fn LLVMGetLinkage(Global: LLVMValueRef) -> LLVMLinkage;
    pub fn LLVMSetLinkage(Global: LLVMValueRef, Linkage: LLVMLinkage) -> ();
    pub fn LLVMGetSection(Global: LLVMValueRef) -> *const ::libc::c_char;
    pub fn LLVMSetSection(Global: LLVMValueRef,
                          Section: *const ::libc::c_char) -> ();
    pub fn LLVMGetVisibility(Global: LLVMValueRef) -> LLVMVisibility;
    pub fn LLVMSetVisibility(Global: LLVMValueRef, Viz: LLVMVisibility) -> ();
    pub fn LLVMGetDLLStorageClass(Global: LLVMValueRef)
     -> LLVMDLLStorageClass;
    pub fn LLVMSetDLLStorageClass(Global: LLVMValueRef,
                                  Class: LLVMDLLStorageClass) -> ();
    pub fn LLVMHasUnnamedAddr(Global: LLVMValueRef) -> LLVMBool;
    pub fn LLVMSetUnnamedAddr(Global: LLVMValueRef, HasUnnamedAddr: LLVMBool) -> ();
    pub fn LLVMGetAlignment(V: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMSetAlignment(V: LLVMValueRef, Bytes: ::libc::c_uint) -> ();

    // Core->Values->Constants->Global Variables
    pub fn LLVMAddGlobal(M: LLVMModuleRef, Ty: LLVMTypeRef,
                         Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMAddGlobalInAddressSpace(M: LLVMModuleRef, Ty: LLVMTypeRef,
                                       Name: *const ::libc::c_char,
                                       AddressSpace: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMGetNamedGlobal(M: LLVMModuleRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMGetFirstGlobal(M: LLVMModuleRef) -> LLVMValueRef;
    pub fn LLVMGetLastGlobal(M: LLVMModuleRef) -> LLVMValueRef;
    pub fn LLVMGetNextGlobal(GlobalVar: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMGetPreviousGlobal(GlobalVar: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMDeleteGlobal(GlobalVar: LLVMValueRef) -> ();
    pub fn LLVMGetInitializer(GlobalVar: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMSetInitializer(GlobalVar: LLVMValueRef,
                              ConstantVal: LLVMValueRef) -> ();
    pub fn LLVMIsThreadLocal(GlobalVar: LLVMValueRef) -> LLVMBool;
    pub fn LLVMSetThreadLocal(GlobalVar: LLVMValueRef,
                              IsThreadLocal: LLVMBool) -> ();
    pub fn LLVMIsGlobalConstant(GlobalVar: LLVMValueRef) -> LLVMBool;
    pub fn LLVMSetGlobalConstant(GlobalVar: LLVMValueRef,
                                 IsConstant: LLVMBool) -> ();
    pub fn LLVMGetThreadLocalMode(GlobalVar: LLVMValueRef) -> LLVMThreadLocalMode;
    pub fn LLVMSetThreadLocalMode(GlobalVar: LLVMValueRef,
                                  Mode: LLVMThreadLocalMode) -> ();
    pub fn LLVMIsExternallyInitialized(GlobalVar: LLVMValueRef) -> LLVMBool;
    pub fn LLVMSetExternallyInitialized(GlobalVar: LLVMValueRef,
                                        IsExtInit: LLVMBool) -> ();

    // Core->Values->Constants->Global Aliases
    pub fn LLVMAddAlias(M: LLVMModuleRef, Ty: LLVMTypeRef,
                        Aliasee: LLVMValueRef, Name: *const ::libc::c_char) -> LLVMValueRef;

    // ..->Function Values
    pub fn LLVMDeleteFunction(Fn: LLVMValueRef) -> ();
    pub fn LLVMGetIntrinsicID(Fn: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMGetFunctionCallConv(Fn: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMSetFunctionCallConv(Fn: LLVMValueRef, CC: ::libc::c_uint) -> ();
    pub fn LLVMGetGC(Fn: LLVMValueRef) -> *const ::libc::c_char;
    pub fn LLVMSetGC(Fn: LLVMValueRef, Name: *const ::libc::c_char) -> ();
    pub fn LLVMAddFunctionAttr(Fn: LLVMValueRef, PA: LLVMAttribute) -> ();
    pub fn LLVMAddTargetDependentFunctionAttr(Fn: LLVMValueRef,
                                              A: *const ::libc::c_char,
                                              V: *const ::libc::c_char) -> ();
    pub fn LLVMGetFunctionAttr(Fn: LLVMValueRef) -> LLVMAttribute;
    pub fn LLVMRemoveFunctionAttr(Fn: LLVMValueRef, PA: LLVMAttribute) -> ();

    // ..->Function Values->Function Parameters
    pub fn LLVMCountParams(Fn: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMGetParams(Fn: LLVMValueRef, Params: *mut LLVMValueRef) -> ();
    pub fn LLVMGetParam(Fn: LLVMValueRef, Index: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMGetParamParent(Inst: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMGetFirstParam(Fn: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMGetLastParam(Fn: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMGetNextParam(Arg: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMGetPreviousParam(Arg: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMAddAttribute(Arg: LLVMValueRef, PA: LLVMAttribute) -> ();
    pub fn LLVMRemoveAttribute(Arg: LLVMValueRef, PA: LLVMAttribute) -> ();
    pub fn LLVMGetAttribute(Arg: LLVMValueRef) -> LLVMAttribute;
    pub fn LLVMSetParamAlignment(Arg: LLVMValueRef, align: ::libc::c_uint) -> ();
}

// Core->Metadata
extern "C" {
    pub fn LLVMMDStringInContext(C: LLVMContextRef,
                                 Str: *const ::libc::c_char,
                                 SLen: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMMDString(Str: *const ::libc::c_char, SLen: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMMDNodeInContext(C: LLVMContextRef, Vals: *mut LLVMValueRef,
                               Count: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMMDNode(Vals: *mut LLVMValueRef, Count: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMGetMDString(V: LLVMValueRef, Len: *mut ::libc::c_uint) -> *const ::libc::c_char;
    pub fn LLVMGetMDNodeNumOperands(V: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMGetMDNodeOperands(V: LLVMValueRef, Dest: *mut LLVMValueRef) -> ();
}

// Core->Basic Block
extern "C" {
    pub fn LLVMBasicBlockAsValue(BB: LLVMBasicBlockRef) -> LLVMValueRef;
    pub fn LLVMValueIsBasicBlock(Val: LLVMValueRef) -> LLVMBool;
    pub fn LLVMValueAsBasicBlock(Val: LLVMValueRef) -> LLVMBasicBlockRef;
    pub fn LLVMGetBasicBlockParent(BB: LLVMBasicBlockRef) -> LLVMValueRef;
    pub fn LLVMGetBasicBlockTerminator(BB: LLVMBasicBlockRef) -> LLVMValueRef;
    pub fn LLVMCountBasicBlocks(Fn: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMGetBasicBlocks(Fn: LLVMValueRef,
                              BasicBlocks: *mut LLVMBasicBlockRef) -> ();
    pub fn LLVMGetFirstBasicBlock(Fn: LLVMValueRef) -> LLVMBasicBlockRef;
    pub fn LLVMGetLastBasicBlock(Fn: LLVMValueRef) -> LLVMBasicBlockRef;
    pub fn LLVMGetNextBasicBlock(BB: LLVMBasicBlockRef) -> LLVMBasicBlockRef;
    pub fn LLVMGetPreviousBasicBlock(BB: LLVMBasicBlockRef) -> LLVMBasicBlockRef;
    pub fn LLVMGetEntryBasicBlock(Fn: LLVMValueRef) -> LLVMBasicBlockRef;
    pub fn LLVMAppendBasicBlockInContext(C: LLVMContextRef, Fn: LLVMValueRef,
                                         Name: *const ::libc::c_char) -> LLVMBasicBlockRef;
    pub fn LLVMAppendBasicBlock(Fn: LLVMValueRef, Name: *const ::libc::c_char) -> LLVMBasicBlockRef;
    pub fn LLVMInsertBasicBlockInContext(C: LLVMContextRef,
                                         BB: LLVMBasicBlockRef,
                                         Name: *const ::libc::c_char) -> LLVMBasicBlockRef;
    pub fn LLVMInsertBasicBlock(InsertBeforeBB: LLVMBasicBlockRef,
                                Name: *const ::libc::c_char) -> LLVMBasicBlockRef;
    pub fn LLVMDeleteBasicBlock(BB: LLVMBasicBlockRef) -> ();
    pub fn LLVMRemoveBasicBlockFromParent(BB: LLVMBasicBlockRef) -> ();
    pub fn LLVMMoveBasicBlockBefore(BB: LLVMBasicBlockRef,
                                    MovePos: LLVMBasicBlockRef) -> ();
    pub fn LLVMMoveBasicBlockAfter(BB: LLVMBasicBlockRef,
                                   MovePos: LLVMBasicBlockRef) -> ();
    pub fn LLVMGetFirstInstruction(BB: LLVMBasicBlockRef) -> LLVMValueRef;
    pub fn LLVMGetLastInstruction(BB: LLVMBasicBlockRef) -> LLVMValueRef;
}

// Core->Instructions
extern "C" {
    pub fn LLVMHasMetadata(Val: LLVMValueRef) -> ::libc::c_int;
    pub fn LLVMGetMetadata(Val: LLVMValueRef, KindID: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMSetMetadata(Val: LLVMValueRef, KindID: ::libc::c_uint,
                           Node: LLVMValueRef) -> ();
    pub fn LLVMGetInstructionParent(Inst: LLVMValueRef) -> LLVMBasicBlockRef;
    pub fn LLVMGetNextInstruction(Inst: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMGetPreviousInstruction(Inst: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMInstructionEraseFromParent(Inst: LLVMValueRef) -> ();
    pub fn LLVMGetInstructionOpcode(Inst: LLVMValueRef) -> LLVMOpcode;
    pub fn LLVMGetICmpPredicate(Inst: LLVMValueRef) -> LLVMIntPredicate;
    pub fn LLVMGetFCmpPredicate(Inst: LLVMValueRef) -> LLVMRealPredicate;
    pub fn LLVMInstructionClone(Inst: LLVMValueRef) -> LLVMValueRef;

    // Instructions->Call Sites and Invocations
    pub fn LLVMSetInstructionCallConv(Instr: LLVMValueRef, CC: ::libc::c_uint) -> ();
    pub fn LLVMGetInstructionCallConv(Instr: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMAddInstrAttribute(Instr: LLVMValueRef, index: ::libc::c_uint,
                                 arg1: LLVMAttribute) -> ();
    pub fn LLVMRemoveInstrAttribute(Instr: LLVMValueRef,
                                    index: ::libc::c_uint,
                                    arg1: LLVMAttribute) -> ();
    pub fn LLVMSetInstrParamAlignment(Instr: LLVMValueRef,
                                      index: ::libc::c_uint,
                                      align: ::libc::c_uint) -> ();
    pub fn LLVMIsTailCall(CallInst: LLVMValueRef) -> LLVMBool;
    pub fn LLVMSetTailCall(CallInst: LLVMValueRef, IsTailCall: LLVMBool) -> ();

    // Instructions->Terminators
    pub fn LLVMGetNumSuccessors(Term: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMGetSuccessor(Term: LLVMValueRef, i: ::libc::c_uint) -> LLVMBasicBlockRef;
    pub fn LLVMSetSuccessor(Term: LLVMValueRef, i: ::libc::c_uint,
                            block: LLVMBasicBlockRef) -> ();
    pub fn LLVMIsConditional(Branch: LLVMValueRef) -> LLVMBool;
    pub fn LLVMGetCondition(Branch: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMSetCondition(Branch: LLVMValueRef, Cond: LLVMValueRef) -> ();
    pub fn LLVMGetSwitchDefaultDest(SwitchInstr: LLVMValueRef) -> LLVMBasicBlockRef;

    // Instruction->PHI Nodes
    pub fn LLVMAddIncoming(PhiNode: LLVMValueRef,
                           IncomingValues: *mut LLVMValueRef,
                           IncomingBlocks: *mut LLVMBasicBlockRef,
                           Count: ::libc::c_uint) -> ();
    pub fn LLVMCountIncoming(PhiNode: LLVMValueRef) -> ::libc::c_uint;
    pub fn LLVMGetIncomingValue(PhiNode: LLVMValueRef, Index: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMGetIncomingBlock(PhiNode: LLVMValueRef, Index: ::libc::c_uint) -> LLVMBasicBlockRef;
    
}

// Core->Values again; these don't appear in Doxygen because they're macro-generated.
extern "C" {
    pub fn LLVMIsAArgument(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsABasicBlock(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAInlineAsm(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAUser(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstant(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsABlockAddress(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantAggregateZero(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantArray(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantDataSequential(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantDataArray(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantDataVector(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantExpr(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantFP(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantInt(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantPointerNull(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantStruct(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAConstantVector(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAGlobalValue(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAGlobalAlias(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAGlobalObject(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFunction(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAGlobalVariable(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAUndefValue(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAInstruction(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsABinaryOperator(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsACallInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAIntrinsicInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsADbgInfoIntrinsic(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsADbgDeclareInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAMemIntrinsic(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAMemCpyInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAMemMoveInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAMemSetInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsACmpInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFCmpInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAICmpInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAExtractElementInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAGetElementPtrInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAInsertElementInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAInsertValueInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsALandingPadInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAPHINode(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsASelectInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAShuffleVectorInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAStoreInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsATerminatorInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsABranchInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAIndirectBrInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAInvokeInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAReturnInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsASwitchInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAUnreachableInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAResumeInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAUnaryInstruction(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAAllocaInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsACastInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAAddrSpaceCastInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsABitCastInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFPExtInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFPToSIInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFPToUIInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAFPTruncInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAIntToPtrInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAPtrToIntInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsASExtInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsASIToFPInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsATruncInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAUIToFPInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAZExtInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAExtractValueInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsALoadInst(Val: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMIsAVAArgInst(Val: LLVMValueRef) -> LLVMValueRef;
}

// Core->Instruction Builders
extern "C" {
    pub fn LLVMCreateBuilderInContext(C: LLVMContextRef) -> LLVMBuilderRef;
    pub fn LLVMCreateBuilder() -> LLVMBuilderRef;
    pub fn LLVMPositionBuilder(Builder: LLVMBuilderRef,
                               Block: LLVMBasicBlockRef, Instr: LLVMValueRef) -> ();
    pub fn LLVMPositionBuilderBefore(Builder: LLVMBuilderRef,
                                     Instr: LLVMValueRef) -> ();
    pub fn LLVMPositionBuilderAtEnd(Builder: LLVMBuilderRef,
                                    Block: LLVMBasicBlockRef) -> ();
    pub fn LLVMGetInsertBlock(Builder: LLVMBuilderRef) -> LLVMBasicBlockRef;
    pub fn LLVMClearInsertionPosition(Builder: LLVMBuilderRef) -> ();
    pub fn LLVMInsertIntoBuilder(Builder: LLVMBuilderRef, Instr: LLVMValueRef) -> ();
    pub fn LLVMInsertIntoBuilderWithName(Builder: LLVMBuilderRef,
                                         Instr: LLVMValueRef,
                                         Name: *const ::libc::c_char) -> ();
    pub fn LLVMDisposeBuilder(Builder: LLVMBuilderRef) -> ();

    // Metadata
    pub fn LLVMSetCurrentDebugLocation(Builder: LLVMBuilderRef,
                                       L: LLVMValueRef) -> ();
    pub fn LLVMGetCurrentDebugLocation(Builder: LLVMBuilderRef) -> LLVMValueRef;
    pub fn LLVMSetInstDebugLocation(Builder: LLVMBuilderRef,
                                    Inst: LLVMValueRef) -> ();

    // Terminators
    pub fn LLVMBuildRetVoid(arg1: LLVMBuilderRef) -> LLVMValueRef;
    pub fn LLVMBuildRet(arg1: LLVMBuilderRef, V: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMBuildAggregateRet(arg1: LLVMBuilderRef,
                                 RetVals: *mut LLVMValueRef,
                                 N: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMBuildBr(arg1: LLVMBuilderRef, Dest: LLVMBasicBlockRef)
     -> LLVMValueRef;
    pub fn LLVMBuildCondBr(arg1: LLVMBuilderRef, If: LLVMValueRef,
                           Then: LLVMBasicBlockRef, Else: LLVMBasicBlockRef)
     -> LLVMValueRef;
    pub fn LLVMBuildSwitch(arg1: LLVMBuilderRef, V: LLVMValueRef,
                           Else: LLVMBasicBlockRef, NumCases: ::libc::c_uint)
     -> LLVMValueRef;
    pub fn LLVMBuildIndirectBr(B: LLVMBuilderRef, Addr: LLVMValueRef,
                               NumDests: ::libc::c_uint) -> LLVMValueRef;
    pub fn LLVMBuildInvoke(arg1: LLVMBuilderRef, Fn: LLVMValueRef,
                           Args: *mut LLVMValueRef, NumArgs: ::libc::c_uint,
                           Then: LLVMBasicBlockRef, Catch: LLVMBasicBlockRef,
                           Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildLandingPad(B: LLVMBuilderRef, Ty: LLVMTypeRef,
                               PersFn: LLVMValueRef,
                               NumClauses: ::libc::c_uint,
                               Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildResume(B: LLVMBuilderRef, Exn: LLVMValueRef)
     -> LLVMValueRef;
    pub fn LLVMBuildUnreachable(arg1: LLVMBuilderRef) -> LLVMValueRef;

    /// Add a case to a `switch` instruction
    pub fn LLVMAddCase(Switch: LLVMValueRef, OnVal: LLVMValueRef,
                       Dest: LLVMBasicBlockRef) -> ();

    /// Add a destination to an `indirectbr` instruction
    pub fn LLVMAddDestination(IndirectBr: LLVMValueRef,
                              Dest: LLVMBasicBlockRef) -> ();

    /// Add a catch or filter clause to a `landingpad` instruction
    pub fn LLVMAddClause(LandingPad: LLVMValueRef, ClauseVal: LLVMValueRef) -> ();

    /// Set the cleanup flag in a `landingpad` instruction.
    pub fn LLVMSetCleanup(LandingPad: LLVMValueRef, Val: LLVMBool) -> ();

    // Arithmetic
    pub fn LLVMBuildAdd(arg1: LLVMBuilderRef, LHS: LLVMValueRef,
                        RHS: LLVMValueRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildNSWAdd(arg1: LLVMBuilderRef, LHS: LLVMValueRef,
                           RHS: LLVMValueRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildNUWAdd(arg1: LLVMBuilderRef, LHS: LLVMValueRef,
                           RHS: LLVMValueRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildFAdd(arg1: LLVMBuilderRef, LHS: LLVMValueRef,
                         RHS: LLVMValueRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildSub(arg1: LLVMBuilderRef, LHS: LLVMValueRef,
                        RHS: LLVMValueRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildNSWSub(arg1: LLVMBuilderRef, LHS: LLVMValueRef,
                           RHS: LLVMValueRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildNUWSub(arg1: LLVMBuilderRef, LHS: LLVMValueRef,
                           RHS: LLVMValueRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildFSub(arg1: LLVMBuilderRef, LHS: LLVMValueRef,
                         RHS: LLVMValueRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildMul(arg1: LLVMBuilderRef, LHS: LLVMValueRef,
                        RHS: LLVMValueRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildNSWMul(arg1: LLVMBuilderRef, LHS: LLVMValueRef,
                           RHS: LLVMValueRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildNUWMul(arg1: LLVMBuilderRef, LHS: LLVMValueRef,
                           RHS: LLVMValueRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildFMul(arg1: LLVMBuilderRef, LHS: LLVMValueRef,
                         RHS: LLVMValueRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildUDiv(arg1: LLVMBuilderRef, LHS: LLVMValueRef,
                         RHS: LLVMValueRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildSDiv(arg1: LLVMBuilderRef, LHS: LLVMValueRef,
                         RHS: LLVMValueRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildExactSDiv(arg1: LLVMBuilderRef, LHS: LLVMValueRef,
                              RHS: LLVMValueRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildFDiv(arg1: LLVMBuilderRef, LHS: LLVMValueRef,
                         RHS: LLVMValueRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildURem(arg1: LLVMBuilderRef, LHS: LLVMValueRef,
                         RHS: LLVMValueRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildSRem(arg1: LLVMBuilderRef, LHS: LLVMValueRef,
                         RHS: LLVMValueRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildFRem(arg1: LLVMBuilderRef, LHS: LLVMValueRef,
                         RHS: LLVMValueRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildShl(arg1: LLVMBuilderRef, LHS: LLVMValueRef,
                        RHS: LLVMValueRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildLShr(arg1: LLVMBuilderRef, LHS: LLVMValueRef,
                         RHS: LLVMValueRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildAShr(arg1: LLVMBuilderRef, LHS: LLVMValueRef,
                         RHS: LLVMValueRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildAnd(arg1: LLVMBuilderRef, LHS: LLVMValueRef,
                        RHS: LLVMValueRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildOr(arg1: LLVMBuilderRef, LHS: LLVMValueRef,
                       RHS: LLVMValueRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildXor(arg1: LLVMBuilderRef, LHS: LLVMValueRef,
                        RHS: LLVMValueRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildBinOp(B: LLVMBuilderRef, Op: LLVMOpcode,
                          LHS: LLVMValueRef, RHS: LLVMValueRef,
                          Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildNeg(arg1: LLVMBuilderRef, V: LLVMValueRef,
                        Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildNSWNeg(B: LLVMBuilderRef, V: LLVMValueRef,
                           Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildNUWNeg(B: LLVMBuilderRef, V: LLVMValueRef,
                           Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildFNeg(arg1: LLVMBuilderRef, V: LLVMValueRef,
                         Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildNot(arg1: LLVMBuilderRef, V: LLVMValueRef,
                        Name: *const ::libc::c_char) -> LLVMValueRef;

    // Memory
    pub fn LLVMBuildMalloc(arg1: LLVMBuilderRef, Ty: LLVMTypeRef,
                           Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildArrayMalloc(arg1: LLVMBuilderRef, Ty: LLVMTypeRef,
                                Val: LLVMValueRef,
                                Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildAlloca(arg1: LLVMBuilderRef, Ty: LLVMTypeRef,
                           Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildArrayAlloca(arg1: LLVMBuilderRef, Ty: LLVMTypeRef,
                                Val: LLVMValueRef,
                                Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildFree(arg1: LLVMBuilderRef, PointerVal: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMBuildLoad(arg1: LLVMBuilderRef, PointerVal: LLVMValueRef,
                         Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildStore(arg1: LLVMBuilderRef, Val: LLVMValueRef,
                          Ptr: LLVMValueRef) -> LLVMValueRef;
    pub fn LLVMBuildGEP(B: LLVMBuilderRef, Pointer: LLVMValueRef,
                        Indices: *mut LLVMValueRef,
                        NumIndices: ::libc::c_uint,
                        Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildInBoundsGEP(B: LLVMBuilderRef, Pointer: LLVMValueRef,
                                Indices: *mut LLVMValueRef,
                                NumIndices: ::libc::c_uint,
                                Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildStructGEP(B: LLVMBuilderRef, Pointer: LLVMValueRef,
                              Idx: ::libc::c_uint,
                              Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildGlobalString(B: LLVMBuilderRef,
                                 Str: *const ::libc::c_char,
                                 Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildGlobalStringPtr(B: LLVMBuilderRef,
                                    Str: *const ::libc::c_char,
                                    Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMGetVolatile(MemoryAccessInst: LLVMValueRef) -> LLVMBool;
    pub fn LLVMSetVolatile(MemoryAccessInst: LLVMValueRef,
                           IsVolatile: LLVMBool) -> ();

    // Casts
    pub fn LLVMBuildTrunc(arg1: LLVMBuilderRef, Val: LLVMValueRef,
                          DestTy: LLVMTypeRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildZExt(arg1: LLVMBuilderRef, Val: LLVMValueRef,
                         DestTy: LLVMTypeRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildSExt(arg1: LLVMBuilderRef, Val: LLVMValueRef,
                         DestTy: LLVMTypeRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildFPToUI(arg1: LLVMBuilderRef, Val: LLVMValueRef,
                           DestTy: LLVMTypeRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildFPToSI(arg1: LLVMBuilderRef, Val: LLVMValueRef,
                           DestTy: LLVMTypeRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildUIToFP(arg1: LLVMBuilderRef, Val: LLVMValueRef,
                           DestTy: LLVMTypeRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildSIToFP(arg1: LLVMBuilderRef, Val: LLVMValueRef,
                           DestTy: LLVMTypeRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildFPTrunc(arg1: LLVMBuilderRef, Val: LLVMValueRef,
                            DestTy: LLVMTypeRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildFPExt(arg1: LLVMBuilderRef, Val: LLVMValueRef,
                          DestTy: LLVMTypeRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildPtrToInt(arg1: LLVMBuilderRef, Val: LLVMValueRef,
                             DestTy: LLVMTypeRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildIntToPtr(arg1: LLVMBuilderRef, Val: LLVMValueRef,
                             DestTy: LLVMTypeRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildBitCast(arg1: LLVMBuilderRef, Val: LLVMValueRef,
                            DestTy: LLVMTypeRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildAddrSpaceCast(arg1: LLVMBuilderRef, Val: LLVMValueRef,
                                  DestTy: LLVMTypeRef,
                                  Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildZExtOrBitCast(arg1: LLVMBuilderRef, Val: LLVMValueRef,
                                  DestTy: LLVMTypeRef,
                                  Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildSExtOrBitCast(arg1: LLVMBuilderRef, Val: LLVMValueRef,
                                  DestTy: LLVMTypeRef,
                                  Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildTruncOrBitCast(arg1: LLVMBuilderRef, Val: LLVMValueRef,
                                   DestTy: LLVMTypeRef,
                                   Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildCast(B: LLVMBuilderRef, Op: LLVMOpcode, Val: LLVMValueRef,
                         DestTy: LLVMTypeRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildPointerCast(arg1: LLVMBuilderRef, Val: LLVMValueRef,
                                DestTy: LLVMTypeRef,
                                Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildIntCast(arg1: LLVMBuilderRef, Val: LLVMValueRef,
                            DestTy: LLVMTypeRef, Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildFPCast(arg1: LLVMBuilderRef, Val: LLVMValueRef,
                           DestTy: LLVMTypeRef, Name: *const ::libc::c_char) -> LLVMValueRef;

    // Comparisons
    pub fn LLVMBuildICmp(arg1: LLVMBuilderRef, Op: LLVMIntPredicate,
                         LHS: LLVMValueRef, RHS: LLVMValueRef,
                         Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildFCmp(arg1: LLVMBuilderRef, Op: LLVMRealPredicate,
                         LHS: LLVMValueRef, RHS: LLVMValueRef,
                         Name: *const ::libc::c_char) -> LLVMValueRef;

    // Miscellaneous instructions
    pub fn LLVMBuildPhi(arg1: LLVMBuilderRef, Ty: LLVMTypeRef,
                        Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildCall(arg1: LLVMBuilderRef, Fn: LLVMValueRef,
                         Args: *mut LLVMValueRef, NumArgs: ::libc::c_uint,
                         Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildSelect(arg1: LLVMBuilderRef, If: LLVMValueRef,
                           Then: LLVMValueRef, Else: LLVMValueRef,
                           Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildVAArg(arg1: LLVMBuilderRef, List: LLVMValueRef,
                          Ty: LLVMTypeRef, Name: *const ::libc::c_char)
     -> LLVMValueRef;
    pub fn LLVMBuildExtractElement(arg1: LLVMBuilderRef, VecVal: LLVMValueRef,
                                   Index: LLVMValueRef,
                                   Name: *const ::libc::c_char)
     -> LLVMValueRef;
    pub fn LLVMBuildInsertElement(arg1: LLVMBuilderRef, VecVal: LLVMValueRef,
                                  EltVal: LLVMValueRef, Index: LLVMValueRef,
                                  Name: *const ::libc::c_char)
     -> LLVMValueRef;
    pub fn LLVMBuildShuffleVector(arg1: LLVMBuilderRef, V1: LLVMValueRef,
                                  V2: LLVMValueRef, Mask: LLVMValueRef,
                                  Name: *const ::libc::c_char)
     -> LLVMValueRef;
    pub fn LLVMBuildExtractValue(arg1: LLVMBuilderRef, AggVal: LLVMValueRef,
                                 Index: ::libc::c_uint,
                                 Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildInsertValue(arg1: LLVMBuilderRef, AggVal: LLVMValueRef,
                                EltVal: LLVMValueRef, Index: ::libc::c_uint,
                                Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildIsNull(arg1: LLVMBuilderRef, Val: LLVMValueRef,
                           Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildIsNotNull(arg1: LLVMBuilderRef, Val: LLVMValueRef,
                              Name: *const ::libc::c_char) -> LLVMValueRef;
    pub fn LLVMBuildPtrDiff(arg1: LLVMBuilderRef, LHS: LLVMValueRef,
                            RHS: LLVMValueRef, Name: *const ::libc::c_char)
     -> LLVMValueRef;
    pub fn LLVMBuildFence(B: LLVMBuilderRef, ordering: LLVMAtomicOrdering,
                          singleThread: LLVMBool, Name: *const ::libc::c_char)
     -> LLVMValueRef;
    pub fn LLVMBuildAtomicRMW(B: LLVMBuilderRef, op: LLVMAtomicRMWBinOp,
                              PTR: LLVMValueRef, Val: LLVMValueRef,
                              ordering: LLVMAtomicOrdering,
                              singleThread: LLVMBool) -> LLVMValueRef;
}

// Core->Module Providers
extern "C" {
    pub fn LLVMCreateModuleProviderForExistingModule(M: LLVMModuleRef)
     -> LLVMModuleProviderRef;
    pub fn LLVMDisposeModuleProvider(M: LLVMModuleProviderRef) -> ();
}

// Core->Memory Buffers
extern "C" {
    pub fn LLVMCreateMemoryBufferWithContentsOfFile(Path:
                                                        *const ::libc::c_char,
                                                    OutMemBuf:
                                                        *mut LLVMMemoryBufferRef,
                                                    OutMessage:
                                                        *mut *mut ::libc::c_char)
     -> LLVMBool;
    pub fn LLVMCreateMemoryBufferWithSTDIN(OutMemBuf:
                                               *mut LLVMMemoryBufferRef,
                                           OutMessage:
                                               *mut *mut ::libc::c_char)
     -> LLVMBool;
    pub fn LLVMCreateMemoryBufferWithMemoryRange(InputData:
                                                     *const ::libc::c_char,
                                                 InputDataLength: ::libc::size_t,
                                                 BufferName:
                                                     *const ::libc::c_char,
                                                 RequiresNullTerminator:
                                                     LLVMBool)
     -> LLVMMemoryBufferRef;
    pub fn LLVMCreateMemoryBufferWithMemoryRangeCopy(InputData:
                                                         *const ::libc::c_char,
                                                     InputDataLength: ::libc::size_t,
                                                     BufferName:
                                                         *const ::libc::c_char)
     -> LLVMMemoryBufferRef;
    pub fn LLVMGetBufferStart(MemBuf: LLVMMemoryBufferRef)
     -> *const ::libc::c_char;
    pub fn LLVMGetBufferSize(MemBuf: LLVMMemoryBufferRef) -> ::libc::size_t;
    pub fn LLVMDisposeMemoryBuffer(MemBuf: LLVMMemoryBufferRef) -> ();
}

// Core->pass registry
extern "C" {
    pub fn LLVMGetGlobalPassRegistry() -> LLVMPassRegistryRef;
}

// Core->Pass managers
extern "C" {
    pub fn LLVMCreatePassManager() -> LLVMPassManagerRef;
    pub fn LLVMCreateFunctionPassManagerForModule(M: LLVMModuleRef)
     -> LLVMPassManagerRef;
    pub fn LLVMCreateFunctionPassManager(MP: LLVMModuleProviderRef)
     -> LLVMPassManagerRef;
    pub fn LLVMRunPassManager(PM: LLVMPassManagerRef, M: LLVMModuleRef)
     -> LLVMBool;
    pub fn LLVMInitializeFunctionPassManager(FPM: LLVMPassManagerRef)
     -> LLVMBool;
    pub fn LLVMRunFunctionPassManager(FPM: LLVMPassManagerRef,
                                      F: LLVMValueRef) -> LLVMBool;
    pub fn LLVMFinalizeFunctionPassManager(FPM: LLVMPassManagerRef)
     -> LLVMBool;
    pub fn LLVMDisposePassManager(PM: LLVMPassManagerRef) -> ();
}

// Core->Threading
extern "C" {
    /// Deprecated: LLVM threading is configured at compile-time with `LLVM_ENABLE_THREADS`
    pub fn LLVMStartMultithreaded() -> LLVMBool;
    /// Deprecated: LLVM threading is configured at compile-time with `LLVM_ENABLE_THREADS`
    pub fn LLVMStopMultithreaded() -> ();
    pub fn LLVMIsMultithreaded() -> LLVMBool;
}
