use std::convert::identity;

use crate::{native_methods_registry::NativeMethodsRegistry, value::Value};

pub(crate) fn register_natives(registry: &mut NativeMethodsRegistry) {
    registry.register_temp_print(|vm, _, _, args| temp_print(vm, args));
    register_noops(registry);
    register_time_methods(registry);
    register_gc_methods(registry);
    register_native_repr_methods(registry);
    register_reflection_methods(registry);
    register_throwable_methods(registry);
}

fn register_noops(registry: &mut NativeMethodsRegistry) {
    registry.register(
        "java/lang/Object",
        "registerNatives",
        "()V",
        |_, _, _, _| Ok(None),
    );
    registry.register(
        "java/lang/System",
        "registerNatives",
        "()V",
        |_, _, _, _| Ok(None),
    );
    registry.register("java/lang/Class", "registerNatives", "()V", |_, _, _, _| {
        Ok(None)
    });
    registry.register(
        "java/lang/ClassLoader",
        "registerNatives",
        "()V",
        |_, _, _, _| Ok(None),
    );
}

fn register_time_methods(registry: &mut NativeMethodsRegistry) {
    registry.register("java/lang/System", "nanoTime", "()J", |_, _, _, _| {
        Ok(Some(Value::Long(get_nano_time())))
    });
    registry.register(
        "java/lang/System",
        "currentTimeMillis",
        "()J",
        |_, _, _, _| Ok(Some(Value::Long(get_current_time_millis()))),
    );
}

fn register_gc_methods(registry: &mut NativeMethodsRegistry) {
    registry.register(
        "java/lang/System",
        "identityHashCode",
        "(Ljava/lang/Object;)I",
        |_, _, _, args| identity_hash_code(args),
    );
    registry.register("java/lang/System", "gc", "()V", |vm, _, _, _| {
        vm.run_garbage_collection()?;
        Ok(None)
    });
}

fn register_native_repr_methods(registry: &mut NativeMethodsRegistry) {
    registry.register(
        "java/lang/System",
        "arraycopy",
        "(Ljava/lang/Object;ILjava/lang/Object;II)V",
        |_, _, _, args| native_array_copy(args),
    );
    registry.register(
        "java/lang/Float",
        "floatToRawIntBits",
        "(F)I",
        |_, _, _, args| float_to_raw_int_bits(&args),
    );
    registry.register(
        "java/lang/Double",
        "doubleToRawLongBits",
        "(D)J",
        |_, _, _, args| double_to_raw_long_bits(&args),
    );
}
