## 무엇을 할 것인가
순수 Rust만 사용한 프로젝트로 다음과 같은 것들을 테스트
- Rust의 패키지로 클라이언트-사이드에서 할 수 있는 작업을 수행(CPU를 크게 소모해야 함)
- 페이지 렌더링 또한 Rust 패키지를 사용해야 함
- 동일한 동작을 Node.js 환경에서 수행하는 NPM 패키지 배포가 가능한지 테스트

## 왜 할 것인가
- 연구 목적, 현재 Rust+WASM 활용으로 어디까지 가능한지 테스트 목적

## 관련 링크
- https://www.arewewebyet.org/
- https://yew.rs/
- https://rustwasm.github.io/wasm-bindgen/examples/raytrace.html
  - https://github.com/rustwasm/wasm-bindgen/tree/main/examples/raytrace-parallel
- https://github.com/futursolo/stylist-rs

## Commands
- start: trunk serve --address=0.0.0.0
