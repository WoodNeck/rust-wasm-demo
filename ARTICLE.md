## Rust 웹개발
### 언제 Rust + Webassembly를 사용할 수 있는가?
- https://rustwasm.github.io/docs/book/why-rust-and-webassembly.html

일반적으로 높은 퍼포먼스를 필요로 하는 어플리케이션 작성시에 webassembly를 필요로 할 수 있으며, 그 때 언어로 Rust 사용을 고려해볼 수 있습니다.
Rust를 사용했을 때 좋은 점은
- Rust로 작성된 외부 crate를 사용 가능, 다만 아래와 같은 경우 사용 불가
  - File I/O API를 사용시
  - 쓰레드를 생성할 경우
    - tokio와 같은 크레이트로 우회 가능
  - C나 시스템 라이브러리에 의존성이 있을 경우

### 그럼 일반적으로 어떤 경우에 적용하면 좋은가?
높은 퍼포먼스를 필요로 하는 작업
- Parser (XML, CSV, TTF font, HTML, 3D 모델, ...)
- Text processing (Regex, diff, sanitize, ...)

### 그 외 장점
- Compile-time assertion
  - https://crates.io/crates/static_assertions
- Inline function (#[inline])
- Rust 자체의 문법적 강점
  - 소유권 (가비지 컬렉션 없이 메모리 안정성 보장)

## 웹 서버
- https://actix.rs/
- https://rocket.rs/

## Rust Cross-Platform GUI
- Dioxus, Egui, Iced, Tauri + Yew
