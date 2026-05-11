# Understanding Publisher and Message Broker

## a. Berapa banyak data yang dikirim publisher ke message broker dalam satu kali run?

Dalam satu kali eksekusi (`cargo run`), program publisher mengirimkan **5 event** ke *message broker*. Masing-masing event berupa objek `UserCreatedEventMessage` yang memiliki dua field: `user_id` dan `user_name`. Kelima pesan tersebut adalah:

| user_id | user_name |
|---------|-----------|
| 1 | 2406423055-Amir |
| 2 | 2406423055-Budi |
| 3 | 2406423055-Cica |
| 4 | 2406423055-Dira |
| 5 | 2406423055-Emir |

Setiap pesan diserialkan menggunakan *Borsh serialization* sebelum dikirim ke queue bernama `user_created` pada RabbitMQ.

## b. URL `amqp://guest:guest@localhost:5672` sama dengan subscriber, apa artinya?

Penggunaan URL yang **sama persis** antara publisher dan subscriber menunjukkan bahwa keduanya terhubung ke **message broker yang sama**, yaitu instansi RabbitMQ yang berjalan di `localhost:5672`. Ini adalah inti dari arsitektur *event-driven* dimana publisher dan subscriber tidak berkomunikasi secara langsung satu sama lain, melainkan keduanya dihubungkan melalui perantara (*broker*) yang sama. Publisher cukup mengirim pesan ke broker dan broker yang bertanggung jawab meneruskannya ke subscriber yang mendaftarkan diri pada queue yang sesuai (`user_created`). Dengan demikian, publisher dan subscriber tetap *loosely coupled* (mereka tidak perlu saling mengenal).

# Screenshot of running RabbitMQ
![Running RabbitMQ](assets/images/RabbitMQ.png)

# Sending and Processing Event

### Screenshot of RabbitMQ Connection
![RabbitMQ Connection](assets/images/RabbitMQ%20connections.png)

Setelah subscriber dijalankan dengan `cargo run`, tampilan RabbitMQ Management di
`http://localhost:15672/#/connections` menunjukkan **1 koneksi aktif** dari subscriber
(IP `192.168.65.1` dengan port `28719`) menggunakan protokol AMQP 0-9-1 dengan
status **running**. Hal ini membuktikan bahwa subscriber telah berhasil terhubung ke
message broker dan siap menerima event dari queue `user_created`.

---

### Screenshot of Publisher & Subscriber Console
![Publisher Subscriber Console](assets/images/Publisher-Subscriber%20console.png)

Ketika `cargo run` dijalankan pada direktori publisher, publisher mengirimkan **5 event**
sekaligus ke message broker RabbitMQ pada queue `user_created`. Kelima event tersebut
berisi data `UserCreatedEventMessage` dengan `user_id` 1–5 dan `user_name`
`2406423055-Amir` hingga `2406423055-Emir`.

Event-event tersebut tidak langsung dikirim ke subscriber, melainkan terlebih dahulu
masuk ke antrian (*queue*) di RabbitMQ. Subscriber yang sudah berjalan dan mendengarkan
queue `user_created` kemudian mengambil dan memproses setiap event satu per satu,
mencetak output seperti:

## Monitoring Chart Based on Publisher

### Screenshot of RabbitMQ Message Rate Spikes
![RabbitMQ Spikes](assets/images/RabbitMQ%20spikes.png)

Pada grafik **Message rates** di RabbitMQ Management Overview, terlihat tiga buah
*spike* (lonjakan) yang muncul di sekitar waktu `14:28:38`, `14:29:05`, dan `14:29:18`.
Setiap spike tersebut berkorelasi langsung dengan satu kali eksekusi `cargo run` pada
direktori publisher.

Setiap kali publisher dijalankan, ia langsung mempublikasikan 5 event sekaligus ke
queue `user_created` dalam waktu yang sangat singkat. Hal ini menyebabkan lonjakan
tajam pada *message rate* (diukur dalam pesan per detik, /s), yang kemudian langsung
turun kembali ke `0.0/s` karena publisher selesai berjalan dan tidak ada pengiriman
pesan lebih lanjut.

Spike tertinggi mencapai sekitar **2.0/s**, yang berarti pada saat itu broker menerima
dan meneruskan pesan dengan laju 2 pesan per detik. Pola naik-turun yang tajam ini
adalah karakteristik khas dari publisher yang bersifat *burst* — mengirim banyak pesan
sekaligus lalu berhenti — berbeda dengan publisher yang mengirim pesan secara
kontinu dan stabil. Ini membuktikan bahwa setiap spike pada grafik adalah jejak
langsung dari satu kali pemanggilan `cargo run` pada publisher.

---

# Bonus: Running on Cloud

Untuk menyelesaikan tugas bonus, eksperimen di atas diulang kembali dengan RabbitMQ yang berjalan di cloud (Railway), bukan di mesin lokal. Publisher dan subscriber tetap dijalankan dari komputer lokal, tetapi keduanya terhubung ke broker RabbitMQ yang di-deploy pada cloud melalui URL koneksi `amqp://` yang disediakan oleh Railway.

## Screenshot of RabbitMQ Web UI on Cloud

![Running 3 subscribers on Cloud (RabbitMQ Web UI)](assets/images/Running%203%20subscribers%20on%20Cloud%20(rabbitmq%20web%20ui).png)

Pada tampilan RabbitMQ Management yang berjalan di cloud (Railway), terlihat bahwa broker berhasil diakses melalui URL `rabbitmq-web-ui-production-b12e.up.railway.app`. Global counts menunjukkan `Connections: 3`, `Exchanges: 11`, dan `Queues: 2`, yang
mengkonfirmasi bahwa ketiga subscriber berhasil terhubung ke broker cloud secara bersamaan. Node yang aktif adalah `rabbit@rabbitmq` yang berjalan di environment cloud Railway.

## Screenshot of Terminal Running 3 Subscribers + 1 Publisher on Cloud

![Running 3 subscribers on Cloud (Terminal)](assets/images/Running%203%20subscribers%20on%20Cloud%20(terminal).png)

Dari terminal, terlihat bahwa 1 publisher dan 3 subscribers dijalankan secara bersamaan, semuanya terhubung ke broker RabbitMQ cloud. Publisher mengirimkan 5 event (`Amir`, `Budi`, `Cica`, `Dira`, `Emir`) ke queue, dan ketiga subscriber menerima pesan secara terdistribusi (seimbang) menggunakan algoritma round-robin:

- **Subscriber 1** menerima: `Amir` (user_id: 1) dan `Dira` (user_id: 4)
- **Subscriber 2** menerima: `Budi` (user_id: 2) dan `Emir` (user_id: 5)
- **Subscriber 3** menerima: `Cica` (user_id: 3)

Hasil ini menunjukkan bahwa mekanisme horizontal scaling dan round-robin distribution pada RabbitMQ bekerja sama persis baik di lingkungan lokal maupun di cloud. Perbedaan utamanya hanyalah pada latensi jaringan yang sedikit lebih tinggi karena koneksi melewati internet, namun semua fungsionalitas event-driven architecture tetap berjalan dengan baik.