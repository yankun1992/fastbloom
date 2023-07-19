use std::ptr::slice_from_raw_parts;

use fastbloom_rs::{BloomFilter, CountingBloomFilter, Deletable, FilterBuilder, Hashes, Membership};
use jni::JNIEnv;
use jni::objects::*;
use jni::sys::*;

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_FilterBuilder_new0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, expected_elements: jlong, false_positive_probability: jdouble,
) -> jlong {
    let mut builder = FilterBuilder::new(expected_elements as u64, false_positive_probability as f64);

    let builder = Box::new(builder);

    Box::into_raw(builder) as jlong
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_FilterBuilder_fromSizeAndHashes0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, size: jlong, hashes: jint,
) -> jlong {
    let mut builder = FilterBuilder::from_size_and_hashes(size as u64, hashes as u32);

    let builder = Box::new(builder);

    Box::into_raw(builder) as jlong
}


#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_FilterBuilder_enableRepeatInsert0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong, enable: jboolean,
) {
    let mut builder = Box::from_raw(raw as *mut FilterBuilder);

    builder.enable_repeat_insert(enable != 0);

    Box::into_raw(builder); // keep builder alive.
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_FilterBuilder_buildBloomFilter0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong,
) -> jlong {
    let mut builder = Box::from_raw(raw as *mut FilterBuilder);

    let filter = Box::new(builder.build_bloom_filter());

    Box::into_raw(builder); // keep builder alive.

    Box::into_raw(filter) as jlong
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_FilterBuilder_buildCountingBloomFilter0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong,
) -> jlong {
    let mut builder = Box::from_raw(raw as *mut FilterBuilder);

    let filter = Box::new(builder.build_counting_bloom_filter());

    Box::into_raw(builder); // keep builder alive.

    Box::into_raw(filter) as jlong
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_FilterBuilder_close0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong,
) {
    let mut builder = Box::from_raw(raw as *mut FilterBuilder);

    drop(builder);
}


#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_BloomFilter_hashes0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong,
) -> jint {
    let mut filter = Box::from_raw(raw as *mut BloomFilter);

    let hashes = filter.hashes();

    Box::into_raw(filter); // keep builder alive.

    hashes as jint
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_BloomFilter_addInt0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong, element: jint,
) {
    let mut filter = Box::from_raw(raw as *mut BloomFilter);

    let element = element as i32;

    filter.add(&i32::to_le_bytes(element));

    Box::into_raw(filter); // keep builder alive.
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_BloomFilter_addLong0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong, element: jlong,
) {
    let mut filter = Box::from_raw(raw as *mut BloomFilter);

    let element = element as i64;

    filter.add(&i64::to_le_bytes(element));

    Box::into_raw(filter); // keep builder alive.
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_BloomFilter_addIntBatch0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong, array: JIntArray<'local>,
) {
    let mut filter = Box::from_raw(raw as *mut BloomFilter);

    let len = env.get_array_length(&array).unwrap() as usize;
    let mut buf = vec![0; len];

    env.get_int_array_region(array, 0, &mut buf).unwrap();


    for element in buf {
        let element = element as i32;

        filter.add(&i32::to_le_bytes(element));
    }

    Box::into_raw(filter); // keep builder alive.
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_BloomFilter_addStr0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong, element: JString<'local>,
) {
    let mut filter = Box::from_raw(raw as *mut BloomFilter);

    let element = env.get_string(&element).unwrap();

    filter.add(element.to_bytes());

    Box::into_raw(filter); // keep builder alive.
}


#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_BloomFilter_addBytes0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong, element: JByteArray<'local>,
) {
    let mut filter = Box::from_raw(raw as *mut BloomFilter);

    let element = env.convert_byte_array(element).unwrap();

    filter.add(&element);

    Box::into_raw(filter); // keep builder alive.
}


#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_BloomFilter_containsInt0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong, element: jint,
) -> jboolean {
    let mut filter = Box::from_raw(raw as *mut BloomFilter);

    let element = element as i32;

    let res = filter.contains(&i32::to_le_bytes(element));

    Box::into_raw(filter); // keep builder alive.

    res as jboolean
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_BloomFilter_containsLong0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong, element: jlong,
) -> jboolean {
    let mut filter = Box::from_raw(raw as *mut BloomFilter);

    let element = element as i64;

    let res = filter.contains(&i64::to_le_bytes(element));

    Box::into_raw(filter); // keep builder alive.

    res as jboolean
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_BloomFilter_containsStr0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong, element: JString<'local>,
) -> jboolean {
    let mut filter = Box::from_raw(raw as *mut BloomFilter);

    let element = env.get_string(&element).unwrap();

    let res = filter.contains(element.to_bytes());

    Box::into_raw(filter); // keep builder alive.

    res as jboolean
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_BloomFilter_containsBytes0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong, element: JByteArray<'local>,
) -> jboolean {
    let mut filter = Box::from_raw(raw as *mut BloomFilter);

    let element = env.convert_byte_array(element).unwrap();

    let res = filter.contains(&element);

    Box::into_raw(filter); // keep builder alive.

    res as jboolean
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_BloomFilter_clear0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong,
) {
    let mut filter = Box::from_raw(raw as *mut BloomFilter);

    filter.clear();

    Box::into_raw(filter); // keep builder alive.
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_BloomFilter_fromBytes0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, array: JByteArray<'local>, hashes: jint)
    -> jlong {
    let bytes = env.convert_byte_array(array).unwrap();

    // println!("len {} {:?}", bytes.len(), &bytes);

    let filter = Box::new(BloomFilter::from_u8_array(&bytes, hashes as u32));

    Box::into_raw(filter) as jlong
}

/// if buf.size is too large, JVM will crash.
#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_BloomFilter_getByteBuffer0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong,
) -> JByteBuffer<'local> {
    let mut filter = Box::from_raw(raw as *mut BloomFilter);
    let bytes = filter.get_u8_array();
    let mut buf = Vec::with_capacity(bytes.len());
    buf.extend_from_slice(bytes);
    // println!("{}", buf.len());
    let ptr = buf.as_mut_ptr();
    let jbuffer = env.new_direct_byte_buffer(ptr, bytes.len()).unwrap();
    Box::into_raw(filter); // keep builder alive.
    jbuffer
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_BloomFilter_getSize0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong,
) -> jint {
    let mut filter = Box::from_raw(raw as *mut BloomFilter);
    let size = filter.config().size >> 3;

    Box::into_raw(filter); // keep builder alive.

    size as jint
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_BloomFilter_copyBytes0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong, array: JByteArray<'local>,
) {
    let mut filter = Box::from_raw(raw as *mut BloomFilter);

    let bytes = filter.get_u8_array();
    let len = bytes.len();

    let i8_ptr = bytes.as_ptr() as *const i8;

    let ptr = slice_from_raw_parts(i8_ptr, len);

    let arr = unsafe { &*ptr };

    // println!("len {} {:?}", len, bytes);

    env.set_byte_array_region(array, 0, arr).unwrap();

    Box::into_raw(filter); // keep builder alive.
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_BloomFilter_union0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong, other: jlong,
) {
    let mut filter = Box::from_raw(raw as *mut BloomFilter);
    let other_filter = Box::from_raw(other as *mut BloomFilter);

    filter.union(&other_filter);

    Box::into_raw(filter); // keep builder alive.
    Box::into_raw(other_filter); // keep builder alive.
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_BloomFilter_intersect0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong, other: jlong,
) {
    let mut filter = Box::from_raw(raw as *mut BloomFilter);
    let other_filter = Box::from_raw(other as *mut BloomFilter);

    filter.intersect(&other_filter);

    Box::into_raw(filter); // keep builder alive.
    Box::into_raw(other_filter); // keep builder alive.
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_BloomFilter_isEmpty0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong,
) -> jboolean {
    let mut filter = Box::from_raw(raw as *mut BloomFilter);

    let res = filter.is_empty();

    Box::into_raw(filter); // keep builder alive.

    res as jboolean
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_BloomFilter_close0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong,
) {
    let filter = Box::from_raw(raw as *mut BloomFilter);

    drop(filter);
}


#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_CountingBloomFilter_hashes0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong,
) -> jint {
    let mut filter = Box::from_raw(raw as *mut CountingBloomFilter);

    let hashes = filter.hashes();

    Box::into_raw(filter); // keep builder alive.

    hashes as jint
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_CountingBloomFilter_addInt0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong, element: jint,
) {
    let mut filter = Box::from_raw(raw as *mut CountingBloomFilter);

    let element = element as i32;

    filter.add(&i32::to_le_bytes(element));

    Box::into_raw(filter); // keep builder alive.
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_CountingBloomFilter_removeInt0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong, element: jint,
) {
    let mut filter = Box::from_raw(raw as *mut CountingBloomFilter);

    let element = element as i32;

    filter.remove(&i32::to_le_bytes(element));

    Box::into_raw(filter); // keep builder alive.
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_CountingBloomFilter_addLong0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong, element: jlong,
) {
    let mut filter = Box::from_raw(raw as *mut CountingBloomFilter);

    let element = element as i64;

    filter.add(&i64::to_le_bytes(element));

    Box::into_raw(filter); // keep builder alive.
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_CountingBloomFilter_removeLong0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong, element: jlong,
) {
    let mut filter = Box::from_raw(raw as *mut CountingBloomFilter);

    let element = element as i64;

    filter.remove(&i64::to_le_bytes(element));

    Box::into_raw(filter); // keep builder alive.
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_CountingBloomFilter_addStr0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong, element: JString<'local>,
) {
    let mut filter = Box::from_raw(raw as *mut CountingBloomFilter);

    let element = env.get_string(&element).unwrap();

    filter.add(element.to_bytes());

    Box::into_raw(filter); // keep builder alive.
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_CountingBloomFilter_removeStr0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong, element: JString<'local>,
) {
    let mut filter = Box::from_raw(raw as *mut CountingBloomFilter);

    let element = env.get_string(&element).unwrap();

    filter.remove(element.to_bytes());

    Box::into_raw(filter); // keep builder alive.
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_CountingBloomFilter_addBytes0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong, element: JByteArray<'local>,
) {
    let mut filter = Box::from_raw(raw as *mut CountingBloomFilter);

    let element = env.convert_byte_array(element).unwrap();

    filter.add(&element);

    Box::into_raw(filter); // keep builder alive.
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_CountingBloomFilter_removeBytes0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong, element: JByteArray<'local>,
) {
    let mut filter = Box::from_raw(raw as *mut CountingBloomFilter);

    let element = env.convert_byte_array(element).unwrap();

    filter.remove(&element);

    Box::into_raw(filter); // keep builder alive.
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_CountingBloomFilter_containsInt0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong, element: jint,
) -> jboolean {
    let mut filter = Box::from_raw(raw as *mut CountingBloomFilter);

    let element = element as i32;

    let res = filter.contains(&i32::to_le_bytes(element));

    Box::into_raw(filter); // keep builder alive.

    res as jboolean
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_CountingBloomFilter_containsLong0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong, element: jlong,
) -> jboolean {
    let mut filter = Box::from_raw(raw as *mut CountingBloomFilter);

    let element = element as i64;

    let res = filter.contains(&i64::to_le_bytes(element));

    Box::into_raw(filter); // keep builder alive.

    res as jboolean
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_CountingBloomFilter_containsStr0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong, element: JString<'local>,
) -> jboolean {
    let mut filter = Box::from_raw(raw as *mut CountingBloomFilter);

    let element = env.get_string(&element).unwrap();

    let res = filter.contains(element.to_bytes());

    Box::into_raw(filter); // keep builder alive.

    res as jboolean
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_CountingBloomFilter_containsBytes0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong, element: JByteArray<'local>,
) -> jboolean {
    let mut filter = Box::from_raw(raw as *mut CountingBloomFilter);

    let element = env.convert_byte_array(element).unwrap();

    let res = filter.contains(&element);

    Box::into_raw(filter); // keep builder alive.

    res as jboolean
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_CountingBloomFilter_getByteBuffer0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong,
) -> JByteBuffer<'local> {
    let mut filter = Box::from_raw(raw as *mut CountingBloomFilter);

    let bytes = filter.get_u8_array();

    let mut buf = vec![0; bytes.len()];

    buf.copy_from_slice(bytes);

    let mut ptr = buf.as_mut_ptr();

    let jbuf = env.new_direct_byte_buffer(ptr, bytes.len()).unwrap();

    Box::into_raw(filter); // keep builder alive.

    jbuf
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_CountingBloomFilter_getSize0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong,
) -> jint {
    let mut filter = Box::from_raw(raw as *mut CountingBloomFilter);
    let size = filter.config().size >> 1;

    Box::into_raw(filter); // keep builder alive.

    size as jint
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_CountingBloomFilter_copyBytes0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong, array: JByteArray<'local>,
) {
    let mut filter = Box::from_raw(raw as *mut CountingBloomFilter);

    let bytes = filter.get_u8_array();
    let len = bytes.len();

    let i8_ptr = bytes.as_ptr() as *const i8;

    let ptr = slice_from_raw_parts(i8_ptr, len);

    let arr = unsafe { &*ptr };

    // println!("len {} {:?}", len, bytes);

    env.set_byte_array_region(array, 0, arr).unwrap();

    Box::into_raw(filter); // keep builder alive.
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_CountingBloomFilter_clear0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong,
) {
    let mut filter = Box::from_raw(raw as *mut CountingBloomFilter);

    filter.clear();

    Box::into_raw(filter); // keep builder alive.
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_CountingBloomFilter_close0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, raw: jlong,
) {
    let mut builder = Box::from_raw(raw as *mut CountingBloomFilter);

    drop(builder);
}

#[no_mangle]
pub unsafe extern "C" fn Java_io_github_yankun1992_bloom_CountingBloomFilter_fromBytes0<'local>(
    mut env: JNIEnv<'local>, clz: JClass<'local>, array: JByteArray<'local>, hashes: jint, enable_repeat_insert: jboolean)
    -> jlong {
    let bytes = env.convert_byte_array(array).unwrap();

    let enable_repeat_insert = enable_repeat_insert != 0;

    let filter = Box::new(CountingBloomFilter::from_u8_array(&bytes, hashes as u32, enable_repeat_insert));

    Box::into_raw(filter) as jlong
}