extern crate wasm_bindgen_test;
extern crate web_sys;
extern crate easybench_wasm;

use wasm_bindgen_test::*;
use easybench_wasm::{ bench, bench_env };
use web_sys::console;

// This runs a unit test in the browser, so it can use browser APIs.
wasm_bindgen_test_configure!(run_in_browser);

fn fib(n: usize) -> usize {
  let mut i = 0; let mut sum = 0; let mut last = 0; let mut curr = 1usize;
  while i < n - 1 {
    sum = curr.wrapping_add(last);
    last = curr;
    curr = sum;
    i += 1;
  }
  sum
}

#[wasm_bindgen_test]
fn doctests_again() {
  println!();
  console::log_1(&format!("fib 200: {}", bench(|| fib(200) )).into());
  console::log_1(&format!("fib 200: {}", bench(|| fib(200) )).into());
  console::log_1(&format!("fib 500: {}", bench(|| fib(500) )).into());
  console::log_1(&format!("reverse: {}", bench_env(vec![0;100], |xs| xs.reverse())).into());
  console::log_1(&format!("sort:    {}", bench_env(vec![0;100], |xs| xs.sort())).into());

  // This is fine:
  console::log_1(&format!("fib 1:   {}", bench(|| fib(500) )).into());
  // This is NOT fine:
  console::log_1(&format!("fib 2:   {}", bench(|| { fib(500); } )).into());
  // This is also fine, but a bit weird:
  console::log_1(&format!("fib 3:   {}", bench_env(0, |x| { *x = fib(500); } )).into());
}

#[wasm_bindgen_test]
fn very_quick() {
  println!();
  console::log_1(&format!("very quick: {}", bench(|| {})).into());
}

#[wasm_bindgen_test]
fn noop() {
  println!();
  console::log_1(&format!("noop base: {}", bench(                    | | {})).into());
  console::log_1(&format!("noop 0:    {}", bench_env(vec![0u64;0],   |_| {})).into());
  console::log_1(&format!("noop 16:   {}", bench_env(vec![0u64;16],  |_| {})).into());
  console::log_1(&format!("noop 64:   {}", bench_env(vec![0u64;64],  |_| {})).into());
  console::log_1(&format!("noop 256:  {}", bench_env(vec![0u64;256], |_| {})).into());
  console::log_1(&format!("noop 512:  {}", bench_env(vec![0u64;512], |_| {})).into());
}

#[wasm_bindgen_test]
fn ret_value() {
  println!();
  console::log_1(&format!("no ret 32:    {}", bench_env(vec![0u64;32],   |x| { x.clone() })).into());
  console::log_1(&format!("return 32:    {}", bench_env(vec![0u64;32],   |x| x.clone())).into());
  console::log_1(&format!("no ret 256:   {}", bench_env(vec![0u64;256],  |x| { x.clone() })).into());
  console::log_1(&format!("return 256:   {}", bench_env(vec![0u64;256],  |x| x.clone())).into());
  console::log_1(&format!("no ret 1024:  {}", bench_env(vec![0u64;1024], |x| { x.clone() })).into());
  console::log_1(&format!("return 1024:  {}", bench_env(vec![0u64;1024], |x| x.clone())).into());
  console::log_1(&format!("no ret 4096:  {}", bench_env(vec![0u64;4096], |x| { x.clone() })).into());
  console::log_1(&format!("return 4096:  {}", bench_env(vec![0u64;4096], |x| x.clone())).into());
  console::log_1(&format!("no ret 50000: {}", bench_env(vec![0u64;50000], |x| { x.clone() })).into());
  console::log_1(&format!("return 50000: {}", bench_env(vec![0u64;50000], |x| x.clone())).into());
}
