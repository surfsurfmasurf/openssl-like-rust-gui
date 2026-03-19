# 코드 워크스루: 요청 하나를 따라가는 여행

이 문서는 사용자의 **클릭 한 번**이 코드 내부에서 어떻게 흘러가는지를
두 가지 시나리오로 처음부터 끝까지 따라갑니다.

---

## 시나리오 1: SHA-256 해시 계산 (가장 단순한 흐름)

> 사용자 행동: Hash 탭에서 "hello"를 입력하고 [Compute Hash] 버튼 클릭

### STEP 0: 앱이 시작될 때

```
main.rs:9  →  iced::application("OpenSSL Tool", App::update, App::view)
                   .run_with(App::new)
```

`iced::application()`에 3개의 함수 포인터를 등록합니다:
- **제목**: `"OpenSSL Tool - Rust Edition"`
- **update**: `App::update` — 모든 이벤트 처리
- **view**: `App::view` — 매 프레임 UI 렌더링

`run_with(App::new)`가 호출되면:

```
app.rs:57  →  App::new() → (App { active_tab: Tab::Hashing, ... }, Task::none())
```

11개 탭의 초기 상태가 모두 생성됩니다. `hash_state`의 초기값:

```
hash_tab.rs:22  →  State {
                      algorithm: Some(HashAlgorithm::Sha256),  // 기본값 SHA-256
                      input: "",
                      result: "",
                    }
```

---

### STEP 1: 화면 렌더링 — `App::view()` 호출

iced는 매 프레임마다 `view()`를 호출하여 전체 UI 트리를 다시 만듭니다.

```
app.rs:159  →  pub fn view(&self) -> Element<'_, Message> {

app.rs:160  →      let sidebar = sidebar::view(&self.active_tab);
                    // 사이드바: 11개 탭 버튼 렌더링
```

`sidebar.rs`에서 `TABS` 배열을 순회하며 각 버튼을 만듭니다:

```
sidebar.rs:57  →  button(label).on_press(Message::TabSelected(sb.tab))
                  // 클릭하면 Message::TabSelected(Tab::Hashing) 발생
```

활성 탭이 `Tab::Hashing`이므로:

```
app.rs:163  →  Tab::Hashing => hash_tab::view(&self.hash_state).map(Message::Hash),
                               ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                               hash_tab 모듈이 자기만의 Element<Msg> 반환
                                                                .map(Message::Hash)
                                                                ~~~~~~~~~~~~~~~~~~~~
                                                                hash_tab::Msg를 App의 Message::Hash로 래핑
```

#### hash_tab::view() 내부

```
hash_tab.rs:52  →  pub fn view(state: &State) -> Element<'_, Msg> {
```

이 함수가 만드는 위젯 트리:

```
card (흰색 카드 컨테이너)
  └── Column
        ├── text "Hash Functions"              (제목)
        ├── text "Compute cryptographic..."     (설명)
        ├── vertical_space(15)
        ├── text "Algorithm"
        ├── pick_list [MD5, SHA-1, SHA-256▼, SHA-384, SHA-512]
        │     └── on_select → Msg::AlgorithmSelected(algo)
        ├── vertical_space(10)
        ├── text "Input Text"
        ├── text_input "Enter text to hash..."
        │     └── on_input → Msg::InputChanged(val)    ← 키 입력마다 발생
        ├── vertical_space(10)
        ├── button "Compute Hash"
        │     └── on_press → Msg::Compute              ← ★ 이 버튼!
        ├── vertical_space(15)
        └── (결과 영역 — 아직 비어있음)
```

**핵심**: 모든 위젯은 "이벤트가 발생하면 이 Msg를 보내라"는 선언만 합니다.
실제 이벤트 처리는 `update()`에서 합니다.

---

### STEP 2: 사용자가 "hello" 입력

사용자가 키보드로 `h`, `e`, `l`, `l`, `o`를 칠 때마다:

```
text_input → on_input → Msg::InputChanged("h")
text_input → on_input → Msg::InputChanged("he")
text_input → on_input → Msg::InputChanged("hel")
text_input → on_input → Msg::InputChanged("hell")
text_input → on_input → Msg::InputChanged("hello")
```

각 메시지는 `.map(Message::Hash)`를 거쳐 `Message::Hash(Msg::InputChanged("hello"))`로 래핑됩니다.

```
app.rs:88  →  Message::Hash(msg) => {
app.rs:89  →      if let Some(status) = hash_tab::update(&mut self.hash_state, msg) {
                                        ~~~~~~~~~~~~~~~~
                                        hash_tab 모듈의 update에 위임
```

```
hash_tab.rs:36  →  Msg::InputChanged(val) => {
hash_tab.rs:37  →      state.input = val;    // "hello" 저장
hash_tab.rs:38  →      None                  // 상태바 업데이트 없음
                   }
```

상태가 변경되었으므로 iced가 `view()`를 다시 호출 → 입력 필드에 "hello" 표시.

---

### STEP 3: [Compute Hash] 클릭 — 이벤트 발생

버튼 클릭 → `Msg::Compute` 생성 → `.map(Message::Hash)` → `Message::Hash(Msg::Compute)`.

```
app.rs:88  →  Message::Hash(msg) => {
                  // msg = Msg::Compute
app.rs:89  →      if let Some(status) = hash_tab::update(&mut self.hash_state, msg) {
app.rs:90  →          self.status = status;
                       // "SHA-256 hash computed" → 상태바에 표시
                  }
app.rs:92  →      Task::none()   // 비동기 작업 없음
```

---

### STEP 4: hash_tab::update() — 암호 로직 호출

```
hash_tab.rs:40  →  Msg::Compute => {
hash_tab.rs:41  →      if let Some(algo) = state.algorithm {
                            // algo = HashAlgorithm::Sha256

hash_tab.rs:42  →          state.result = hashing::compute_hash(algo, state.input.as_bytes());
                            ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                            crypto 레이어 호출! "hello".as_bytes() = [104, 101, 108, 108, 111]

hash_tab.rs:43  →          Some(format!("{} hash computed", algo))
                            // → Some("SHA-256 hash computed")
                       }
```

**여기가 UI → crypto 경계입니다.**

---

### STEP 5: crypto::hashing::compute_hash() — 실제 해시 계산

```
hashing.rs:35  →  pub fn compute_hash(algorithm: HashAlgorithm, data: &[u8]) -> String {
hashing.rs:36  →      match algorithm {
                          ...
hashing.rs:45  →          HashAlgorithm::Sha256 => {
hashing.rs:46  →              let result = sha2::Sha256::digest(data);
                              // sha2 크레이트의 Digest 트레잇 사용
                              // data = [104, 101, 108, 108, 111]
                              // result = [0x2c, 0xf2, 0x4d, 0xba, ...]  (32바이트)

hashing.rs:47  →              hex::encode(result)
                              // → "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e7304..."
                          }
```

이 함수는:
1. `sha2::Sha256::digest()` — Rust의 `digest::Digest` 트레잇 호출
2. `hex::encode()` — 32바이트 → 64자 16진수 문자열로 변환

**반환값**: `"2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"`

---

### STEP 6: 결과 표시 — view() 재호출

`state.result`에 값이 저장되었으므로, 다음 `view()` 호출에서:

```
hash_tab.rs:75  →  let result_section = if !state.result.is_empty() {
                       // 이제 true!

hash_tab.rs:77  →      let result_box = container(
hash_tab.rs:78  →          text(&state.result)   // "2cf24dba..."
hash_tab.rs:79  →              .size(13)
hash_tab.rs:80  →              .font(iced::Font::MONOSPACE)   // 고정폭 글꼴
                       );

hash_tab.rs:96  →      let bit_len = state.result.len() * 4;
                        // 64 * 4 = 256
hash_tab.rs:97  →      let info = text(format!("{} bits ({} hex chars)", bit_len, state.result.len()));
                        // "256 bits (64 hex chars)"
```

최종 UI:
```
┌──────────────────────────────────────────────┐
│  Hash Functions                              │
│  Compute cryptographic hash digests...       │
│                                              │
│  Algorithm: [SHA-256 ▼]                      │
│                                              │
│  Input Text:                                 │
│  ┌──────────────────────────────────────┐    │
│  │ hello                                │    │
│  └──────────────────────────────────────┘    │
│                                              │
│  [Compute Hash]                              │
│                                              │
│  Result:                                     │
│  ┌──────────────────────────────────────┐    │
│  │ 2cf24dba5fb0a30e26e83b2ac5b9e29e... │    │
│  └──────────────────────────────────────┘    │
│  [Copy]  256 bits (64 hex chars)             │
└──────────────────────────────────────────────┘
  상태바: SHA-256 hash computed
```

---

### 시나리오 1 전체 흐름 요약

```
[사용자 클릭]
    │
    ▼
iced 런타임: 이벤트 감지
    │
    ▼
hash_tab::view()에서 등록한 on_press
    │  Msg::Compute 생성
    ▼
.map(Message::Hash) 래핑
    │  Message::Hash(Msg::Compute)
    ▼
App::update() 패턴 매칭
    │  Message::Hash(msg) 분기
    ▼
hash_tab::update(state, msg)
    │  Msg::Compute 분기
    ▼
crypto::hashing::compute_hash(Sha256, b"hello")   ← UI→crypto 경계
    │  sha2::Sha256::digest() + hex::encode()
    ▼
state.result = "2cf24dba..."  (상태 변경)
    │
    ▼
iced 런타임: 상태 변경 감지 → view() 재호출
    │
    ▼
hash_tab::view(): result가 비어있지 않으므로 결과 섹션 렌더링
    │
    ▼
[화면에 해시 결과 표시]
```

---

## 시나리오 2: AES-256-GCM 암호화 (복잡한 흐름)

> 사용자 행동: Symmetric 탭 → AES-256 + GCM 선택 → Gen Key → Gen IV → "secret" 입력 → [Encrypt >>] 클릭

이 시나리오는 여러 단계의 상호작용과 에러 처리를 포함합니다.

### STEP 0: 탭 전환

사이드바에서 "Symmetric" 클릭:

```
sidebar.rs:58  →  button(label).on_press(Message::TabSelected(Tab::SymmetricEncryption))
```

```
app.rs:84  →  Message::TabSelected(tab) => {
app.rs:85  →      self.active_tab = tab;     // Tab::SymmetricEncryption
app.rs:86  →      Task::none()
              }
```

다음 `view()`에서:

```
app.rs:164  →  Tab::SymmetricEncryption => {
app.rs:165  →      symmetric_tab::view(&self.symmetric_state).map(Message::Symmetric)
               }
```

이제 Symmetric 탭의 UI가 렌더링됩니다. 초기 상태:

```
symmetric_tab.rs:33  →  State {
                           algorithm: Some(SymAlgorithm::Aes256),  // 기본
                           mode: Some(CipherMode::Gcm),            // 기본
                           key_hex: "",
                           iv_hex: "",
                           plaintext: "",
                           ciphertext: "",
                           error: "",
                        }
```

---

### STEP 1: [Gen Key] 클릭 — 키 생성

```
symmetric_tab.rs:139  →  styled_btn("Gen Key", Msg::GenerateKey)
```

클릭 → `Msg::GenerateKey` → `.map(Message::Symmetric)` → `Message::Symmetric(Msg::GenerateKey)`

```
app.rs:94  →  Message::Symmetric(msg) => {
app.rs:95  →      if let Some(status) = symmetric_tab::update(&mut self.symmetric_state, msg) {
```

```
symmetric_tab.rs:54  →  Msg::GenerateKey => {
symmetric_tab.rs:55  →      if let Some(algo) = state.algorithm {
                                 // algo = SymAlgorithm::Aes256

symmetric_tab.rs:56  →          let key = symmetric::generate_key(algo);
                                 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                                 crypto 레이어 호출!
```

#### crypto::symmetric::generate_key()

```
symmetric.rs:248  →  pub fn generate_key(algo: SymAlgorithm) -> Vec<u8> {
symmetric.rs:249  →      use rand::RngCore;
symmetric.rs:250  →      let mut key = vec![0u8; algo.key_size()];
                          // algo.key_size() = 32 (AES-256은 32바이트 키)

symmetric.rs:251  →      rand::rngs::OsRng.fill_bytes(&mut key);
                          // OS의 CSPRNG(암호학적 보안 난수 생성기) 사용
                          // Windows: BCryptGenRandom
                          // Linux: getrandom() syscall

symmetric.rs:252  →      key   // 예: [0xa3, 0x7f, 0x12, ..., 0xb9]  (32바이트)
                  }
```

UI로 돌아와서:

```
symmetric_tab.rs:57  →  state.key_hex = hex::encode(&key);
                         // → "a37f12...b9" (64자 hex 문자열)
```

→ view() 재호출 → Key 입력 필드에 자동 채워짐

---

### STEP 2: [Gen IV] 클릭 — IV/Nonce 생성

같은 패턴:

```
symmetric_tab.rs:62  →  Msg::GenerateIv => {
symmetric_tab.rs:63  →      let iv = symmetric::generate_iv(algo, mode);
```

```
symmetric.rs:255  →  pub fn generate_iv(algo: SymAlgorithm, mode: CipherMode) -> Vec<u8> {
symmetric.rs:257  →      let mut iv = vec![0u8; algo.iv_size(mode)];
                          // GCM 모드: iv_size = 12 바이트 (96-bit nonce)
                          // CBC 모드라면: iv_size = 16 바이트 (128-bit IV)

symmetric.rs:258  →      rand::rngs::OsRng.fill_bytes(&mut iv);
                  }
```

→ `state.iv_hex` = 24자 hex 문자열

---

### STEP 3: "secret" 입력

매 키 입력마다:

```
symmetric_tab.rs:52  →  Msg::PlaintextChanged(v) => { state.plaintext = v; None }
```

---

### STEP 4: [Encrypt >>] 클릭 — 핵심 흐름

```
symmetric_tab.rs:168  →  styled_btn("Encrypt >>", Msg::Encrypt)
```

→ `Message::Symmetric(Msg::Encrypt)` → `symmetric_tab::update()`

```
symmetric_tab.rs:68  →  Msg::Encrypt => {
symmetric_tab.rs:69  →      let (algo, mode) = match (state.algorithm, state.mode) {
symmetric_tab.rs:70  →          (Some(a), Some(m)) => (a, m),
                                // a = SymAlgorithm::Aes256, m = CipherMode::Gcm
symmetric_tab.rs:71  →          _ => return None,
                            };
```

#### 4-1: Hex 문자열 → 바이트 디코딩

```
symmetric_tab.rs:73  →  match (hex::decode(&state.key_hex), hex::decode(&state.iv_hex)) {
symmetric_tab.rs:74  →      (Ok(key), Ok(iv)) => {
                                // key: Vec<u8> (32바이트)
                                // iv: Vec<u8> (12바이트)
```

잘못된 hex를 입력했다면:

```
symmetric_tab.rs:84  →  _ => { state.error = "Invalid hex in key or IV".into(); None }
```

→ 에러 컨테이너 렌더링 (빨간 테두리 박스)

#### 4-2: crypto::symmetric::encrypt() 호출

```
symmetric_tab.rs:75  →  match symmetric::encrypt(algo, mode, &key, &iv, state.plaintext.as_bytes()) {
```

```
symmetric.rs:75   →  pub fn encrypt(algo, mode, key, iv, plaintext) -> Result<Vec<u8>, CryptoError> {
symmetric.rs:82   →      match mode {
symmetric.rs:83   →          CipherMode::Gcm => encrypt_gcm(algo, key, iv, plaintext),
                              // GCM 분기로 진입
```

```
symmetric.rs:101  →  fn encrypt_gcm(algo, key, nonce, plaintext) -> Result<...> {
symmetric.rs:107  →      let nonce = GcmNonce::from_slice(nonce);
                          // 12바이트 → GCM Nonce 타입으로 변환

symmetric.rs:116  →      SymAlgorithm::Aes256 => {
symmetric.rs:117  →          let cipher = Aes256Gcm::new_from_slice(key)
                              // 32바이트 키로 AES-256-GCM 인스턴스 생성
                              .map_err(|e| CryptoError::EncryptionFailed(...))?;

symmetric.rs:119  →          cipher.encrypt(nonce, plaintext)
                              // AEAD 암호화 수행:
                              // 1. AES-256으로 평문 암호화
                              // 2. GCM 인증 태그(16바이트) 생성
                              // 3. ciphertext || auth_tag 반환
                              .map_err(|e| CryptoError::EncryptionFailed(...))?
                          }
```

반환값: `Ok(Vec<u8>)` — 암호문 + 인증 태그 (평문보다 16바이트 더 김)

#### 4-3: 결과를 Base64로 인코딩

```
symmetric_tab.rs:76  →  Ok(ct) => {
symmetric_tab.rs:77  →      use base64::{engine::general_purpose::STANDARD, Engine};
symmetric_tab.rs:78  →      state.ciphertext = STANDARD.encode(&ct);
                              // 바이너리 → Base64 문자열
                              // 예: "AxK9f2...Qw=="

symmetric_tab.rs:79  →      Some("Encrypted successfully".into())
                              // → 상태바에 표시
```

#### 4-4: 에러가 발생한 경우

만약 키 길이가 맞지 않으면:

```
symmetric.rs:117  →  Aes256Gcm::new_from_slice(key)  → Err(InvalidLength)
                     → CryptoError::EncryptionFailed("invalid length")
```

이 에러가 UI까지 올라옵니다:

```
symmetric_tab.rs:81  →  Err(e) => { state.error = e.to_string(); None }
```

view()에서:

```
symmetric_tab.rs:186  →  if !state.error.is_empty() {
                             container(text(&state.error).color(theme::ERROR))
                                 .style(|_| container::Style {
                                     background: Some(Color::from_rgb(1.0, 0.95, 0.95)),
                                     border: Border { color: theme::ERROR, ... },
                                 })
                             // → 빨간 배경, 빨간 테두리의 에러 박스 표시
```

---

### STEP 5: 결과 화면

view() 재호출 → Ciphertext 필드에 Base64 값이 표시됨:

```
┌─────────────────────────────────────────────────────────────────┐
│  Symmetric Encryption                                           │
│  AES (CBC/GCM), DES, 3DES encryption and decryption            │
│                                                                 │
│  [AES-256 ▼]  [GCM ▼]                                          │
│                                                                 │
│  Key (hex):                                                     │
│  ┌────────────────────────────────────────────┐ [Gen Key]       │
│  │ a37f12c8...e5b9 (64자)                     │                 │
│  └────────────────────────────────────────────┘                 │
│  IV / Nonce (hex):                                              │
│  ┌────────────────────────────────────────────┐ [Gen IV]        │
│  │ 8c3a7d...f1 (24자)                         │                 │
│  └────────────────────────────────────────────┘                 │
│                                                                 │
│  ┌─ Plaintext ──────┐            ┌─ Ciphertext (Base64) ─────┐ │
│  │ secret            │ [Encrypt>>]│ AxK9f2nR...Qw==           │ │
│  │                   │ [<<Decrypt]│                            │ │
│  └───────────────────┘            └───────────────────────────┘ │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
  상태바: Encrypted successfully
```

---

### 시나리오 2 전체 흐름 요약

```
[Encrypt >> 클릭]
     │
     ▼
iced: Msg::Encrypt 생성
     │
     ▼
.map(Message::Symmetric) 래핑
     │  Message::Symmetric(Msg::Encrypt)
     ▼
App::update()
     │  Message::Symmetric(msg) → symmetric_tab::update() 위임
     ▼
symmetric_tab::update()
     │  1. hex::decode(key_hex) → key: Vec<u8> 32바이트
     │  2. hex::decode(iv_hex)  → iv: Vec<u8> 12바이트
     ▼
crypto::symmetric::encrypt(Aes256, Gcm, &key, &iv, b"secret")   ← UI→crypto 경계
     │
     ▼
encrypt_gcm()
     │  1. GcmNonce::from_slice(nonce)    ← nonce 타입 변환
     │  2. Aes256Gcm::new_from_slice(key) ← 암호 인스턴스 생성
     │  3. cipher.encrypt(nonce, plaintext) ← AEAD 암호화
     ▼
Ok(ciphertext_with_tag): Vec<u8>   (평문 + 16바이트 인증태그)
     │
     ▼ (crypto → UI 복귀)
base64::STANDARD.encode(&ct)
     │  바이너리 → "AxK9f2nR...Qw=="
     ▼
state.ciphertext = "AxK9f2nR...Qw=="  (상태 변경)
state.status = "Encrypted successfully"
     │
     ▼
iced: view() 재호출 → Ciphertext 필드에 결과 표시
```

---

## 비동기 흐름이 추가되는 경우 (참고)

Hash와 Symmetric은 동기 처리(즉시 완료)이지만,
**RSA 키 생성**과 **TLS 연결**은 시간이 걸리므로 비동기 처리합니다.

비동기 흐름은 `Task::perform()`을 사용합니다:

```
[Generate RSA-4096 클릭]
     │
     ▼
Msg::Generate
     │
     ▼
asymmetric_tab::update()
     │  state.generating = true   (로딩 표시용)
     │
     ▼ Task::perform() 반환
App::update()
     │  task.map(Message::Asymmetric) → iced에 Task 등록
     ▼
iced 런타임: 백그라운드 스레드에서 RSA 키 생성 실행
     │  (UI는 "Generating..." 표시하며 반응 유지)
     │  ... 수 초 소요 ...
     ▼
완료 → Msg::Generated(Ok(keypair)) 발생
     │
     ▼
asymmetric_tab::update()
     │  state.public_key_pem = keypair.public
     │  state.private_key_pem = keypair.private
     │  state.generating = false
     ▼
view() 재호출 → PEM 키가 화면에 표시
```

핵심 차이: `Task::none()` 대신 `Task::perform(future, callback)`을 반환하면,
iced가 future를 비동기로 실행하고 완료 시 callback 메시지를 발생시킵니다.

---

## 계층별 역할 정리

| 계층 | 역할 | 예시 |
|---|---|---|
| **main.rs** | iced 앱 설정, 진입점 | 윈도우 크기, 제목, 테마 |
| **app.rs** | 상태 소유, 메시지 라우팅 | `Message::Hash(msg)` → `hash_tab::update()` |
| **ui/*_tab.rs** | 입력 검증, 데이터 변환, UI 렌더링 | hex 디코딩, Base64 인코딩, 위젯 구성 |
| **crypto/*.rs** | 순수 암호 연산 | `&[u8]` → `Result<Vec<u8>>` |
| **theme.rs** | 색상 상수 | `ACCENT`, `ERROR`, `SIDEBAR_BG` |

**데이터 흐름 방향**:
```
사용자 입력 → iced 런타임 → App::update() → tab::update() → crypto 함수
                                                                  │
사용자 눈   ← iced 렌더링 ← App::view()  ← tab::view()  ← 상태 변경 ←┘
```
