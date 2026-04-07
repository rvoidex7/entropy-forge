# Entropy Forge: UI Bileşen Haritası (UI Mapping)

Hesaplanan özelliklerin kullanıcı arayüzündeki (UI) somut karşılıkları. Gereksiz hiçbir görsel unsur (arama, profil, log vb.) içermez.

## 1. Dashboard (Use)
*   **Girdiler:**
    *   `Plaintext Input`: Metin giriş alanı (Textarea).
    *   `Hex Output Toggle`: Çıktının hex formatında gösterilip gösterilmeyeceğini belirleyen seçim kutusu.
    *   `Initiate Forge Button`: Şifreleme işlemini başlatan ana buton.
*   **Çıktılar:**
    *   `Ciphertext Output`: İşlem sonucunu gösteren salt-okunur alan.
    *   `Lattice Visualizer`: 8x8 boyutunda, bayt değerlerine göre renklenen (gradiyent/monokrom) karelerden oluşan grid.
    *   `Help Link`: "Bu nasıl çalışır?" diyerek Learn sekmesine yönlendiren metin.

## 2. Analiz Paneli (Test)
*   **Kontroller:**
    *   `Sample Size Slider`: Test edilecek veri miktarını (1K - 1M bayt) belirleyen sürgü.
    *   `Run Tests Button`: Analiz sürecini başlatan buton.
*   **Göstergeler:**
    *   `Entropy Bars`: Shannon ve Min-Entropy için ilerleme çubukları (Progress bars) ve sayısal değerler.
    *   `Metric Labels`: Mean, Chi-Square ve Longest Run için basit metin etiketleri.
    *   `NIST Results Table`: 5 testin adını, P-Değerini ve Durumunu (✓ Pass / ✗ Fail) gösteren temiz tablo.

## 3. Performans Paneli (Benchmark)
*   **Kontroller:**
    *   `Data Volume Slider`: Benchmark yapılacak toplam veri boyutunu belirleyen sürgü.
    *   `Execute Benchmark Button`: Testi başlatan buton.
*   **Göstergeler:**
    *   `Throughput Display`: MB/s cinsinden büyük dijital değer.
    *   `Latency Display`: µs cinsinden değer.
    *   `Session Stats`: Toplam süre ve üretilen bayt miktarını gösteren ufak etiketler.

## 4. Eğitim Laboratuvarı (Learn)
*   **Genel Kontroller:**
    *   `Lesson Selector`: XOR / Shannon / NIST arasında geçiş sağlayan buton grubu.
    *   `Playback Controls`: Geri, Oynat/Duraklat, İleri butonları.
    *   `Animation Speed Slider`: Adım geçiş hızını belirleyen sürgü.
*   **İçerik Alanları:**
    *   `Step Info Header`: "Step 3 of 8: XORing Bit 4" gibi açıklayıcı başlık.
    *   `Visualization Canvas`: Bitlerin, tabloların veya formüllerin dinamik olarak değiştiği merkez alan.
    *   `Progress Tracker`: Toplam işlemin neresinde olduğumuzu gösteren adım noktaları veya hex-badge dizisi.