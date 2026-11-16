# ex9: 해시맵(HashMap) 및 Entry API

## 1. 개요
키-값(Key-Value) 쌍을 저장하는 `HashMap`의 생성, 두 벡터의 결합을 통한 맵 생성, 키-값 순회, 그리고 `entry` API를 활용한 빈도수 계산을 학습하는 실습입니다.

## 2. 주요 학습 내용
- 해시맵 생성 및 데이터 삽입: `HashMap::new()`, `insert()` 메서드 및 `HashMap::from`을 사용한 초기화를 실습합니다.
- 이터레이터 활용: `into_iter().zip().collect()`를 사용하여 두 벡터를 키-값 해시맵으로 변환합니다.
- Entry API: `entry(key).or_insert(default)`를 활용하여 키의 존재 여부에 따라 기본값을 설정하고 값을 갱신(빈도수 계산)합니다.
- 순회: 해시맵의 참조를 순회(`&(k, val)`)하거나 `keys()` 메서드를 통해 키 목록을 확인합니다.

## 3. 실행 방법
```bash
cargo run
```
