# Entropy Forge: Teknik Özellikler Listesi (Feature List)

Bu döküman, mevcut Rust codebase (`entropy-forge` kütüphanesi) temel alınarak programın yapabildiği gerçek işlemleri ve hesapladığı metrikleri listeler.

## 1. Kriptografik İşlemler (Use Tab)
*   **Algoritma:** XOR tabanlı Stream Cipher (Akış Şifreleme).
*   **İşlem:** Düz metni (Plaintext), entropi kaynağından gelen anahtar akışı (Keystream) ile XOR'layarak şifreler veya şifreyi çözer.
*   **Görselleştirme:** Üretilen anahtar akışının (Keystream) durumunu gösteren 64 baytlık (8x8) Lattice Grid.

## 2. Kalite Analizi ve İstatistiksel Testler (Test Tab)
*   **Temel Metrikler (Quality Metrics):**
    *   **Shannon Entropy:** Verinin öngörülebilirliğini ölçer (maks 8.0 bit/bayt).
    *   **Min-Entropy:** En kötü durum entropi tahmini (Güvenlik garantisi için kritik).
    *   **Mean Byte Value:** Bayt değerlerinin ortalaması (İdeal: 127.5).
    *   **Chi-Square:** Bayt dağılımının tekdüzeliğini test eder.
    *   **Longest Run:** Birbirini takip eden aynı bitlerin maksimum uzunluğu.
*   **NIST SP 800-22 Test Bataryası:**
    *   **Frequency (Monobit) Test:** 0 ve 1 sayısının dengesini kontrol eder.
    *   **Runs Test:** 0 ve 1'ler arasındaki geçiş sıklığını kontrol eder.
    *   **Longest Run of Ones Test:** Bloklar içindeki en uzun '1' dizisini ölçer.
    *   **Chi-Square (Uniformity) Test:** Bayt dağılımını NIST standartlarında test eder.
    *   **Serial Test:** Ardışık bit paternlerinin frekansını kontrol eder.

## 3. Performans Ölçümü (Benchmark Tab)
*   **Throughput:** Entropi üretim hızı (MB/s).
*   **Latency:** Bayt başına gecikme süresi (µs/byte).
*   **İstatistik:** Toplam üretilen veri miktarı ve geçen süre.

## 4. Eğitim Laboratuvarı (Learn Tab)
*   **XOR Visualizer:** Karakter -> ASCII -> Binary dönüşümü ve bit-bit XOR işleminin adım adım gösterimi.
*   **Shannon Entropy Visualizer:** Bayt sayımı -> Olasılık hesabı -> Entropi katkısı -> Toplam entropi adımlarının matematiksel dökümü.
*   **NIST Frequency Visualizer:** Bit akışı -> 1/0 sayımı -> İstatistiksel S_obs hesabı -> P-Değeri (Erfc) hesabı ve Pass/Fail sonucu.