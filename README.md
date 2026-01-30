# Rust Coding Challenge: Async Provider Fetcher Library

## Objective

สร้าง **Rust Library Crate** สำหรับดึงข้อมูล จากผู้ให้บริการหลายเจ้า (Multi-providers) โดยเป้าหมายของโจทย์นี้คือการวัดทักษะความเชี่ยวชาญในด้าน **Library Design, Async Rust (Tokio), Type System (Generics & Lifetimes), และ Compile-time Configurations**

โจทย์นี้จำลองสถานการณ์การสร้าง Library ประสิทธิภาพสูงเพื่อนำไปใช้ต่อใน Internal Services อื่นๆ

## Functional Requirements

### 1. รองรับหลาย Provider (Feature Flags)
Library ต้องรองรับการดึงข้อมูลจาก 2 แหล่ง (Providers) โดยผู้ใช้งานจะเป็นคนเลือกผ่าน Cargo Features ในตอน Compile
- **Features:** `a_provider` และ `b_provider`
- **เงื่อนไขสำคัญ:** Library ต้อง **Compile ไม่ผ่าน (Fail to compile)** หาก:
    - ผู้ใช้ไม่เลือก Feature ใดเลย
    - ผู้ใช้เลือกทั้ง 2 Features พร้อมกัน (Mutual Exclusion)
- **API Logic:** คุณต้องจัดการ Logic ที่แตกต่างกันของแต่ละ Provider (เช่น URL หรือโครงสร้าง JSON)

### 2. ระบบ Monitoring ที่ยืดหยุ่น (Generics)
Library ต้องเปิดช่องให้ผู้ใช้งานสามารถ "เสียบ" (Inject) ระบบ Monitoring ของตัวเองเข้ามาได้ (เช่น เพื่อเก็บ Log หรือ Metrics) โดยต้องไม่มี Overhead ตอน Runtime (***Static Dispatch****)

### 3. การจัดการ Configuration (Lifetimes)
- สร้าง Struct `Config` สำหรับเก็บค่า config ต่างๆ (เช่น `base_url`)
- ต้องเก็บข้อมูลแบบ Reference เท่านั้น เพื่อลดการจอง Memory (Allocation) โดยไม่จำเป็น (ห้ามใช้ String)


### Error Handling
- สร้าง Custom Error Enum หรือใช้ crate นอกก็ได้
- ต้องแปลง Error จาก Library อื่นๆ (เช่น Network error, Parsing error) มาเป็น Error ของเราให้เหมาะสม

## Deliverables
1. **Library Source Code (`src/lib.rs`)**: โค้ดหลักของ Library ที่มีการ implement logic ทั้งหมด
2. **Unit Tests (`#[test]`)**:
    - ทดสอบการสร้าง Struct และ Logic เบื้องต้น
3. **Example Usage (`examples/demo.rs`)**:
    - สร้างไฟล์ตัวอย่างในโฟลเดอร์ `examples/` เพื่อแสดงวิธีการนำ Library ไปใช้งานจริง
    - print ผลลัพธ์จากการเรียกใช้ Library ออกมา
    - ต้องสามารถรันได้ด้วยคำสั่ง: `cargo run --example demo --features ...`

## Evaluation Criteria
เราจะรีวิวโค้ดของคุณโดยดูจากหัวข้อต่อไปนี้:
1. **ความถูกต้อง (Correctness):** โค้ดทำงานได้จริงหรือไม่? เงื่อนไข Feature Flags ทำงานถูกต้องและป้องกันการเลือกผิดหรือไม่?
2. **ความเป็น Rust (Rust Idioms):** การใช้ **Lifetimes**, **Generics**, และกฎ **Borrowing/Ownership** ถูกต้องและเหมาะสม ไม่มีการ Clone โดยไม่จำเป็น
3. **Async/Await:** เข้าใจการเขียน Async และจัดการ Trait bounds ได้ถูกต้องเมื่อทำงานกับ Tokio (Multithreaded Runtime)
4. **ประสบการณ์ผู้ใช้ (Developer Experience):** การออกแบบ API ใช้งานง่ายหรือไม่? 
5. **คุณภาพโค้ด (Code Quality):** การตั้งชื่อตัวแปร, การจัดระเบียบโค้ด (Modules), และความสะอาดของโค้ด

## Note
- **ไม่ต้องใช้ API จริง**: คุณสามารถใช้ URL จำลอง หรือ Mock API (mockito) ภายในได้เลย
- หาก API จริงยุ่งยากเกินไป สามารถจำลอง Response JSON ขึ้นมาเองได้ แต่ Flow การทำงานของ HTTP Request (Reqwest) และโครงสร้างโค้ดต้องถูกต้อง
- เน้นที่ **โครงสร้างและการออกแบบ (Architecture)** ของ Library เป็นสำคัญ
