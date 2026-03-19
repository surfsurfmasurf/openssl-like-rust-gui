# OpenSSL Tool - Rust GUI Edition

OpenSSL의 핵심 암호화 기능을 Rust로 재구현한 데스크톱 GUI 애플리케이션입니다.
**iced** 프레임워크(Elm 아키텍처 기반 순수 Rust GUI)를 사용하여 그래픽 인터페이스를 제공합니다.

![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

---

## 빌드 및 실행

```bash
# 빌드
cargo build --release

# 실행
cargo run --release
```

**요구사항**: Rust 1.70+ (2021 edition)

---

## 기능 목록 (11개 탭)

| 탭 | OpenSSL 대응 명령어 | 설명 |
|---|---|---|
| **Hash** | `openssl dgst` | MD5, SHA-1, SHA-256, SHA-384, SHA-512 해시 |
| **Symmetric** | `openssl enc` | AES-128/192/256 (CBC, GCM), DES, 3DES 암복호화 |
| **Asymmetric** | `openssl genpkey` | RSA-2048/4096, ECDSA P-256/P-384 키페어 생성 |
| **Certificates** | `openssl req`, `openssl x509` | X.509 자체서명 인증서, CSR 생성, 인증서 파싱 |
| **Signatures** | `openssl dgst -sign/-verify` | RSA-SHA256, ECDSA 디지털 서명 및 검증 |
| **Base64** | `openssl base64` | Base64 인코딩/디코딩 |
| **Random** | `openssl rand` | 암호학적 난수 생성 (Hex, Base64, Raw) |
| **File Encrypt** | `openssl enc -in -out` | AES-256-GCM 파일 암복호화 (패스워드 기반) |
| **TLS Connect** | `openssl s_client` | TLS 연결 테스트, 서버 인증서 체인 조회 |
| **Key Inspect** | `openssl pkey -text`, `openssl rsa -text` | PEM 키/인증서 내용 파싱 및 표시 |
| **Algorithms** | `openssl list -cipher-algorithms` | 지원되는 전체 알고리즘 목록 |

---

## 프로젝트 구조

```
openssl_like/
├── Cargo.toml              # 의존성 정의
├── README.md
└── src/
    ├── main.rs             # 앱 진입점
    ├── app.rs              # 앱 코어 (상태관리, 메시지 라우팅)
    ├── theme.rs            # UI 색상/스타일 상수
    ├── utils.rs            # 공통 유틸리티 (hex 변환)
    ├── crypto/             # 암호화 엔진 (순수 로직, UI 무관)
    │   ├── mod.rs          #   CryptoError 통합 에러 타입
    │   ├── hashing.rs      #   해시 함수 (SHA, MD5)
    │   ├── symmetric.rs    #   대칭암호 (AES-CBC/GCM, DES, 3DES)
    │   ├── asymmetric.rs   #   비대칭키 생성 (RSA, ECDSA)
    │   ├── signatures.rs   #   디지털 서명/검증
    │   ├── certificates.rs #   X.509 인증서, CSR
    │   ├── encoding.rs     #   Base64
    │   ├── random.rs       #   난수 생성
    │   ├── file_ops.rs     #   파일 암복호화
    │   ├── tls.rs          #   TLS 클라이언트 연결
    │   └── key_inspect.rs  #   PEM 키 파싱/검사
    └── ui/                 # GUI 레이어 (iced 위젯)
        ├── mod.rs
        ├── sidebar.rs          # 사이드바 네비게이션
        ├── hash_tab.rs         # 해시 탭 UI
        ├── symmetric_tab.rs    # 대칭암호 탭 UI
        ├── asymmetric_tab.rs   # 비대칭키 탭 UI
        ├── certificate_tab.rs  # 인증서 탭 UI (서브탭 3개)
        ├── signature_tab.rs    # 서명 탭 UI
        ├── encoding_tab.rs     # Base64 탭 UI
        ├── random_tab.rs       # 난수 탭 UI
        ├── file_encrypt_tab.rs # 파일암호화 탭 UI
        ├── tls_tab.rs          # TLS 연결 탭 UI
        ├── key_inspect_tab.rs  # 키 검사 탭 UI
        └── ciphers_tab.rs      # 알고리즘 목록 탭 UI
```

---

## 아키텍처 상세 설명

### 1. 계층 분리: `crypto/` vs `ui/`

프로젝트는 **암호화 로직**과 **UI 레이어**를 엄격하게 분리합니다.

```
┌─────────────────────────────────────────────┐
│  UI Layer (src/ui/)                         │
│  iced 위젯, 사용자 입력 처리, 결과 표시       │
├─────────────────────────────────────────────┤
│  App Core (src/app.rs)                      │
│  상태 관리, 메시지 라우팅, 탭 전환            │
├─────────────────────────────────────────────┤
│  Crypto Layer (src/crypto/)                 │
│  순수 함수, UI 무관, 단위 테스트 가능         │
└─────────────────────────────────────────────┘
```

- **`crypto/`** 모듈은 UI에 대한 의존성이 전혀 없습니다. `&[u8]` → `Result<Vec<u8>>` 형태의 순수 함수로만 구성되어 있어, CLI 도구나 라이브러리로도 재사용할 수 있습니다.
- **`ui/`** 모듈은 각 탭별로 `State` 구조체, `Msg` enum, `update()`, `view()` 함수를 노출하는 Elm 아키텍처 패턴을 따릅니다.

### 2. Elm 아키텍처 (The Elm Architecture, TEA)

iced 프레임워크의 핵심 패턴:

```
사용자 입력 → Message → update(state, msg) → 새 State → view(state) → UI 렌더링
                ↑                                                        │
                └────────────────────────────────────────────────────────┘
```

각 탭 모듈은 이 패턴을 동일하게 구현합니다:

```rust
// 탭별 패턴 (예: hash_tab.rs)
pub struct State { ... }           // 탭의 모든 상태
pub enum Msg { ... }               // 탭에서 발생 가능한 모든 이벤트
pub fn update(state, msg) -> ...   // 상태 변경 로직
pub fn view(state) -> Element      // UI 렌더링
```

### 3. `app.rs` - 중앙 라우터

`App` 구조체가 모든 탭의 상태를 소유하고, `Message` enum으로 이벤트를 라우팅합니다:

```rust
pub enum Message {
    TabSelected(Tab),          // 탭 전환
    Hash(hash_tab::Msg),       // 해시 탭 이벤트 래핑
    Symmetric(symmetric_tab::Msg),
    Tls(tls_tab::Msg),
    // ... 각 탭별 래핑
}
```

`update()` 에서 패턴 매칭으로 각 탭의 update 함수에 위임하고, `view()` 에서 활성 탭에 따라 해당 탭의 view를 렌더링합니다. `.map(Message::TabName)` 으로 자식 메시지를 부모 메시지로 변환합니다.

### 4. `crypto/` 모듈 상세

#### `crypto/mod.rs` - 통합 에러 타입

```rust
pub enum CryptoError {
    InvalidKeyLength { expected, got },
    HexDecode(hex::FromHexError),
    EncryptionFailed(String),
    DecryptionFailed(String),
    PemError(String),
    CertificateError(String),
    // ...
}
```

`thiserror`를 사용하여 `Display`, `Error` 트레잇을 자동 구현합니다. 모든 crypto 함수는 `Result<T, CryptoError>`를 반환합니다.

#### `crypto/symmetric.rs` - 대칭암호 엔진

가장 복잡한 모듈로, 5개 알고리즘 × 2개 모드 조합을 처리합니다:

| 알고리즘 | CBC | GCM |
|---|---|---|
| AES-128 | ✅ cbc + aes | ✅ aes-gcm |
| AES-192 | ✅ cbc + aes | ❌ (라이브러리 미지원) |
| AES-256 | ✅ cbc + aes | ✅ aes-gcm |
| DES | ✅ cbc + des | ❌ |
| 3DES | ✅ cbc + des | ❌ |

- CBC 모드: `cbc::Encryptor<aes::Aes256>` + PKCS#7 패딩
- GCM 모드: `aes_gcm::Aes256Gcm` (인증 암호화, 패딩 불필요)
- `generate_key()` / `generate_iv()`: OS CSPRNG으로 적절한 길이의 키/IV 자동 생성

#### `crypto/asymmetric.rs` - 비대칭키 생성

RSA 키 생성은 CPU 집약적이므로(특히 4096비트) **비동기 처리**합니다:

```rust
// asymmetric_tab.rs에서
iced::Task::perform(
    async move {
        tokio::task::spawn_blocking(move || {
            asymmetric::generate_keypair(algo)
        }).await
    },
    Msg::Generated,
)
```

`Task::perform` → `spawn_blocking`으로 UI 스레드 블로킹을 방지합니다.

#### `crypto/tls.rs` - TLS 클라이언트

`rustls` + `webpki-roots`를 사용한 TLS 연결:

1. 시스템 루트 인증서 로드 (Mozilla 인증서 번들)
2. TCP 연결 수립 → TLS 핸드셰이크
3. 프로토콜 버전, 암호 스위트, 인증서 체인 추출
4. `x509-parser`로 서버 인증서 파싱

#### `crypto/file_ops.rs` - 파일 암호화

파일 포맷: `[16-byte salt][12-byte nonce][AES-256-GCM ciphertext + auth tag]`

키 유도: SHA-256 기반 반복 해싱 (10,000 iterations)
```
key = SHA256(salt || password)
for i in 0..10000: key = SHA256(key || salt)
```

#### `crypto/key_inspect.rs` - 키 파싱

PEM 헤더를 기반으로 키 타입을 자동 감지합니다:

```
"RSA PRIVATE KEY" → PKCS#1 RSA 개인키 파싱
"PRIVATE KEY"     → PKCS#8 (RSA → ECDSA P-256 → P-384 순서로 시도)
"PUBLIC KEY"      → RSA → ECDSA 순서로 시도
"CERTIFICATE"     → X.509 인증서에서 공개키 정보 추출
```

### 5. UI 레이아웃

```
+--------------------+------------------------------------------+
| [#] Hash           |                                          |
| [E] Symmetric      |     << 활성 탭 콘텐츠 영역 >>             |
| [K] Asymmetric     |                                          |
| [C] Certificates   |     입력 필드, 버튼, 결과 표시             |
| [S] Signatures     |                                          |
| [B] Base64         |                                          |
| [R] Random         |                                          |
| [F] File Encrypt   |                                          |
| [T] TLS Connect    |                                          |
| [I] Key Inspect    |                                          |
| [?] Algorithms     |                                          |
+--------------------+------------------------------------------+
| 상태바: Ready / Hash computed / Encrypted successfully         |
+---------------------------------------------------------------+
```

- **사이드바** (220px, 다크 테마): 탭 전환 버튼, 활성 탭 하이라이트
- **콘텐츠 영역**: 각 탭의 `view()` 함수가 렌더링
- **상태바**: 마지막 작업 결과 표시

### 6. 의존성 구조

```
iced 0.13 ─── GUI 프레임워크 (wgpu 렌더링)
tokio 1   ─── 비동기 런타임 (RSA 키 생성, TLS 연결)

sha1, sha2, md-5 ─── RustCrypto 해시
aes-gcm, cbc, aes, des ─── RustCrypto 대칭암호
rsa, p256, p384, ecdsa ─── RustCrypto 비대칭키/서명
rcgen ─── 인증서/CSR 생성
x509-parser ─── 인증서 파싱
rustls + webpki-roots ─── TLS 클라이언트

base64, hex, pem ─── 인코딩
rand (OsRng) ─── CSPRNG
thiserror ─── 에러 타입 매크로
```

**설계 원칙**: `ring` 대신 **RustCrypto 생태계**를 선택했습니다.
- 순수 Rust (C 의존성 없음)
- DES/3DES 포함 전체 알고리즘 커버
- 크로스 플랫폼 빌드 간편

---

## 라이선스

MIT License
