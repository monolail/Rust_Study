# ex13: 이터레이터 응용 실습 (집계, 슬라이싱, 변환 및 filter_map)

## 1. 개요
Rust의 이터레이터를 활용한 다양한 집계 함수(`sum`, `product`, `count`, `max`), 슬라이싱 메서드(`take`, `take_while`, `step_by`), 그리고 `filter_map`을 이용한 파싱 및 변환 기법을 학습하는 실습입니다.

## 2. 주요 학습 내용
- 기본 집계 연산: `sum()`, `product()`, `count()`, `max()`를 통한 범위 및 벡터 원소의 연산.
- 이터레이터 제어 및 필터링:
  - `take(n)`: 앞에서부터 n개의 요소를 선택.
  - `take_while()`: 조건이 거짓이 되는 순간 순회를 중단.
  - `step_by(n)`: n개 간격으로 건너뛰며 요소를 추출.
- 변환 및 조건 체이닝: `filter().map().sum()`과 같이 여러 이터레이터 어댑터를 결합한 데이터 가공.
- `filter_map()`: 변환과 필터링을 한 번에 수행하여 `Result`(`parse::<i32>().ok()`)나 `Option`(`to_digit(10)`)에서 유효한 값만 추출 및 컬렉션 수집.

## 3. 실행 방법
```bash
cargo test -- --nocapture
```
