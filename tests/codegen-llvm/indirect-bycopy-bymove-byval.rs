//! Regression test for issue <https://github.com/rust-lang/rust/issues/155241>.
//! Arguments passed indirectly via a hidden pointer must be copied to an alloca,
//! except for by-val or by-move.
//@ compile-flags: -Cno-prepopulate-passes -Copt-level=3
//@ only-x86_64-unknown-linux-gnu

#![crate_type = "lib"]
#![feature(fn_traits, stmt_expr_attributes)]
#![expect(unused)]

#[derive(Copy, Clone)]
struct Thing(usize, usize, usize);

// The argument of the second call is a by-move argument.

// CHECK-LABEL: @normal
// CHECK: call void @llvm.memcpy.p0.p0.i64(ptr align 8 [[normal_V1:%.*]], ptr align 8 %value, i64 24, i1 false)
// CHECK: call void @opaque(ptr{{.*}} [[normal_V1]])
// CHECK: call void @opaque(ptr{{.*}} %value)
// CHECK: call void @llvm.memcpy.p0.p0.i64(ptr align 8 [[normal_V3:%.*]], ptr align 8 @anon{{.*}}, i64 24, i1 false)
// CHECK: call void @opaque(ptr{{.*}} [[normal_V3]])
#[unsafe(no_mangle)]
pub fn normal() {
    #[inline(never)]
    #[unsafe(no_mangle)]
    fn opaque(mut thing: Thing) {
        thing.0 = 1;
    }
    let value = Thing(0, 0, 0);
    opaque(value);
    opaque(value);
    const VALUE: Thing = Thing(0, 0, 0);
    opaque(VALUE);
}

// The argument of the second call is a by-move argument.

// CHECK-LABEL: @untupled
// CHECK: call void @llvm.memcpy.p0.p0.i64(ptr align 8 [[untupled_V1:%.*]], ptr align 8 %value, i64 24, i1 false)
// CHECK: call indirect_bycopy_bymove_byval::untupled::{closure#0}
// CHECK-NEXT: call void @{{.*}}(ptr {{.*}}, ptr{{.*}} [[untupled_V1]])
// CHECK: call indirect_bycopy_bymove_byval::untupled::{closure#1}
// CHECK-NEXT: call void @{{.*}}(ptr {{.*}}, ptr{{.*}} %value)
// CHECK: call void @llvm.memcpy.p0.p0.i64(ptr align 8 [[untupled_V3:%.*]], ptr align 8 @anon{{.*}}, i64 24, i1 false)
// CHECK: call indirect_bycopy_bymove_byval::untupled::{closure#2}
// CHECK-NEXT: call void @{{.*}}(ptr {{.*}}, ptr{{.*}} [[untupled_V3]])
#[unsafe(no_mangle)]
pub fn untupled() {
    let value = (Thing(0, 0, 0),);
    (#[inline(never)]
    |mut thing: Thing| {
        thing.0 = 1;
    })
    .call(value);
    (#[inline(never)]
    |mut thing: Thing| {
        thing.0 = 2;
    })
    .call(value);
    const VALUE: (Thing,) = (Thing(0, 0, 0),);
    (#[inline(never)]
    |mut thing: Thing| {
        thing.0 = 3;
    })
    .call(VALUE);
}

// All memcpy calls are redundant for byval.

// CHECK-LABEL: @byval
// CHECK: call void @opaque_byval(ptr{{.*}} byval([24 x i8]){{.*}} %value)
// CHECK: call void @opaque_byval(ptr{{.*}} byval([24 x i8]){{.*}} %value)
// CHECK: call void @opaque_byval(ptr{{.*}} byval([24 x i8]){{.*}} @anon{{.*}})
#[unsafe(no_mangle)]
pub fn byval() {
    #[derive(Copy, Clone)]
    #[repr(C)]
    struct Thing(usize, usize, usize);
    #[inline(never)]
    #[unsafe(no_mangle)]
    extern "C" fn opaque_byval(mut thing: Thing) {
        thing.0 = 1;
    }
    let value = Thing(0, 0, 0);
    opaque_byval(value);
    opaque_byval(value);
    const VALUE: Thing = Thing(0, 0, 0);
    opaque_byval(VALUE);
}
